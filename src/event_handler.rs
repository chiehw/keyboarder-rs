use crate::action::Action;
use crate::common::*;
use crate::event::{Event, Event::*, KeyEvent};
use crate::x11::Key;
use crate::x11::*;

pub struct EventHandler {
    // keyboard: Keyboard,
    pressed_key: Vec<Key>,
    actions: Vec<Action>,
}

impl EventHandler {
    pub fn new() -> Result<Self> {
        // let keyboard = Keyboard::new()?;
        Ok(Self {
            // keyboard,
            pressed_key: vec![],
            actions: vec![],
        })
    }
}

impl Drop for EventHandler {
    fn drop(&mut self) {
        self.update_modifiers(&[]);
    }
}

impl EventHandler {
    /// Use the prepared keycode and keysym mapping.
    pub fn on_event(&mut self, event: &Event) -> Result<Vec<Action>> {
        match event {
            KeyEvent(keyboard_event) => {
                self.on_keyboard_event(keyboard_event);
            }
            OverrideTimeout => {}
        }
        Ok(self.actions.drain(..).collect())
    }

    fn on_keyboard_event(&mut self, keyboard_event: &KeyEvent) {}

    pub fn update_modifiers(&mut self, modifiers: &[Key]) {
        // let modifier_state = self.keyboard.get_modifier_state();
    }

    fn send_action(&mut self, action: Action) {
        self.actions.push(action);
    }
}
