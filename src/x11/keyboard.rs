use crate::event::CodeState;
use anyhow::ensure;
use bitflags::*;
use std::{
    cell::RefCell,
    collections::HashMap,
    ffi::{CStr, OsStr},
    os::unix::prelude::OsStrExt,
};

use xkbcommon::xkb;

#[derive(Debug)]
pub struct ModifierState {
    shift: bool,
    ctrl: bool,
    alt: bool,
    meta: bool,
}

impl ModifierState {
    pub fn new(mask: u32) -> Self {
        Self {
            shift: mask & ShiftMask == ShiftMask,
            ctrl: mask & ControlMask == ControlMask,
            alt: mask & Mod1Mask == Mod1Mask,
            meta: mask & Mod4Mask == Mod4Mask,
        }
    }

    /// todo!: return extra_modifiers, missing_modifiers
    ///
    ///  Get the codes that should be clicked,
    /// modifiers of both side can be sync after clicking the keys.
    ///
    /// The modifers in the vec represent the active state of the remote modifier,
    /// compare it with the local modifiers.
    pub fn diff_modifiers(&self, modifiers: &[Key]) -> Vec<CodeState> {
        let mut codes: Vec<CodeState> = vec![];

        let shift = modifiers.contains(&Key::ShiftLeft) || modifiers.contains(&Key::ShiftRight);
        let ctrl = modifiers.contains(&Key::ControlLeft) || modifiers.contains(&Key::ControlRight);
        let alt = modifiers.contains(&Key::AltLeft) || modifiers.contains(&Key::AltRight);
        let meta = modifiers.contains(&Key::MetaLeft) || modifiers.contains(&Key::MetaRight);

        if !shift && self.shift {
            codes.push(CodeState::with_key(Key::ShiftLeft, false))
        }
        if shift && !self.shift {
            codes.push(CodeState::with_key(Key::ShiftLeft, true))
        }
        if !ctrl && self.ctrl {
            codes.push(CodeState::with_key(Key::ControlLeft, false))
        }
        if ctrl && !self.ctrl {
            codes.push(CodeState::with_key(Key::ControlLeft, true))
        }
        if !alt && self.alt {
            codes.push(CodeState::with_key(Key::AltLeft, false))
        }
        if alt && !self.alt {
            codes.push(CodeState::with_key(Key::AltLeft, true))
        }
        if !meta && self.meta {
            codes.push(CodeState::with_key(Key::MetaLeft, false))
        }
        if meta && !self.meta {
            codes.push(CodeState::with_key(Key::MetaLeft, true))
        }

        codes
    }
}

/// https://stackoverflow.com/questions/69656145/how-does-modifiersas-in-xmodmap-work-under-linux-operating-system
/// Use xmodmap -pm to get meaning of modifier
#[allow(non_upper_case_globals)]
pub const ShiftMask: u32 = 1; // shift
#[allow(non_upper_case_globals)]
pub const LockMask: u32 = 2; // caps_lock
#[allow(non_upper_case_globals)]
pub const ControlMask: u32 = 4; // contrl
#[allow(non_upper_case_globals)]
pub const Mod1Mask: u32 = 8; // alt_l
#[allow(non_upper_case_globals)]
pub const Mod2Mask: u32 = 16; // num_lock
#[allow(non_upper_case_globals)]
pub const Mod3Mask: u32 = 32;
#[allow(non_upper_case_globals)]
pub const Mod4Mask: u32 = 64; // super_l
#[allow(non_upper_case_globals)]
pub const Mod5Mask: u32 = 128; // iso_level3_shift

/// https://github.com/fufesou/rdev/blob/cedc4e62744566775026af4b434ef799804c1130/src/rdev.rs#L112
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Key {
    AltLeft,
    AltRight, // AltGr
    Backspace,
    CapsLock,
    ControlLeft,
    ControlRight,
    Delete,
    DownArrow,
    End,
    Escape,
    F1,
    F2,
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
    Home,
    LeftArrow,
    MetaLeft,
    MetaRight,
    PageDown,
    PageUp,
    Return,
    RightArrow,
    ShiftLeft,
    ShiftRight,
    Space,
    Tab,
    UpArrow,
    PrintScreen,
    ScrollLock,
    Pause,
    NumLock,
    BackQuote,
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
    IntlBackslash,
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
    Apps,
}

impl Into<u32> for Key {
    fn into(self) -> u32 {
        scancode_from_key(self)
    }
}

use serde::{Deserialize, Serialize};

use super::scancode_from_key;

/// These keycodes identify keys based on their physical
/// position on an ANSI-standard US keyboard.
#[derive(
    Debug,
    Deserialize,
    Serialize,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Copy,
    Ord,
    PartialOrd,
    // FromDynamic,
    // ToDynamic,
)]
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

pub fn query_lc_ctype() -> anyhow::Result<&'static OsStr> {
    let ptr = unsafe { libc::setlocale(libc::LC_CTYPE, std::ptr::null()) };
    ensure!(!ptr.is_null(), "failed to query locale");

    let cstr = unsafe { CStr::from_ptr(ptr) };
    Ok(OsStr::from_bytes(cstr.to_bytes()))
}

pub fn build_physkeycode_map(keymap: &xkb::Keymap) -> HashMap<xkb::Keycode, PhysKeyCode> {
    let mut map = HashMap::new();

    // See <https://abaines.me.uk/updates/linux-x11-keys> for info on
    // these names and how they relate to the ANSI standard US keyboard
    // See also </usr/share/X11/xkb/keycodes/evdev> on a Linux system
    // to examine the mapping. FreeBSD and other unixes will use a different
    // set of keycode values.
    // We're using the symbolic names here to look up the keycodes that
    // correspond to the various key locations.
    for (name, phys) in &[
        ("ESC", PhysKeyCode::Escape),
        ("FK01", PhysKeyCode::F1),
        ("FK02", PhysKeyCode::F2),
        ("FK03", PhysKeyCode::F3),
        ("FK04", PhysKeyCode::F4),
        ("FK05", PhysKeyCode::F5),
        ("FK06", PhysKeyCode::F6),
        ("FK07", PhysKeyCode::F7),
        ("FK08", PhysKeyCode::F8),
        ("FK09", PhysKeyCode::F9),
        ("FK10", PhysKeyCode::F10),
        ("FK11", PhysKeyCode::F11),
        ("FK12", PhysKeyCode::F12),
        ("PRSC", PhysKeyCode::PrintScreen),
        ("SCLK", PhysKeyCode::ScrollLock),
        ("PAUS", PhysKeyCode::Pause),
        ("TLDE", PhysKeyCode::BackQuote),
        ("AE01", PhysKeyCode::Num1),
        ("AE02", PhysKeyCode::Num2),
        ("AE03", PhysKeyCode::Num3),
        ("AE04", PhysKeyCode::Num4),
        ("AE05", PhysKeyCode::Num5),
        ("AE06", PhysKeyCode::Num6),
        ("AE07", PhysKeyCode::Num7),
        ("AE08", PhysKeyCode::Num8),
        ("AE09", PhysKeyCode::Num9),
        ("AE10", PhysKeyCode::Num0),
        ("AE11", PhysKeyCode::Minus),
        ("AE12", PhysKeyCode::Equal),
        ("BKSL", PhysKeyCode::BackSlash),
        ("BKSP", PhysKeyCode::Backspace),
        ("INS", PhysKeyCode::Insert),
        ("HOME", PhysKeyCode::Home),
        ("PGUP", PhysKeyCode::PageUp),
        ("NMLK", PhysKeyCode::NumLock),
        ("KPDV", PhysKeyCode::KpDivide),
        ("KPMU", PhysKeyCode::KpMultiply),
        ("KPSU", PhysKeyCode::KpMinus),
        ("TAB", PhysKeyCode::Tab),
        ("AD01", PhysKeyCode::KeyQ),
        ("AD02", PhysKeyCode::KeyW),
        ("AD03", PhysKeyCode::KeyE),
        ("AD04", PhysKeyCode::KeyR),
        ("AD05", PhysKeyCode::KeyT),
        ("AD06", PhysKeyCode::KeyY),
        ("AD07", PhysKeyCode::KeyU),
        ("AD08", PhysKeyCode::KeyI),
        ("AD09", PhysKeyCode::KeyO),
        ("AD10", PhysKeyCode::KeyP),
        ("AD11", PhysKeyCode::LeftBracket),
        ("AD12", PhysKeyCode::RightBracket),
        ("DELE", PhysKeyCode::Delete),
        ("END", PhysKeyCode::End),
        ("PGDN", PhysKeyCode::PageDown),
        ("KP7", PhysKeyCode::Kp7),
        ("KP8", PhysKeyCode::Kp8),
        ("KP9", PhysKeyCode::Kp9),
        ("KPAD", PhysKeyCode::KpPlus),
        ("CAPS", PhysKeyCode::CapsLock),
        ("AC01", PhysKeyCode::KeyA),
        ("AC02", PhysKeyCode::KeyS),
        ("AC03", PhysKeyCode::KeyD),
        ("AC04", PhysKeyCode::KeyF),
        ("AC05", PhysKeyCode::KeyG),
        ("AC06", PhysKeyCode::KeyH),
        ("AC07", PhysKeyCode::KeyJ),
        ("AC08", PhysKeyCode::KeyK),
        ("AC09", PhysKeyCode::KeyL),
        ("AC10", PhysKeyCode::SemiColon),
        ("AC11", PhysKeyCode::Quote),
        ("RTRN", PhysKeyCode::Return),
        ("KP4", PhysKeyCode::Kp4),
        ("KP5", PhysKeyCode::Kp5),
        ("KP6", PhysKeyCode::Kp6),
        ("LFSH", PhysKeyCode::ShiftLeft),
        ("AB01", PhysKeyCode::KeyZ),
        ("AB02", PhysKeyCode::KeyX),
        ("AB03", PhysKeyCode::KeyC),
        ("AB04", PhysKeyCode::KeyV),
        ("AB05", PhysKeyCode::KeyB),
        ("AB06", PhysKeyCode::KeyN),
        ("AB07", PhysKeyCode::KeyM),
        ("AB08", PhysKeyCode::Comma),
        ("AB09", PhysKeyCode::Dot),
        ("AB10", PhysKeyCode::Slash),
        ("RTSH", PhysKeyCode::ShiftRight),
        ("UP", PhysKeyCode::UpArrow),
        ("KP1", PhysKeyCode::Kp1),
        ("KP2", PhysKeyCode::Kp2),
        ("KP3", PhysKeyCode::Kp3),
        ("KPEN", PhysKeyCode::KpEnter),
        ("LCTL", PhysKeyCode::ControlLeft),
        ("LALT", PhysKeyCode::AltLeft),
        ("SPCE", PhysKeyCode::Space),
        ("RALT", PhysKeyCode::AltRight),
        ("RCTL", PhysKeyCode::ControlRight),
        ("LEFT", PhysKeyCode::LeftArrow),
        ("DOWN", PhysKeyCode::DownArrow),
        ("RGHT", PhysKeyCode::RightArrow),
        ("KP0", PhysKeyCode::Kp0),
        ("KPDL", PhysKeyCode::KpDelete),
        ("LWIN", PhysKeyCode::MetaLeft),
        ("RWIN", PhysKeyCode::MetaRight),
        ("MUTE", PhysKeyCode::VolumeMute),
        ("VOL-", PhysKeyCode::VolumeDown),
        ("VOL+", PhysKeyCode::VolumeUp),
        ("HELP", PhysKeyCode::Help),
    ] {
        if let Some(code) = keymap.key_by_name(name) {
            map.insert(code, *phys);
        }
    }

    map
}

pub struct XKeyboard {
    phys_code_map: RefCell<HashMap<xkb::Keycode, PhysKeyCode>>,
    state: RefCell<xkb::State>,
    device_id: u8,
}

impl XKeyboard {
    pub fn new(connection: &xcb::Connection) -> anyhow::Result<Self> {
        let context = xkb::Context::new(xkb::CONTEXT_NO_FLAGS);
        let device_id = xkb::x11::get_core_keyboard_device_id(connection);
        ensure!(device_id != -1, "Couldn't find core keyboard device");

        let keymap = xkb::x11::keymap_new_from_device(
            &context,
            connection,
            device_id,
            xkb::KEYMAP_COMPILE_NO_FLAGS,
        );
        let state = xkb::x11::state_new_from_device(&keymap, connection, device_id);
        let phys_code_map = build_physkeycode_map(&keymap);

        Ok(Self {
            phys_code_map: RefCell::new(phys_code_map),
            state: RefCell::new(state),
            device_id: device_id as _,
        })
    }

    pub fn process_key_event_impl(&self) {
        let xcode: xkb::Keycode = 9;
        let _pressed: bool = true;

        let _phys_code = self.phys_code_map.borrow().get(&xcode).copied();
        let raw_modifiers = self.get_key_modifiers();
        dbg!(raw_modifiers);
    }

    /// https://stackoverflow.com/questions/69656145/how-does-modifiersas-in-xmodmap-work-under-linux-operating-system
    /// Use xmodmap -pm to get meaning of modifier  
    pub fn get_key_modifiers(&self) -> Modifiers {
        let mut res = Modifiers::default();

        if self.mod_is_active(xkb::MOD_NAME_SHIFT) {
            res |= Modifiers::SHIFT;
        }
        if self.mod_is_active(xkb::MOD_NAME_CTRL) {
            res |= Modifiers::CTRL;
        }
        if self.mod_is_active(xkb::MOD_NAME_ALT) {
            res |= Modifiers::ALT;
        }
        if self.mod_is_active(xkb::MOD_NAME_LOGO) {
            res |= Modifiers::META;
        }
        res
    }

    pub fn device_id(&self) -> u8 {
        self.device_id
    }

    fn mod_is_active(&self, modifier: &str) -> bool {
        self.state
            .borrow()
            .mod_name_is_active(modifier, xkb::STATE_MODS_EFFECTIVE)
    }
}

bitflags! {
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
            (Self::META, "Meta"),
            (Self::LEFT_ALT, "LEFT_ALT"),
            (Self::RIGHT_ALT, "RIGHT_ALT"),
            (Self::LEFT_CTRL, "LEFT_CTRL"),
            (Self::RIGHT_CTRL, "RIGHT_CTRL"),
            (Self::LEFT_SHIFT, "LEFT_SHIFT"),
            (Self::RIGHT_SHIFT, "RIGHT_SHIFT"),
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
}
