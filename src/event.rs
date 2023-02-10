use rdev::Key;

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
}

impl From<bool> for State {
    fn from(value: bool) -> Self {
        match value {
            true => State::Press,
            false => State::Release,
        }
    }
}
