use crate::types::Modifiers;

use super::listen_keyboard;

pub struct WListener {
    pressed_modifiers: Modifiers,
}

impl WListener {
    pub fn new() -> WListener {
        Self::default()
    }

    pub fn run(&self) -> anyhow::Result<()> {
        listen_keyboard()
    }
}

impl Default for WListener {
    fn default() -> Self {
        let pressed_modifiers = Modifiers::NONE;
        Self { pressed_modifiers }
    }
}
