use crate::action::Action;
use crate::common::*;
use crate::event::{Event, Event::*, KeyboardEvent};
use crate::keyboard::Keyboard;
use rdev::Key;

pub struct EventHandler {
    keyboard: Keyboard,
    pressed_key: Vec<Key>,
    actions: Vec<Action>,
}

impl EventHandler {
    pub fn new() -> Result<Self> {
        let keyboard = Keyboard::new()?;
        Ok(Self {
            keyboard,
            pressed_key: vec![],
            actions: vec![],
        })
    }
}

impl Drop for EventHandler {
    fn drop(&mut self) {}
}

impl EventHandler {
    /// Use the prepared keycode and keysym mapping.
    pub fn on_event(&mut self, event: &Event) -> Result<Vec<Action>> {
        match event {
            KeyboardEvent(keyboard_event) => {
                self.on_keyboard_event(keyboard_event);
            }
            OverrideTimeout => {}
        }
        Ok(self.actions.drain(..).collect())
    }

    fn on_keyboard_event(&mut self, keyboard_event: &KeyboardEvent) {
        self.check_modifiers();
        self.send_action(Action::Sync(keyboard_event.code_state.clone()));
    }

    pub fn check_modifiers(&mut self) {
        self.keyboard.get_current_modifiers();
    }

    fn send_action(&mut self, action: Action) {
        self.actions.push(action);
    }
}
