use crate::platform_impl::platform::common::listen_keyboard;
use crate::types::Modifiers;

pub struct WinListener {
    pressed_modifiers: Modifiers,
}

impl WinListener {
    pub fn new() -> WinListener {
        Self::default()
    }

    pub fn run(&self) -> anyhow::Result<()> {
        listen_keyboard()
    }
}

impl Default for WinListener {
    fn default() -> Self {
        let pressed_modifiers = Modifiers::NONE;
        Self { pressed_modifiers }
    }
}
