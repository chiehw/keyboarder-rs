use std::cell::RefCell;

use anyhow::Context;
use xkbcommon::xkb;

use super::XConnection;
use xkb::compose::Status as ComposeStatus;

const XCB_KEY_PRESS: u8 = 2;
const XCB_KEY_RELEASE: u8 = 3;

pub struct XSimulator {
    conn: XConnection,
    device_id: u8,
    pressed_key: Vec<u32>,
    screen_num: i32,
    root: xcb::x::Window,
}

impl XSimulator {
    pub fn new(conn: XConnection) -> Self {
        let screen_num: i32 = conn.screen_num;
        let root = conn.root;
        let device_id = conn.keyboard.device_id();
        Self {
            conn,
            pressed_key: vec![],
            screen_num,
            root,
            device_id,
        }
    }

    pub fn simulate_keycode(&self, keycode: u8, pressed: bool) {
        if let Err(err) = self.process_key_event_impl(keycode, pressed) {
            log::error!("{err:#}")
        };
    }

    pub fn simualte_keysym(&self, keysym: u32, pressed: bool){

    }

    fn process_key_event_impl(&self, keycode: u8, pressed: bool) -> anyhow::Result<()> {
        self.send_native(keycode, pressed)?;
        Ok(())
    }

    fn send_native(&self, keycode: u8, press: bool) -> anyhow::Result<()> {
        let r#type = match press {
            true => XCB_KEY_PRESS,
            false => XCB_KEY_RELEASE,
        };
        self.conn.send_request_no_reply_log(&xcb::xtest::FakeInput {
            r#type,
            detail: keycode,
            time: 0,
            root: self.root,
            root_x: 0,
            root_y: 0,
            deviceid: self.device_id,
        });
        self.conn.flush().context("flushing pending requests")?;

        anyhow::Ok(())
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeadKeyStatus {
    /// Not in a dead key processing hold
    None,
    /// Holding until composition is done; the string is the uncommitted
    /// composition text to show as a placeholder
    Composing(String),
}
