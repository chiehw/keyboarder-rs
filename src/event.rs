use crate::x11::Key;

/// todo?: Is repeat necessary?

#[derive(Debug, Clone)]
pub enum Event {
    KeyboardEvent(KeyboardEvent),
    OverrideTimeout,
}

#[derive(Debug, Clone)]
pub struct KeyboardEvent {
    pub code_state: CodeState,
    pub modifiers: Vec<Key>,
}

impl KeyboardEvent {
    pub fn new(code: Code, state: State, modifiers: Vec<Key>) -> Self {
        Self {
            code_state: CodeState { code, state },
            modifiers,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CodeState {
    // // KeyCode or KeySym
    pub code: Code,
    pub state: State,
}

impl CodeState {
    pub fn new(code: Code, state: State) -> Self {
        Self { code, state }
    }

    pub fn with_key(key: Key, _press: bool) -> Self {
        Self {
            code: Code::KeyCode(key.into()),
            state: crate::event::State::Press,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Code {
    KeyCode(u32),
    KeySym(u32),
}

#[derive(Debug, Clone)]
pub enum State {
    Press,
    Release,
    Click,
}

impl From<bool> for State {
    fn from(value: bool) -> Self {
        match value {
            true => State::Press,
            false => State::Release,
        }
    }
}
