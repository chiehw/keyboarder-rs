use bitflags::*;
use serde::{Deserialize, Serialize};

pub type Keycode = u32;
pub use anyhow::{anyhow, ensure, Result};
pub use log;
pub use std::os::raw::c_int;

pub const TRUE: c_int = 1;
pub const FALSE: c_int = 0;

pub type KeySym = u32;

/// These keycodes identify keys based on their physical
/// position on an ANSI-standard US keyboard.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash, Copy, Ord, PartialOrd)]
pub enum PhysKeyCode {
    AltLeft,
    AltRight,
    ControlLeft,
    ControlRight,
    Backspace,
    CapsLock,
    Delete,
    DownArrow,
    End,
    Escape,
    F1,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F2,
    F20,
    Home,
    LeftArrow,
    MetaLeft,
    MetaRight,
    PageDown,
    PageUp,
    Return,
    ShiftLeft,
    ShiftRight,
    Space,
    Tab,
    UpArrow,
    PrintScreen,
    ScrollLock,
    Pause,
    NumLock,
    BackQuote, // Grave
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    Num0,
    Minus,
    Equal,
    KeyQ,
    KeyW,
    KeyE,
    KeyR,
    KeyT,
    KeyY,
    KeyU,
    KeyI,
    KeyO,
    KeyP,
    LeftBracket,
    RightBracket,
    KeyA,
    KeyS,
    KeyD,
    KeyF,
    KeyG,
    KeyH,
    KeyJ,
    KeyK,
    KeyL,
    SemiColon,
    Quote,
    BackSlash,
    // IntlBackslash,
    KeyZ,
    KeyX,
    KeyC,
    KeyV,
    KeyB,
    KeyN,
    KeyM,
    Comma,
    Dot,
    Slash,
    Insert,
    KpReturn,
    KpMinus,
    KpPlus,
    KpMultiply,
    KpDivide,
    KpDecimal,
    KpEnter,
    Kp0,
    Kp1,
    Kp2,
    Kp3,
    Kp4,
    Kp5,
    Kp6,
    Kp7,
    Kp8,
    Kp9,
    Function,
    Help,
    RightArrow,
    KpDelete,
    VolumeDown,
    VolumeUp,
    VolumeMute,
}

bitflags! {
    /// https://stackoverflow.com/questions/69656145/how-does-modifiersas-in-xmodmap-work-under-linux-operating-system
    /// Use xmodmap -pm to get meaning of modifier
    #[derive(Default, Deserialize, Serialize)]
    pub struct Modifiers: u16 {
        const NONE = 0;

        const SHIFT = 1<<1;
        const ALT = 1<<2;
        const CTRL = 1<<3;
        const META = 1<<4;

        const LEFT_ALT = 1<<5;
        const RIGHT_ALT = 1<<6;
        const LEFT_CTRL = 1<<7;
        const RIGHT_CTRL = 1<<8;
        const LEFT_SHIFT = 1<<9;
        const RIGHT_SHIFT = 1<<10;

        const CAPS = 1<<11;
        const NUM = 1<<12;

        const ALT_GR = 1<<13;
    }
}

impl TryFrom<String> for Modifiers {
    type Error = String;

    fn try_from(s: String) -> Result<Modifiers, String> {
        let mut mods = Modifiers::NONE;

        for ele in s.split('|') {
            // Allow for whitespace; debug printing Modifiers includes spaces
            // around the `|` so it is desirable to be able to reverse that
            // encoding here.
            let element = ele.trim();
            if element == "SHIFT" {
                mods |= Modifiers::SHIFT;
            } else if element == "ALT" {
                mods |= Modifiers::ALT;
            } else if ele == "CTRL" {
                mods |= Modifiers::CTRL;
            } else if ele == "Meta" {
                mods |= Modifiers::META;
            } else if ele == "NONE" || ele == "" {
                mods |= Modifiers::NONE;
            } else {
                return Err(format!("invalid modifier name {} in {}", ele, s));
            }
        }
        Ok(mods)
    }
}

impl From<Modifiers> for String {
    fn from(modifiers: Modifiers) -> String {
        modifiers.to_string()
    }
}

impl ToString for Modifiers {
    fn to_string(&self) -> String {
        let mut s = String::new();
        if *self == Self::NONE {
            s.push_str("NONE");
        }

        for (value, label) in [
            (Self::SHIFT, "SHIFT"),
            (Self::ALT, "ALT"),
            (Self::CTRL, "CTRL"),
            (Self::META, "META"),
            (Self::LEFT_ALT, "LEFT_ALT"),
            (Self::RIGHT_ALT, "RIGHT_ALT"),
            (Self::LEFT_CTRL, "LEFT_CTRL"),
            (Self::RIGHT_CTRL, "RIGHT_CTRL"),
            (Self::LEFT_SHIFT, "LEFT_SHIFT"),
            (Self::RIGHT_SHIFT, "RIGHT_SHIFT"),
            (Self::CAPS, "CAPS"),
            (Self::NUM, "NUM"),
            (Self::ALT_GR, "ALT_GR"),
        ] {
            if !self.contains(value) {
                continue;
            }
            if !s.is_empty() {
                s.push('|');
            }
            s.push_str(label);
        }

        s
    }
}

impl Modifiers {
    /// Remove positional and other "supplemental" bits that
    /// are used to carry around implementation details, but that
    /// are not bits that should be matched when matching key
    /// assignments.
    pub fn remove_positional_mods(self) -> Self {
        self - (Self::LEFT_ALT
            | Self::RIGHT_ALT
            | Self::LEFT_CTRL
            | Self::RIGHT_CTRL
            | Self::LEFT_SHIFT
            | Self::RIGHT_SHIFT)
    }

    /// todo!: return extra_modifiers, missing_modifiers
    ///
    ///  Get the codes that should be clicked,
    /// modifiers of both side can be sync after clicking the keys.
    ///
    /// The modifers in the vec represent the active state of the remote modifier,
    /// compare it with the local modifiers.
    pub fn diff_modifiers(&self, modifiers: &Modifiers) -> Vec<KeyEvent> {
        let mut key_event_vec: Vec<KeyEvent> = vec![];
        let target_modifiers = modifiers.remove_positional_mods();

        for pair in &[
            (Modifiers::SHIFT, PhysKeyCode::ShiftLeft),
            (Modifiers::CTRL, PhysKeyCode::ControlLeft),
            (Modifiers::ALT, PhysKeyCode::AltLeft),
            (Modifiers::META, PhysKeyCode::MetaLeft),
            (Modifiers::CAPS, PhysKeyCode::CapsLock),
            (Modifiers::NUM, PhysKeyCode::NumLock),
            (Modifiers::ALT_GR, PhysKeyCode::AltRight),
        ] {
            let (modifier, phys) = pair.to_owned();

            let pressed = target_modifiers.contains(modifier);
            if modifier == Modifiers::CAPS || modifier == Modifiers::NUM {
                if pressed && !self.contains(modifier) || !pressed && self.contains(modifier) {
                    key_event_vec.push(KeyEvent::with_phys(phys, true));
                    key_event_vec.push(KeyEvent::with_phys(phys, false));
                }
                continue;
            }
            if !pressed && self.contains(modifier) {
                key_event_vec.push(KeyEvent::with_phys(phys, false))
            }
            if pressed && !self.contains(modifier) {
                key_event_vec.push(KeyEvent::with_phys(phys, true))
            }
        }

        key_event_vec
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeyEvent {
    /// Which key was pressed
    pub key: KeyCode,
    // pressed or release
    pub press: bool,
    /// Which modifiers are down
    pub modifiers: Modifiers,
    pub click: bool,
}

impl Default for KeyEvent {
    fn default() -> KeyEvent {
        Self {
            key: KeyCode::RawCode(0),
            press: false,
            modifiers: Modifiers::NONE,
            click: false,
        }
    }
}

impl KeyEvent {
    pub fn with_phys(key: PhysKeyCode, press: bool) -> Self {
        Self {
            key: KeyCode::Physical(key),
            press,
            modifiers: Modifiers::NONE,
            click: false,
        }
    }

    pub fn with_keycode(key: KeyCode, press: bool) -> Self {
        Self {
            key,
            press,
            modifiers: Modifiers::NONE,
            click: false,
        }
    }

    /// if SHIFT is held and we have KeyCode::Char('c') we want to normalize
    /// that keycode to KeyCode::Char('C'); that is what this function does.
    pub fn normalize_shift(mut self) -> Self {
        let (key, modifiers) = normalize_shift(self.key, self.modifiers);
        self.key = key;
        self.modifiers = modifiers;

        self
    }

    /// If CTRL is held down and we have KeyCode::Char(_) with the
    /// ASCII control value encoded, decode it back to the ASCII
    /// alpha keycode instead.
    pub fn normalize_ctrl(mut self) -> Self {
        let (key, modifiers) = normalize_ctrl(self.key, self.modifiers);
        self.key = key;
        self.modifiers = modifiers;

        self
    }
}

pub fn get_key_event_from_u8_vec(buff: &[u8]) -> anyhow::Result<KeyEvent> {
    let key_event: KeyEvent = bincode::deserialize(buff)?;
    Ok(key_event)
}

pub fn get_u8_vec_from_key_event(key_event: &KeyEvent) -> anyhow::Result<Vec<u8>> {
    let buff: Vec<u8> = bincode::serialize(key_event)?;
    Ok(buff)
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
    KeySym(u32),
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

pub fn is_ascii_control(c: char) -> Option<char> {
    let c = c as u32;
    if c < 0x20 {
        let de_ctrl = ((c as u8) | 0x40) as char;
        Some(de_ctrl.to_ascii_lowercase())
    } else {
        None
    }
}

fn normalize_shift(key: KeyCode, modifiers: Modifiers) -> (KeyCode, Modifiers) {
    if modifiers.contains(Modifiers::SHIFT) {
        match key {
            KeyCode::Char(c) if c.is_ascii_uppercase() => (key, modifiers - Modifiers::SHIFT),
            KeyCode::Char(c) if c.is_ascii_lowercase() => (
                KeyCode::Char(c.to_ascii_uppercase()),
                modifiers - Modifiers::SHIFT,
            ),
            _ => (key, modifiers),
        }
    } else {
        (key, modifiers)
    }
}

fn normalize_ctrl(key: KeyCode, modifiers: Modifiers) -> (KeyCode, Modifiers) {
    if modifiers.contains(Modifiers::CTRL) {
        if let KeyCode::Char(c) = key {
            if (c as u32) < 0x20 {
                let de_ctrl = ((c as u8) | 0x40) as char;
                return (KeyCode::Char(de_ctrl.to_ascii_lowercase()), modifiers);
            }
        }
    }
    (key, modifiers)
}
