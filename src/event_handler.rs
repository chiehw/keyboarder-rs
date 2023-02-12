use crate::action::Action;
use crate::event::{Event, Event::*, KeyboardEvent};
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
            KeyboardEvent(keyboard_event) => {
                self.on_keyboard_event(keyboard_event);
            }
            OverrideTimeout => {}
        }
        Ok(self.actions.drain(..).collect())
    }

    fn on_keyboard_event(&mut self, keyboard_event: &KeyboardEvent) {
        self.update_modifiers(&keyboard_event.modifiers);
        self.send_action(Action::Simulate(keyboard_event.code_state.clone()));
    }

    pub fn update_modifiers(&mut self, modifiers: &[Key]) {
        // let modifier_state = self.keyboard.get_modifier_state();
        let modifier_state = ModifierState::new(16);
        let codes = modifier_state.diff_modifiers(modifiers);
        for code_state in codes {
            self.send_action(Action::Simulate(code_state.clone()));
        }
    }

    fn send_action(&mut self, action: Action) {
        self.actions.push(action);
    }
}
