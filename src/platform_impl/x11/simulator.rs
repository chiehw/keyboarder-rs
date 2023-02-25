use super::connection::XConnection;
use super::keyboard::MOD_NAME_ISO_LEVEL3_SHIFT;

use crate::connection::ConnectionOps;
use crate::simulate::{Simulate, SENDER};
use crate::types::{KeyCode, KeyEvent, Modifiers, ServerMode};

use crate::types::PhysKeyCode;
use anyhow::{ensure, Context, Ok};
use filedescriptor::Pipe;
use xkbcommon::xkb;

use std::borrow::Borrow;
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
            std::thread::spawn(move || {
                let conn = XConnection::with_simulator(mode)
                    .context("Failed to init Connection")
                    .unwrap();

                if let Err(err) = conn.run_message_loop(&mut read_fd) {
                    log::error!("Failed to process message: {:?}", err);
                };
            })
        })
    }

    fn event_to_server(key_event: &KeyEvent) -> anyhow::Result<()> {
        let mut binding = SENDER.lock().unwrap();
        let sender = binding.as_mut();

        if let Some(sender) = sender {
            let buf = key_event.to_u8_vec()?;
            let size = sender.write(&buf)?;
            if size != buf.len() {
                log::error!("Can't write key event");
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        }

        Ok(())
    }

    fn simulate_keycode(&mut self, keycode: u32, press: bool) {
        if let Err(err) = self.process_keycode_event_impl(keycode, press) {
            log::error!("{err:#}")
        };
    }

    fn simulate_keysym(&mut self, keysym: u32, press: bool) {
        let keyboard = &self.conn().keyboard;
        if let Some(keycode) = keyboard.get_keycode_by_keysym(keysym) {
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
        if let Err(err) = self.process_char_impl(chr) {
            log::error!("{err:#}")
        };
    }

    fn simulate_phys(&mut self, phys: PhysKeyCode, press: bool) {
        let keyboard = &self.conn().keyboard;
        if let Some(keycode) = keyboard.get_keycode_by_phys(phys) {
            self.simulate_keycode(keycode, press);
        } else {
            log::error!(
                "No PhysKeyCode {:?} in {:?}",
                phys,
                keyboard.get_active_layout_name()
            );
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

    fn process_char_impl(&mut self, chr: char) -> anyhow::Result<()> {
        let keyboard = &self.conn().keyboard;

        let key_event = keyboard.get_key_event_by_char(chr);
        ensure!(key_event.is_some(), "Not found char `{:?}`", chr);
        self.process_key_event_impl(&key_event.unwrap())?;

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

        match mode {
            ServerMode::Map => {
                let press = key_event.press;
                if let Some(raw_event) = key_event.raw_event {
                    self.simulate_phys(raw_event.key, press)
                }
            }
            ServerMode::Translate => {
                let key_event_vec = self
                    .get_current_modifiers()
                    .diff_modifiers(&key_event.modifiers);
                if let Some(raw_event) = key_event.raw_event {
                    // Don't need to sync modifier when press modifiers
                    if !raw_event.key.is_modifier() {
                        self.prepare_pressed_keys(&key_event_vec)?;
                    }
                    self.simulate_key_event(key_event);
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
        log::debug!(
            "simualte keycode {:?}({:?}) -> {:?}",
            keycode,
            conn.keyboard.get_phys_by_keycode(keycode.into()).unwrap(),
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
    }
}
