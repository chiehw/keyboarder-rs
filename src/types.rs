use bitflags::*;
use serde::{Deserialize, Serialize};

pub type Keycode = u32;
pub type Scancode = u32;
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
    Apps,
}

impl PhysKeyCode {
    /// Return true if the key represents a modifier key.
    pub fn is_modifier(&self) -> bool {
        matches!(
            self,
            Self::ShiftLeft
                | Self::ShiftRight
                | Self::ControlLeft
                | Self::ControlRight
                | Self::MetaLeft
                | Self::MetaRight
                | Self::AltLeft
                | Self::AltRight
        )
    }
    pub fn to_key_code(self) -> KeyCode {
        match self {
            Self::ShiftLeft => KeyCode::LeftShift,
            Self::ControlLeft => KeyCode::LeftControl,
            Self::MetaLeft => KeyCode::LeftWindows,
            Self::AltLeft => KeyCode::LeftAlt,
            Self::ShiftRight => KeyCode::RightShift,
            Self::ControlRight => KeyCode::RightControl,
            Self::MetaRight => KeyCode::RightWindows,
            Self::AltRight => KeyCode::RightAlt,
            Self::LeftArrow => KeyCode::LeftArrow,
            Self::RightArrow => KeyCode::RightArrow,
            Self::UpArrow => KeyCode::UpArrow,
            Self::DownArrow => KeyCode::DownArrow,
            Self::CapsLock => KeyCode::CapsLock,
            Self::F1 => KeyCode::Function(1),
            Self::F2 => KeyCode::Function(2),
            Self::F3 => KeyCode::Function(3),
            Self::F4 => KeyCode::Function(4),
            Self::F5 => KeyCode::Function(5),
            Self::F6 => KeyCode::Function(6),
            Self::F7 => KeyCode::Function(7),
            Self::F8 => KeyCode::Function(8),
            Self::F9 => KeyCode::Function(9),
            Self::F10 => KeyCode::Function(10),
            Self::F11 => KeyCode::Function(11),
            Self::F12 => KeyCode::Function(12),
            Self::F13 => KeyCode::Function(13),
            Self::F14 => KeyCode::Function(14),
            Self::F15 => KeyCode::Function(15),
            Self::F16 => KeyCode::Function(16),
            Self::F17 => KeyCode::Function(17),
            Self::F18 => KeyCode::Function(18),
            Self::F19 => KeyCode::Function(19),
            Self::F20 => KeyCode::Function(20),
            Self::Kp0 => KeyCode::Numpad(0),
            Self::Kp1 => KeyCode::Numpad(1),
            Self::Kp2 => KeyCode::Numpad(2),
            Self::Kp3 => KeyCode::Numpad(3),
            Self::Kp4 => KeyCode::Numpad(4),
            Self::Kp5 => KeyCode::Numpad(5),
            Self::Kp6 => KeyCode::Numpad(6),
            Self::Kp7 => KeyCode::Numpad(7),
            Self::Kp8 => KeyCode::Numpad(8),
            Self::Kp9 => KeyCode::Numpad(9),
            Self::KpMultiply => KeyCode::Multiply,
            Self::KpDecimal => KeyCode::Decimal,
            Self::KpDivide => KeyCode::Divide,
            Self::KpPlus => KeyCode::Plus,
            Self::KpMinus => KeyCode::Minus,
            Self::KeyA => KeyCode::Char('a'),
            Self::KeyB => KeyCode::Char('b'),
            Self::KeyC => KeyCode::Char('c'),
            Self::KeyD => KeyCode::Char('d'),
            Self::KeyE => KeyCode::Char('e'),
            Self::KeyF => KeyCode::Char('f'),
            Self::KeyG => KeyCode::Char('g'),
            Self::KeyH => KeyCode::Char('h'),
            Self::KeyI => KeyCode::Char('i'),
            Self::KeyJ => KeyCode::Char('j'),
            Self::KeyK => KeyCode::Char('k'),
            Self::KeyL => KeyCode::Char('l'),
            Self::KeyM => KeyCode::Char('m'),
            Self::KeyN => KeyCode::Char('n'),
            Self::KeyO => KeyCode::Char('o'),
            Self::KeyP => KeyCode::Char('p'),
            Self::KeyQ => KeyCode::Char('q'),
            Self::KeyR => KeyCode::Char('r'),
            Self::KeyS => KeyCode::Char('s'),
            Self::KeyT => KeyCode::Char('t'),
            Self::KeyU => KeyCode::Char('u'),
            Self::KeyV => KeyCode::Char('v'),
            Self::KeyW => KeyCode::Char('w'),
            Self::KeyX => KeyCode::Char('x'),
            Self::KeyY => KeyCode::Char('y'),
            Self::KeyZ => KeyCode::Char('z'),
            Self::BackSlash => KeyCode::Char('\\'),
            Self::Comma => KeyCode::Char(','),
            Self::Backspace => KeyCode::Char('\u{8}'),
            Self::KpDelete | Self::Delete => KeyCode::Char('\u{7f}'),
            Self::End => KeyCode::End,
            Self::Home => KeyCode::Home,
            Self::Equal | Self::Equal => KeyCode::Char('='),
            Self::Escape => KeyCode::Char('\u{1b}'),
            Self::Function => KeyCode::Physical(self),
            Self::BackQuote => KeyCode::Char('`'),
            Self::Help => KeyCode::Help,
            Self::Insert => KeyCode::Insert,
            Self::Num0 => KeyCode::Char('0'),
            Self::Num1 => KeyCode::Char('1'),
            Self::Num2 => KeyCode::Char('2'),
            Self::Num3 => KeyCode::Char('3'),
            Self::Num4 => KeyCode::Char('4'),
            Self::Num5 => KeyCode::Char('5'),
            Self::Num6 => KeyCode::Char('6'),
            Self::Num7 => KeyCode::Char('7'),
            Self::Num8 => KeyCode::Char('8'),
            Self::Num9 => KeyCode::Char('9'),
            Self::Return => KeyCode::Char('\r'),
            Self::LeftBracket => KeyCode::Char('['),
            Self::RightBracket => KeyCode::Char(']'),
            Self::Minus => KeyCode::Char('-'),
            Self::VolumeMute => KeyCode::VolumeMute,
            Self::VolumeUp => KeyCode::VolumeUp,
            Self::VolumeDown => KeyCode::VolumeDown,
            Self::NumLock => KeyCode::NumLock,
            Self::PageUp => KeyCode::PageUp,
            Self::PageDown => KeyCode::PageDown,
            Self::Dot => KeyCode::Char('.'),
            Self::Quote => KeyCode::Char('\''),
            Self::SemiColon => KeyCode::Char(';'),
            Self::Slash => KeyCode::Char('/'),
            Self::Space => KeyCode::Char(' '),
            Self::Tab => KeyCode::Char('\t'),
            Self::PrintScreen => KeyCode::PrintScreen,
            Self::ScrollLock => KeyCode::ScrollLock,
            Self::Pause => KeyCode::Pause,
            Self::KpReturn => KeyCode::Char('\r'),
            Self::Apps => KeyCode::Apps,
        }
    }
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

        let modifier_vec: Vec<_> = s.split('|').map(|ele| ele.trim()).collect();
        for (value, label) in [
            (Self::NONE, "NONE"),
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
            if modifier_vec.contains(&label) {
                mods |= value;
            }
        }

        Ok(mods)
    }
}

impl From<PhysKeyCode> for Modifiers {
    fn from(phys_key: PhysKeyCode) -> Modifiers {
        let mut mods = Modifiers::NONE;

        for (key, modifier) in [
            (PhysKeyCode::AltLeft, Modifiers::ALT),
            (PhysKeyCode::AltRight, Modifiers::ALT),
            (PhysKeyCode::ControlLeft, Modifiers::CTRL),
            (PhysKeyCode::ControlRight, Modifiers::CTRL),
            (PhysKeyCode::ShiftLeft, Modifiers::SHIFT),
            (PhysKeyCode::ShiftRight, Modifiers::SHIFT),
            (PhysKeyCode::MetaLeft, Modifiers::META),
            (PhysKeyCode::MetaRight, Modifiers::META),
        ] {
            if phys_key == key {
                mods |= modifier;
            }
        }

        mods
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

    pub fn trans_positional_mods(self) -> Self {
        let mut modifiers = self;

        for (m, (left_mod, right_mod)) in [
            (Self::ALT, (Self::LEFT_ALT, Self::RIGHT_ALT)),
            (Self::CTRL, (Self::LEFT_CTRL, Self::RIGHT_CTRL)),
            (Self::SHIFT, (Self::LEFT_SHIFT, Self::RIGHT_SHIFT)),
        ] {
            if self.contains(left_mod) || self.contains(right_mod) {
                dbg!(m);
                modifiers = modifiers - left_mod - right_mod;
                modifiers |= m;
            }
        }
        modifiers
    }

    /// todo!: return extra_modifiers, missing_modifiers
    /// FIXME: linux & windows impl
    ///
    ///  Get the codes that should be clicked,
    /// modifiers of both side can be sync after clicking the keys.
    ///
    /// The modifers in the vec represent the active state of the remote modifier,
    /// compare it with the local modifiers.
    pub fn diff_modifiers(&self, modifiers: &Modifiers) -> Vec<KeyEvent> {
        let mut key_event_vec: Vec<KeyEvent> = vec![];
        let target_modifiers = modifiers;

        for pair in &[
            (Modifiers::CAPS, PhysKeyCode::CapsLock),
            (Modifiers::NUM, PhysKeyCode::NumLock),
        ] {
            let (modifier, phys) = pair.to_owned();
            let pressed = target_modifiers.contains(modifier);

            if pressed && !self.contains(modifier) || !pressed && self.contains(modifier) {
                key_event_vec.push(KeyEvent::with_phys(phys, true));
                key_event_vec.push(KeyEvent::with_phys(phys, false));
            }
            continue;
        }

        for pair in &[
            (Modifiers::SHIFT, PhysKeyCode::ShiftLeft),
            (Modifiers::CTRL, PhysKeyCode::ControlLeft),
            (Modifiers::ALT, PhysKeyCode::AltLeft),
            (Modifiers::META, PhysKeyCode::MetaLeft),
            (Modifiers::ALT_GR, PhysKeyCode::AltRight),
        ] {
            let (modifier, phys) = pair.to_owned();
            let pressed = target_modifiers.contains(modifier);

            if !pressed && self.contains(modifier) {
                key_event_vec.push(KeyEvent::with_phys(phys, false))
            }
            if pressed && !self.contains(modifier) {
                key_event_vec.push(KeyEvent::with_phys(phys, true))
            }
        }

        key_event_vec
    }

    pub fn is_shortcut(&self) -> bool {
        for mods in [
            Modifiers::CTRL,
            Modifiers::ALT,
            Modifiers::LEFT_CTRL,
            Modifiers::LEFT_ALT,
            Modifiers::RIGHT_CTRL,
            Modifiers::RIGHT_ALT,
            Modifiers::META,
        ] {
            if self.contains(mods) {
                return true;
            }
        }
        false
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RawKeyEvent {
    /// The physical location of the key on an ANSI-Standard US layout
    pub key: PhysKeyCode,
    pub press: bool,
    pub modifiers: Modifiers,
    /// The OS and hardware dependent key code for the key
    /// - windows: virtual key
    /// - linux: keysym
    pub raw_code: u32,
    /// The *other* OS and hardware dependent key code for the key
    #[cfg(windows)]
    pub scan_code: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeyEvent {
    /// Which key was pressed
    pub key: KeyCode,
    // pressed or release
    pub press: bool,
    /// Which modifiers are down
    pub modifiers: Modifiers,
    pub raw_event: Option<RawKeyEvent>,
}

impl Default for KeyEvent {
    fn default() -> KeyEvent {
        Self {
            key: KeyCode::RawCode(0),
            press: false,
            modifiers: Modifiers::NONE,
            raw_event: None,
        }
    }
}

impl KeyEvent {
    pub fn with_phys(key: PhysKeyCode, press: bool) -> Self {
        Self {
            key: KeyCode::Physical(key),
            press,
            modifiers: Modifiers::NONE,
            raw_event: None,
        }
    }

    pub fn with_keycode(key: KeyCode, press: bool) -> Self {
        Self {
            key,
            press,
            modifiers: Modifiers::NONE,
            raw_event: None,
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

    pub fn to_u8_vec(&self) -> anyhow::Result<Vec<u8>> {
        let buff: Vec<u8> = bincode::serialize(self)?;
        Ok(buff)
    }
}

pub struct KeyEventBin(Vec<u8>);

impl KeyEventBin {
    pub fn new(buf: Vec<u8>) -> Self {
        Self(buf)
    }
    pub fn to_key_event(&self) -> anyhow::Result<KeyEvent> {
        let key_event: KeyEvent = bincode::deserialize(&self.0)?;
        Ok(key_event)
    }
}

impl From<KeyEvent> for KeyEventBin {
    fn from(key_event: KeyEvent) -> Self {
        if let Ok(buff) = key_event.to_u8_vec() {
            KeyEventBin(buff)
        } else {
            KeyEventBin(vec![])
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
    Plus,
    Separator,
    Minus,
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
    Apps,
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
    if modifiers.contains(Modifiers::LEFT_CTRL) || modifiers.contains(Modifiers::RIGHT_CTRL) {
        if let KeyCode::Char(c) = key {
            if (c as u32) < 0x20 {
                let de_ctrl = ((c as u8) | 0x40) as char;
                return (KeyCode::Char(de_ctrl.to_ascii_lowercase()), modifiers);
            }
        }
    }
    (key, modifiers)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GroupIndex {
    N1,
    N2,
    N3,
    N4,
}

#[derive(Debug)]
pub enum ResolvedDeadKey {
    InvalidDeadKey,
    Combined(char),
    InvalidCombination(char),
}

pub enum ServerMode {
    Map,
    Translate,
    Auto,
}
