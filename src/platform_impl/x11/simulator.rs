use super::connection::XConnection;

use crate::simulate::Simulate;
use crate::types::{DeadKeyStatus, KeyCode, KeyEvent, SimulateEvent};

use crate::types::PhysKeyCode;
use anyhow::{ensure, Context, Ok};

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
    dead_key_status: DeadKeyStatus,
    root: xcb::x::Window,
}

impl Simulate for XSimulator {
    fn spawn_server() -> anyhow::Result<JoinHandle<()>> {
        let pipe = Pipe::new()?;

        let mut write_fd = pipe.write;
        let mut read_fd = pipe.read;

        write_fd.set_non_blocking(true)?;
        read_fd.set_non_blocking(true)?;

        SENDER.lock().unwrap().replace(write_fd);

        Ok({
            std::thread::spawn(move || {
                let conn = XConnection::with_simulator()
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

    fn simulate_event(&mut self, sim_event: crate::types::SimulateEvent) {
        match sim_event {
            SimulateEvent::KeyCodeEvent(keycode_event) => {
                let press = keycode_event.press;
                match keycode_event.keycode {
                    KeyCode::RawCode(keycode) => {
                        self.simulate_keycode(keycode, press);
                    }
                    KeyCode::Physical(phys) => {
                        self.simulate_phys(phys, press);
                    }
                    KeyCode::KeySym(keysym) => {
                        self.simulate_keysym(keysym, press);
                    }
                    _ => {}
                }
            }
            SimulateEvent::KeyEvent(key_event) => {
                self.simulate_key_event(&key_event);
            }
            SimulateEvent::CharNoModifi(chr) => {
                self.simulate_char_without_modifiers(chr);
            }
        }
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
            dead_key_status: DeadKeyStatus::None,
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

    fn process_key_event_impl(&mut self, key_event: &KeyEvent) -> anyhow::Result<()> {
        let keyboard = &self.conn().keyboard;

        let current_modifiers = keyboard.get_current_modifiers();
        let key_event_vec = current_modifiers.diff_modifiers(&key_event.modifiers);

        self.prepare_pressed_keys(&key_event_vec)?;

        match key_event.key {
            KeyCode::RawCode(keycode) => {
                if key_event.click {
                    self.simulate_keycode(keycode, true);
                    self.simulate_keycode(keycode, false);
                } else {
                    self.simulate_keycode(keycode, key_event.press)
                };
            }
            KeyCode::KeySym(keysym) => {
                if key_event.click {
                    self.simulate_keysym(keysym, true);
                    self.simulate_keysym(keysym, false);
                } else {
                    self.simulate_keysym(keysym, key_event.press)
                };
            }
            KeyCode::Physical(phys) => {
                if key_event.click {
                    self.simulate_phys(phys, true);
                    self.simulate_phys(phys, false);
                } else {
                    self.simulate_phys(phys, key_event.press)
                };
            }

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
