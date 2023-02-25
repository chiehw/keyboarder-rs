use crate::platform_impl::platform::connection::WinConnection;
use crate::simulate::Simulate;
use crate::types::ServerMode;
use std::rc::Rc;

pub struct WinSimulator {
    pub mode: Option<ServerMode>,
}

impl WinSimulator {
    pub fn new(_conn: &Rc<WinConnection>) -> WinSimulator {
        Self { mode: None }
    }
}

impl Simulate for WinSimulator {
    fn spawn_server(mode: ServerMode) -> anyhow::Result<std::thread::JoinHandle<()>> {
        todo!()
    }

    fn event_to_server(_key_event: &crate::types::KeyEvent) -> anyhow::Result<()> {
        todo!()
    }

    fn simulate_keycode(&mut self, _keycode: u32, _press: bool) {
        todo!()
    }

    fn simulate_keysym(&mut self, _keysym: u32, _press: bool) {
        todo!()
    }

    fn simulate_char_without_modifiers(&mut self, _chr: char) {
        todo!()
    }

    fn simulate_phys(&mut self, _phys: crate::types::PhysKeyCode, _press: bool) {
        todo!()
    }

    fn simulate_key_event(&mut self, _key_event: &crate::types::KeyEvent) {
        todo!()
    }

    fn simulate_server(&mut self, key_event: &crate::types::KeyEvent) {
        todo!()
    }
}
