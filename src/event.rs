use serde::{Deserialize, Serialize};

use crate::x11::{Key, Modifiers, PhysKeyCode};

/// todo?: Is repeat necessary?

#[derive(Debug, Clone)]
pub enum Event {
    KeyboardEvent(KeyboardEvent),
    OverrideTimeout,
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyEvent {
    /// Which key was pressed
    pub key: KeyCode,
    // pressed or release
    pub press: bool,
    /// Which modifiers are down
    pub modifiers: Modifiers,
}

impl KeyEvent {
    pub fn with_phys(key: PhysKeyCode, press: bool) -> Self {
        Self {
            key: KeyCode::Physical(key),
            press,
            modifiers: Modifiers::NONE,
        }
    }

    pub fn with_keycode(key: KeyCode, press: bool) -> Self {
        Self {
            key,
            press,
            modifiers: Modifiers::NONE,
        }
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

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize, Ord, PartialOrd)]
pub enum KeyCode {
    /// The decoded unicode character
    Char(char),
    Composed(String),
    RawCode(u32),
    Physical(PhysKeyCode),

    Hyper,
    Super,
    Meta,

    /// Ctrl-break on windows
    Cancel,
    // There is no `Backspace`; use `Char('\u{8}') instead
    // There is no `Tab`; use `Char('\t')` instead
    Clear,
    // There is no `Enter`; use `Char('\r')` instead
    Shift,
    // There is no `Escape`; use `Char('\u{1b}') instead
    LeftShift,
    RightShift,
    Control,
    LeftControl,
    RightControl,
    Alt,
    LeftAlt,
    RightAlt,
    Pause,
    CapsLock,
    VoidSymbol,
    PageUp,
    PageDown,
    End,
    Home,
    LeftArrow,
    RightArrow,
    UpArrow,
    DownArrow,
    Select,
    Print,
    Execute,
    PrintScreen,
    Insert,
    // There is no `Delete`; use `Char('\u{7f}')` instead
    Help,
    LeftWindows,
    RightWindows,
    Applications,
    Sleep,
    /// Numeric keypad digits 0-9
    Numpad(u8),
    Multiply,
    Add,
    Separator,
    Subtract,
    Decimal,
    Divide,
    /// F1-F24 are possible
    Function(u8),
    NumLock,
    ScrollLock,
    Copy,
    Cut,
    Paste,
    BrowserBack,
    BrowserForward,
    BrowserRefresh,
    BrowserStop,
    BrowserSearch,
    BrowserFavorites,
    BrowserHome,
    VolumeMute,
    VolumeDown,
    VolumeUp,
    MediaNextTrack,
    MediaPrevTrack,
    MediaStop,
    MediaPlayPause,
    ApplicationLeftArrow,
    ApplicationRightArrow,
    ApplicationUpArrow,
    ApplicationDownArrow,
}

impl KeyCode {
    /// Return true if the key represents a modifier key.
    pub fn is_modifier(&self) -> bool {
        matches!(
            self,
            Self::Hyper
                | Self::Super
                | Self::Meta
                | Self::Shift
                | Self::LeftShift
                | Self::RightShift
                | Self::Control
                | Self::LeftControl
                | Self::RightControl
                | Self::Alt
                | Self::LeftAlt
                | Self::RightAlt
                | Self::LeftWindows
                | Self::RightWindows
        )
    }
}

#[derive(Debug, Clone)]
pub struct KeyboardEvent {
    pub key_event: KeyEvent,
    pub pressed_keys: Vec<PhysKeyCode>,
}

impl KeyboardEvent {
    pub fn new(key_event: KeyEvent, pressed_keys: Vec<PhysKeyCode>) -> Self {
        Self {
            key_event,
            pressed_keys,
        }
    }
}
