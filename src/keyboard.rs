use crate::{
    common::*,
    ffi::{
        self, MyDisplay, XkbAllocKeyboard, XkbDescPtr, XkbDescRec, XkbGetControls, XkbGetNames,
        XkbGetState, XkbStateRec, XKB_ALL_CTRLS_MASK, XKB_ALL_NAMES_MASK,
    },
};

use ffi::{XkbAllClientInfoMask, XkbUseCoreKbd};
use std::{
    collections::HashMap,
    os::raw::{c_uint, c_ulong},
};
use x11::{
    xlib::{
        self, Display, XCloseDisplay, XDefaultRootWindow, XDisplayKeycodes, XOpenDisplay,
        XQueryPointer, XkbGetMap, XkbKeycodeToKeysym,
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
    pub fn get_keycode(&self, keysym: &KeySym) -> Option<KeyCode> {
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

    pub fn get_current_modifiers(&self) {
        pub type Window = c_ulong;
        unsafe {
            let root: Window = XDefaultRootWindow(self.display);
            let mut dummy: Window = 0;
            let (mut root_x, mut root_y, mut win_x, mut win_y): (c_int, c_int, c_int, c_int) =
                (0, 0, 0, 0);
            let mut mask: c_uint = 0;
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

/// https://stackoverflow.com/questions/69656145/how-does-modifiersas-in-xmodmap-work-under-linux-operating-system
/// Use xmodmap -pm to get meaning of modifier
#[allow(non_upper_case_globals)]
pub const ShiftMask: u8 = 1;
#[allow(non_upper_case_globals)]
pub const LockMask: u8 = 2;
#[allow(non_upper_case_globals)]
pub const ControlMask: u8 = 4;
#[allow(non_upper_case_globals)]
pub const Mod1Mask: u8 = 8;
#[allow(non_upper_case_globals)]
pub const Mod2Mask: u8 = 16;
#[allow(non_upper_case_globals)]
pub const Mod3Mask: u8 = 32;
#[allow(non_upper_case_globals)]
pub const Mod4Mask: u8 = 64;
#[allow(non_upper_case_globals)]
pub const Mod5Mask: u8 = 128;

pub struct ModifierState {
    shift: bool,
    alt: bool,
    caps_lock: bool,
}

impl ModifierState {
    pub fn new(state: u32){
        // Self{
        //     shift
        // }
    }
}
