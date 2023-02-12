use crate::{
    event::CodeState,
    x11::{
        ffi::{
            self, MyDisplay, XkbAllocKeyboard, XkbDescPtr, XkbDescRec, XkbGetControls, XkbGetNames,
            XkbGetState, XkbStateRec, XKB_ALL_CTRLS_MASK, XKB_ALL_NAMES_MASK,
        },
        *,
    },
};
use ffi::{XkbAllClientInfoMask, XkbUseCoreKbd};
use std::{
    cell::RefCell,
    collections::HashMap,
    os::raw::{c_uint, c_ulong},
};
use x11::{
    xlib::{
        Display, XCloseDisplay, XDefaultRootWindow, XDisplayKeycodes, XOpenDisplay, XQueryPointer,
        XkbGetMap, XkbKeycodeToKeysym,
    },
    xtest,
};

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
