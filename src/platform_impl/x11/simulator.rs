use super::connection::XConnection;
use super::keyboard::MOD_NAME_ISO_LEVEL3_SHIFT;

use crate::connection::ConnectionOps;
use crate::keysyms::{self, char_to_keysym};
use crate::simulate::{Simulate, SENDER};
use crate::types::{KeyCode, KeyEvent, Modifiers, ServerMode, SimEvent};

use crate::types::PhysKeyCode;
use anyhow::{ensure, Context};
use filedescriptor::Pipe;
use xkbcommon::xkb;

use std::borrow::Borrow;
use std::collections::HashMap;
use std::io::Write;
use std::thread::JoinHandle;
use std::{
    collections::HashSet,
    rc::{Rc, Weak},
};

const XCB_KEY_PRESS: u8 = 2;
const XCB_KEY_RELEASE: u8 = 3;

pub struct XSimulator {
    conn: Weak<XConnection>,
    device_id: u8,
    pressed_key: HashSet<u8>,
    root: xcb::x::Window,
    pub mode: Option<ServerMode>,
    pub rebinding_keysyms: HashMap<u32, u32>,
}

impl Simulate for XSimulator {
    fn spawn_server(mode: ServerMode) -> anyhow::Result<JoinHandle<()>> {
        let pipe = Pipe::new()?;

        let mut write_fd = pipe.write;
        let mut read_fd = pipe.read;

        write_fd.set_non_blocking(true)?;
        read_fd.set_non_blocking(true)?;

        SENDER.lock().unwrap().replace(write_fd);

        Ok({
            std::thread::spawn(move || match XConnection::with_simulator(mode) {
                Ok(conn) => {
                    if let Err(err) = conn.run_message_loop(&mut read_fd) {
                        log::error!("Failed to process message: {:?}", err);
                    };
                }
                Err(err) => {
                    log::error!(
                        "Failed to init Connection, Please check env Display: {:?}",
                        err
                    )
                }
            })
        })
    }

    fn event_to_server(event: &SimEvent) -> anyhow::Result<()> {
        let mut binding = SENDER.lock().unwrap();
        let sender = binding.as_mut();

        if let Some(sender) = sender {
            let buf: Vec<u8> = event.clone().try_into()?;
            let size = sender.write(&buf)?;
            if size != buf.len() {
                log::error!("Can't write key event");
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        }

        Ok(())
    }

    fn simulate_keysym(&mut self, keysym: u32, press: bool) {
        let keyboard = &self.conn().keyboard;
        if let Some(keycode) = keyboard.get_keycode_by_keysym(keysym) {
            log::debug!("simulate keysym {:?} -> {:?}", keysym, press);
            self.simulate_keycode(keycode, press);
        } else {
            log::error!(
                "No keysym {:?} in {:?}",
                keysym,
                keyboard.get_active_layout_name()
            );
        };
    }

    fn simulate_char_without_modifiers(&mut self, chr: char) {
        log::debug!("simulate char: {:?} ", chr);
        if let Err(err) = self.process_char_impl(chr) {
            log::error!("Failed to simulate {err:#}")
        };
    }

    fn simulate_phys(&mut self, phys: PhysKeyCode, press: bool) {
        let keyboard = &self.conn().keyboard;
        if let Some(keycode) = keyboard.get_keycode_by_phys(phys) {
            log::debug!("simulate phys {:?} => {:?}", phys, press);
            self.simulate_keycode(keycode, press);
        } else {
            log::error!(
                "No PhysKeyCode {:?} in {:?}",
                phys,
                keyboard.get_active_layout_name()
            );
        };
    }

    fn simulate_keycode(&mut self, keycode: u32, press: bool) {
        if let Err(err) = self.process_keycode_event_impl(keycode, press) {
            log::error!("{err:#}")
        };
    }

    fn simulate_key_event(&mut self, key_event: &KeyEvent) {
        if let Err(err) = self.process_key_event_impl(key_event) {
            log::error!("{err:#}")
        };
    }

    fn simulate_server(&mut self, key_event: &KeyEvent) {
        if let Err(err) = self.process_server_event_impl(key_event) {
            log::error!("{err:#}")
        };
    }

    fn release_modifiers(&mut self) -> anyhow::Result<()> {
        let cur_modifiers = self.get_current_modifiers();
        let target_modifiers = Modifiers::NONE;
        let key_event_vec = cur_modifiers.diff_modifiers(&target_modifiers);

        self.prepare_pressed_keys(&key_event_vec)?;

        Ok(())
    }
}

impl XSimulator {
    pub fn new(conn: &Rc<XConnection>) -> Self {
        let root = conn.root;
        let device_id = conn.keyboard.get_device_id();

        XSimulator {
            conn: Rc::downgrade(conn),
            pressed_key: HashSet::new(),
            root,
            device_id,
            mode: None,
            rebinding_keysyms: HashMap::new(),
        }
    }

    /// restore_flag is used to restore the keyboard state.
    fn prepare_pressed_keys(&mut self, key_event_vec: &Vec<KeyEvent>) -> anyhow::Result<()> {
        for key_event in key_event_vec {
            if let KeyCode::Physical(phys) = key_event.key {
                let press = key_event.press;
                match key_event.press {
                    true => self.simulate_phys(phys, press),
                    false => self.simulate_phys(phys, press),
                }
            }
        }
        Ok(())
    }

    fn rebinding_keycode(&self, keysym: u32) -> anyhow::Result<u32> {
        let conn = &self.conn();

        let mut unused = conn.keyboard.unused_keycodes.borrow_mut();
        if unused.len() > 1 {
            let keycode = unused.remove(0);
            conn.send_request_no_reply_log(&xcb::x::ChangeKeyboardMapping {
                keycode_count: 1,
                first_keycode: keycode as u8,
                keysyms_per_keycode: 1,
                keysyms: &[keysym],
            });
            conn.flush().context("flushing pending requests")?;
            Ok(keycode)
        } else {
            anyhow::bail!("Can't find unused keycode");
        }
    }

    fn process_char_impl(&mut self, chr: char) -> anyhow::Result<()> {
        let keysym = keysyms::char_to_keysym(chr);

        let conn = &self.conn();
        let keyboard = &conn.keyboard;
        let char_key_event = keyboard.get_key_event_by_keysym(keysym);

        if let Some(char_key_event) = char_key_event {
            let target_modifiers = char_key_event.modifiers;
            let cur_modifiers = self.get_current_modifiers();
            let key_event_vec = cur_modifiers.diff_modifiers(&target_modifiers);
            self.prepare_pressed_keys(&key_event_vec)?;

            if let KeyCode::RawCode(keycode) = char_key_event.key {
                self.simulate_keycode(keycode, true);
            }
            if let KeyCode::RawCode(keycode) = char_key_event.key {
                self.simulate_keycode(keycode, false);
            }
        } else if let Some(&keycode) = self.rebinding_keysyms.get(&keysym) {
            self.release_modifiers()?;
            self.simulate_keycode(keycode, true);
            self.simulate_keycode(keycode, false);
        } else if let Ok(keycode) = self.rebinding_keycode(keysym) {
            log::info!(
                "Remapping keycode={keycode} => keysym={:?} char={:?}')",
                keysym,
                chr
            );
            self.rebinding_keysyms.insert(keysym, keycode);

            self.release_modifiers()?;
            self.simulate_keycode(keycode, true);
            self.simulate_keycode(keycode, false);
        } else {
            anyhow::bail!("Failed to process char: char={chr}, keysym={keysym}")
        }

        Ok(())
    }

    /// https://stackoverflow.com/questions/69656145/how-does-modifiersas-in-xmodmap-work-under-linux-operating-system
    /// Use xmodmap -pm to get meaning of modifier
    ///
    /// simulate key will not send xkb event to update state. so we should get new state.
    pub fn get_current_modifiers(&self) -> Modifiers {
        let conn = self.conn();
        let kbd = &conn.keyboard;

        let keymap = &conn.keyboard.keymap.borrow();
        let device_id = kbd.get_device_id();
        let state = xkb::x11::state_new_from_device(keymap, &conn, device_id.into());

        let mut res = Modifiers::default();
        for (mod_name, modifier) in [
            (xkb::MOD_NAME_SHIFT, Modifiers::SHIFT),
            (xkb::MOD_NAME_CTRL, Modifiers::CTRL),
            (xkb::MOD_NAME_ALT, Modifiers::ALT),
            (xkb::MOD_NAME_LOGO, Modifiers::META),
            (xkb::MOD_NAME_CAPS, Modifiers::CAPS),
            (xkb::MOD_NAME_NUM, Modifiers::NUM),
            (MOD_NAME_ISO_LEVEL3_SHIFT, Modifiers::ALT_GR),
        ] {
            if state.mod_name_is_active(mod_name, xkb::STATE_MODS_EFFECTIVE) {
                res |= modifier;
            }
        }

        res
    }

    fn process_server_event_impl(&mut self, key_event: &KeyEvent) -> anyhow::Result<()> {
        let mode = if let Some(mode) = &self.mode {
            mode
        } else {
            anyhow::bail!("Can't find simulate mode");
        };
        let conn = self.conn();
        let press = key_event.press;

        match mode {
            ServerMode::Map => {
                if let Some(raw_event) = key_event.raw_event {
                    self.simulate_phys(raw_event.key, press)
                }
            }
            ServerMode::Translate => {
                let kbd = conn.keyboard.borrow();
                let char_keysym = kbd.char_keysym.borrow();

                let cur_modifiers = self.get_current_modifiers();
                let target_modifers = key_event.modifiers.trans_positional_mods();
                let key_event_vec = cur_modifiers.diff_modifiers(&target_modifers);

                match key_event.key {
                    KeyCode::Char(chr) => {
                        if !press {
                            return Ok(());
                        }
                        let keysym = char_to_keysym(chr);
                        if !cur_modifiers.is_shortcut() && !chr.is_control() {
                            // Fr:
                            // "!" => keycode=33, but shift + 1 is US
                            // exclude: delete(\u{8})
                            self.simulate_char_without_modifiers(chr);
                        } else if chr.is_control() {
                            // PhysKeyCode: \u{8} => Delete( chr is )
                            if let Some(&keysym) = char_keysym.get(&(chr as u32)) {
                                self.prepare_pressed_keys(&key_event_vec)?;

                                self.simulate_keysym(keysym, true);
                                self.simulate_keysym(keysym, false);
                            } else {
                                log::error!("Faile to process control char: {:?}", chr);
                            }
                        } else if kbd.keysym_keycode_map.borrow().contains_key(&keysym) {
                            // PhysKeyCode: q => KeyQ in US, q => keyA(Input char "a") in Fr
                            self.prepare_pressed_keys(&key_event_vec)?;

                            self.simulate_keysym(keysym, true);
                            self.simulate_keysym(keysym, false);
                        } else {
                            self.simulate_char_without_modifiers(chr);
                        }
                    }
                    KeyCode::Physical(phys) => self.simulate_phys(phys, press),
                    KeyCode::RawCode(_) => {
                        log::error!("Unexcept key event: {:?}", key_event);
                    }
                    _ => {}
                }
            }
            ServerMode::Auto => todo!(),
        }

        Ok(())
    }

    fn process_key_event_impl(&mut self, key_event: &KeyEvent) -> anyhow::Result<()> {
        match key_event.key {
            KeyCode::RawCode(keycode) => self.simulate_keycode(keycode, key_event.press),
            KeyCode::KeySym(keysym) => self.simulate_keysym(keysym, key_event.press),
            KeyCode::Physical(phys) => self.simulate_phys(phys, key_event.press),

            _ => {}
        }

        Ok(())
    }

    fn process_keycode_event_impl(&mut self, keycode: u32, press: bool) -> anyhow::Result<()> {
        ensure!(
            (8..=255).contains(&keycode),
            "Unexpected keycode, keycode should in (8, 255)"
        );
        let keycode: u8 = keycode.try_into()?;

        match press {
            true => self.pressed_key.insert(keycode),
            false => self.pressed_key.remove(&keycode),
        };
        self.send_native(keycode, press)?;
        Ok(())
    }

    fn send_native(&self, keycode: u8, press: bool) -> anyhow::Result<()> {
        let r#type = match press {
            true => XCB_KEY_PRESS,
            false => XCB_KEY_RELEASE,
        };
        let conn = self.conn();
        conn.send_request_no_reply_log(&xcb::xtest::FakeInput {
            r#type,
            detail: keycode,
            time: 0,
            root: self.root,
            root_x: 0,
            root_y: 0,
            deviceid: self.device_id,
        });
        conn.flush().context("flushing pending requests")?;
        log::trace!(
            "simulate keycode {:?}({:?}) -> {:?}",
            keycode,
            conn.keyboard.get_phys_by_keycode(keycode.into()),
            press
        );

        anyhow::Ok(())
    }

    fn conn(&self) -> Rc<XConnection> {
        self.conn.upgrade().expect("XConnection to be alive")
    }

    fn release_pressed_keys(&mut self) {
        if !self.pressed_key.is_empty() {
            log::debug!("Auto release key: {:?}", self.pressed_key);
            for keycode in &self.pressed_key {
                if let Err(err) = self.send_native(*keycode, false) {
                    log::error!("{err:#}")
                };
            }
            self.pressed_key.clear();
        }
    }
}

impl Drop for XSimulator {
    fn drop(&mut self) {
        self.release_pressed_keys();
        // FIXME
        // self.release_modifiers().ma;
    }
}
