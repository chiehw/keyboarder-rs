use crate::{
    common::*,
    event::CodeState,
    ffi::{
        self, MyDisplay, XkbAllocKeyboard, XkbDescPtr, XkbDescRec, XkbGetControls, XkbGetNames,
        XkbGetState, XkbStateRec, XKB_ALL_CTRLS_MASK, XKB_ALL_NAMES_MASK,
    },
    linux::keycodes::scancode_from_key,
};

use ffi::{XkbAllClientInfoMask, XkbUseCoreKbd};
use std::{
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

/// from: https://github.com/indianakernick/The-Fat-Controller/blob/master/src/linux_x11/mod.rs#L91
/// XkbKeycodeToKeysym will not get the correct keysym when there are multipul group.
/// The following example will get the wrong mapping.
/// e.g. input source have three input-source(Russian, Franch, English[US]),
/// then change their ordering, 108 will get different answer, some answer are wrong.
///
/// In this example, groups_num is the num of keysyms will get by this keycode.
/// Each keysym in a group corresponds to a shift level.
///
/// todo: So maybe try another solution:
/// https://stackoverflow.com/questions/10157826/xkb-how-to-convert-a-keycode-to-keysym
unsafe fn create_keysym_map_vec(
    display: *mut Display,
    min_keycode: KeyCode,
    max_keycode: KeyCode,
    xkb_desc: *mut XkbDescRec,
) -> Vec<HashMap<KeySym, KeyCode>> {
    let my_display: *mut MyDisplay = (display as *const i64) as _;
    let keyboard: XkbDescPtr = XkbAllocKeyboard();
    XkbGetNames(my_display, XKB_ALL_NAMES_MASK, keyboard);
    XkbGetControls(my_display, XKB_ALL_CTRLS_MASK, keyboard);
    let group_source_size = (*(*keyboard).names).groups.len();

    let mut key_map_vec: Vec<HashMap<KeySym, KeyCode>> = Vec::with_capacity(group_source_size);
    for _i in 0..group_source_size {
        let key_map = HashMap::new();
        key_map_vec.push(key_map);
    }

    let level = 0;
    for keycode in min_keycode..=max_keycode {
        let groups_num = ffi::XkbKeyNumGroups(xkb_desc, keycode as u8);

        if groups_num == 0 {
            continue;
        } else if groups_num == 1 {
            let keysym = XkbKeycodeToKeysym(display, keycode as u8, 0, level);
            for id in 0..group_source_size {
                let key_map = key_map_vec.get_mut(id).unwrap();
                key_map.insert(keysym as u32, keycode);
            }
        } else {
            for id in 0..groups_num {
                let keysym = XkbKeycodeToKeysym(display, keycode as u8, id, level);
                let key_map = key_map_vec.get_mut(id as usize).unwrap();
                key_map.insert(keysym as u32, keycode);
            }
        }
    }

    key_map_vec
}

unsafe fn find_unused_key_code(
    min_keycode: KeyCode,
    max_keycode: KeyCode,
    xkb_desc: *mut XkbDescRec,
) -> Vec<KeyCode> {
    let mut res: Vec<KeyCode> = Vec::new();
    for keycode in min_keycode..=max_keycode {
        let groups = ffi::XkbKeyNumGroups(xkb_desc, keycode as u8);
        if groups == 0 {
            res.push(keycode);
        }
    }

    res
}

pub struct Keyboard {
    display: *mut Display,
    // todo: when the num of input methods than three.
    keysym_map_vec: Vec<HashMap<KeySym, KeyCode>>,
    unused_keycode: Vec<KeyCode>,
}

impl Keyboard {
    pub fn new() -> Result<Self> {
        unsafe {
            let display = XOpenDisplay(std::ptr::null());
            if display.is_null() {
                return Err(anyhow!("Missing Display, Try `export Display=:0`"));
            }

            // This is alway 8-255 on linux.
            let mut min_keycode = 0;
            let mut max_keycode = 0;
            XDisplayKeycodes(display, &mut min_keycode, &mut max_keycode);
            let min_keycode = min_keycode as KeyCode;
            let max_keycode = max_keycode as KeyCode;

            // https://github.com/chiehw/blog/blob/main/2022/8.md#%E8%B7%A8-crate
            // xlib XkbClientMapRec is not complete.
            let xkb_desc = XkbGetMap(display, XkbAllClientInfoMask, XkbUseCoreKbd);
            if xkb_desc.is_null() {
                return Err(anyhow!("Failed XkbGetMap"));
            }
            let xkb_desc: *mut XkbDescRec = (xkb_desc as *const i64) as _;

            let keysym_map_vec = create_keysym_map_vec(display, min_keycode, max_keycode, xkb_desc);
            let unused_keycode = find_unused_key_code(min_keycode, max_keycode, xkb_desc);

            Ok(Self {
                display,
                keysym_map_vec,
                unused_keycode,
            })
        }
    }
}

impl Drop for Keyboard {
    fn drop(&mut self) {
        unsafe {
            XCloseDisplay(self.display);
        }
    }
}

impl Keyboard {
    pub fn get_modifier_state(&self) -> ModifierState {
        pub type Window = c_ulong;
        let mut mask: c_uint = 0;
        unsafe {
            let root: Window = XDefaultRootWindow(self.display);
            let mut dummy: Window = 0;
            let (mut root_x, mut root_y, mut win_x, mut win_y): (c_int, c_int, c_int, c_int) =
                (0, 0, 0, 0);

            XQueryPointer(
                self.display,
                root,
                &mut dummy,
                &mut dummy,
                &mut root_x,
                &mut root_y,
                &mut win_x,
                &mut win_y,
                &mut mask,
            );
        }
        ModifierState::new(mask)
    }

    fn get_keycode(&self, keysym: &KeySym) -> Option<KeyCode> {
        let group = unsafe {
            let mut state: XkbStateRec = std::mem::zeroed();
            let display: *mut MyDisplay = (self.display as *const i64) as _;
            XkbGetState(display, XkbUseCoreKbd, &mut state);
            state.group
        };

        if let Some(keysym_map) = self.keysym_map_vec.get((group + 1) as usize) {
            if !keysym_map.contains_key(keysym) {
                None
            } else {
                keysym_map.get(keysym).copied()
            }
        } else {
            None
        }
    }

    fn send_native(&self, display: *mut Display, keycode: KeyCode, down: bool) -> Option<()> {
        let res = match down {
            true => unsafe { xtest::XTestFakeKeyEvent(display, keycode, TRUE, 0) },
            false => unsafe { xtest::XTestFakeKeyEvent(display, keycode, FALSE, 0) },
        };
        if res == 0 {
            None
        } else {
            Some(())
        }
    }
}

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

    /// Get the codes that should be clicked,
    /// modifiers of both side can be sync after clicking the keys.
    ///
    /// The modifers in the vec represent the active state of the remote modifier,
    /// compare it with the local modifiers.
    pub fn compare_modifers(&self, modifiers: &[Key]) -> Vec<CodeState> {
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
