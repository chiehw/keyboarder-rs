use crate::event::{DeadKeyStatus, KeyCode, KeyEvent, KeyboardEvent};

use super::{PhysKeyCode, XConnection};
use anyhow::{ensure, Context, Ok};
use std::{
    cell::RefCell,
    collections::HashSet,
    rc::{Rc, Weak},
};
use xkb::compose::Status as ComposeStatus;
use xkbcommon::xkb;

const XCB_KEY_PRESS: u8 = 2;
const XCB_KEY_RELEASE: u8 = 3;

pub struct XSimulator {
    conn: Weak<XConnection>,
    device_id: u8,
    pressed_key: HashSet<u8>,
    dead_key_status: DeadKeyStatus,
    root: xcb::x::Window,
}

impl XSimulator {
    pub fn new(conn: &Rc<XConnection>) -> Self {
        let root = conn.root;
        let device_id = conn.keyboard.device_id();

        Self {
            conn: Rc::downgrade(conn),
            pressed_key: HashSet::new(),
            root,
            device_id,
            dead_key_status: DeadKeyStatus::None,
        }
    }

    pub fn simulate_keycode(&mut self, keycode: u32, pressed: bool) {
        if let Err(err) = self.process_keycode_event_impl(keycode, pressed) {
            log::error!("{err:#}")
        };
    }

    pub fn simulate_keysym(&mut self, keysym: u32, pressed: bool) {
        let keyboard = &self.conn().keyboard;
        if let Some(keycode) = keyboard.get_keycode_by_keysym(keysym) {
            self.simulate_keycode(keycode, pressed);
        } else {
            log::error!(
                "No keysym {:?} in {:?}",
                keysym,
                keyboard.get_active_layout_name()
            );
        };
    }

    pub fn simulate_phys(&mut self, phys: PhysKeyCode, pressed: bool) {
        let keyboard = &self.conn().keyboard;
        if let Some(keycode) = keyboard.get_keycode_by_phys(phys) {
            self.simulate_keycode(keycode, pressed);
        } else {
            log::error!(
                "No PhysKeyCode {:?} in {:?}",
                phys,
                keyboard.get_active_layout_name()
            );
        };
    }

    pub fn simulate_keyboard_event(&mut self, keyboard_event: &KeyboardEvent) {
        if let Err(err) = self.process_keyboard_event_impl(keyboard_event) {
            log::error!("{err:#}")
        };
    }

    fn process_keyboard_event_impl(
        &mut self,
        keyboard_event: &KeyboardEvent,
    ) -> anyhow::Result<()> {
        let keyboard = &self.conn().keyboard;

        let current_modifiers = keyboard.get_current_modifiers();
        let key_event_vec = current_modifiers.diff_modifiers(&keyboard_event.key_event.modifiers);
        println!("{:?}", &key_event_vec);

        self.prepare_pressed_keys(&key_event_vec)?;
        let key_event = &keyboard_event.key_event;

        match key_event.key {
            KeyCode::RawCode(keycode) => self.simulate_keycode(keycode, key_event.press),
            KeyCode::Physical(phys) => self.simulate_phys(phys, key_event.press),
            _ => {}
        }

        Ok(())
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

    fn process_keycode_event_impl(&mut self, keycode: u32, pressed: bool) -> anyhow::Result<()> {
        ensure!(
            (8..=255).contains(&keycode),
            "Unexpected keycode, keycode should in (8, 255)"
        );
        let keycode: u8 = keycode.try_into()?;

        match pressed {
            true => self.pressed_key.insert(keycode),
            false => self.pressed_key.remove(&keycode),
        };
        self.send_native(keycode, pressed)?;
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
}

impl Drop for XSimulator {
    fn drop(&mut self) {
        if !self.pressed_key.is_empty() {
            log::debug!("Auto release key: {:?}", self.pressed_key);
            for keycode in &self.pressed_key {
                if let Err(err) = self.send_native(*keycode, false) {
                    log::error!("{err:#}")
                };
            }
        }
    }
}

struct Compose {
    state: xkb::compose::State,
    composition: String,
}

impl Compose {
    fn reset(&mut self) {
        self.composition.clear();
        self.state.reset();
    }

    fn feed(&mut self, _xcode: xkb::Keycode, xsym: xkb::Keysym, _keystate: &RefCell<xkb::State>) {
        if matches!(
            self.state.status(),
            ComposeStatus::Nothing | ComposeStatus::Cancelled | ComposeStatus::Composed
        ) {
            self.composition.clear();
        }

        let _previously_composing = !self.composition.is_empty();
        self.state.feed(xsym);
    }
}
