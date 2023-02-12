use crate::action::Action;
use crate::common::*;
use crate::event::{Event, Event::*, KeyboardEvent};
use crate::keyboard::{Key, Keyboard};

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
        self.check_modifiers(&keyboard_event.modifiers);
        self.send_action(Action::Simulate(keyboard_event.code_state.clone()));
    }

    pub fn check_modifiers(&mut self, modifiers: &[Key]) {
        let modifier_state = self.keyboard.get_modifier_state();
        let codes = modifier_state.compare_modifers(modifiers);
        for code_state in codes {
            self.send_action(Action::Simulate(code_state.clone()));
        }
    }

    fn send_action(&mut self, action: Action) {
        self.actions.push(action);
    }
}
