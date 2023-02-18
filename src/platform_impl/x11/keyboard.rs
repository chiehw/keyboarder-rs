use crate::{
    platform_impl::build_phys_keycode_map,
    types::{KeyCode, KeyEvent},
    types::{Modifiers, PhysKeyCode},
};
use std::{
    cell::RefCell,
    collections::HashMap,
    ffi::{CStr, OsStr},
    os::unix::prelude::OsStrExt,
};

use serde::{Deserialize, Serialize};
use xkbcommon::xkb::{self};

pub const MOD_NAME_ISO_LEVEL3_SHIFT: &str = "Mod5";

pub fn query_lc_ctype() -> anyhow::Result<&'static OsStr> {
    let ptr = unsafe { libc::setlocale(libc::LC_CTYPE, std::ptr::null()) };
    anyhow::ensure!(!ptr.is_null(), "failed to query locale");

    let cstr = unsafe { CStr::from_ptr(ptr) };
    Ok(OsStr::from_bytes(cstr.to_bytes()))
}

#[inline]
pub fn level_to_modifiers(level: u32) -> Modifiers {
    match level {
        0 => Modifiers::NONE,
        1 => Modifiers::SHIFT,
        2 => Modifiers::ALT_GR,
        3 => Modifiers::SHIFT | Modifiers::ALT_GR,
        _ => Modifiers::NONE,
    }
}

pub fn build_char_event_map(
    keymap: &xkb::Keymap,
    min_keycode: u32,
    max_keycode: u32,
    layout: u32,
) -> HashMap<char, KeyEvent> {
    let mut map: HashMap<char, KeyEvent> = HashMap::new();

    // todo
    for keycode in min_keycode..=max_keycode {
        let num_level = keymap.num_levels_for_key(keycode, layout);
        for level in 0..num_level {
            let keysyms = keymap.key_get_syms_by_level(keycode, layout, level);
            if keysyms.is_empty() {
                continue;
            }
            let keysym = keysyms[0];
            let char_u32 = xkb::keysym_to_utf32(keysym);
            if let Some(chr) = char::from_u32(char_u32) {
                let key_event = KeyEvent {
                    key: KeyCode::RawCode(keycode),
                    press: false,
                    modifiers: level_to_modifiers(level),
                    click: true,
                };
                map.insert(chr, key_event);
            }
        }
    }

    map
}

pub struct XKeyboard {
    phys_code_map: RefCell<HashMap<PhysKeyCode, xkb::Keycode>>,
    code_phys_map: RefCell<HashMap<xkb::Keycode, PhysKeyCode>>,
    keysym_map: RefCell<HashMap<xkb::Keysym, xkb::Keycode>>,
    char_event_map: RefCell<HashMap<char, KeyEvent>>,
    unused_keycodes: RefCell<Vec<xkb::Keycode>>,
    state: RefCell<xkb::State>,
    keymap: xkb::Keymap,
    device_id: u8,
    layout_index: u32,
}

impl XKeyboard {
    pub fn new(connection: &xcb::Connection) -> anyhow::Result<Self> {
        let context = xkb::Context::new(xkb::CONTEXT_NO_FLAGS);
        let device_id = xkb::x11::get_core_keyboard_device_id(connection);
        anyhow::ensure!(device_id != -1, "Couldn't find core keyboard device");

        let keymap = xkb::x11::keymap_new_from_device(
            &context,
            connection,
            device_id,
            xkb::KEYMAP_COMPILE_NO_FLAGS,
        );
        let state = xkb::x11::state_new_from_device(&keymap, connection, device_id);
        let (code_phys_map, phys_code_map) = build_phys_keycode_map(&keymap);
        let mut keysym_map = HashMap::new();
        let mut unused_keycodes: Vec<xkb::Keycode> = vec![];

        let min_keycode = keymap.min_keycode();
        let max_keycode = keymap.max_keycode();

        for keycode in min_keycode..max_keycode {
            let keysym = state.key_get_one_sym(keycode);
            if keysym == 0 {
                unused_keycodes.push(keysym);
            } else {
                keysym_map.insert(keysym, keycode);
            }
        }

        let layout_num = keymap.num_layouts();
        let mut layout_index = 0;
        for idx in 0..layout_num {
            let res = state.layout_index_is_active(idx, xkb::STATE_LAYOUT_LOCKED);
            if res {
                layout_index = idx;
            }
        }

        let char_event_map: HashMap<char, KeyEvent> =
            build_char_event_map(&keymap, min_keycode, max_keycode, layout_index);

        Ok(Self {
            phys_code_map: RefCell::new(phys_code_map),
            code_phys_map: RefCell::new(code_phys_map),
            keysym_map: RefCell::new(keysym_map),
            char_event_map: RefCell::new(char_event_map),
            unused_keycodes: RefCell::new(unused_keycodes),
            state: RefCell::new(state),
            keymap,
            device_id: device_id as _,
            layout_index,
        })
    }

    /// https://stackoverflow.com/questions/69656145/how-does-modifiersas-in-xmodmap-work-under-linux-operating-system
    /// Use xmodmap -pm to get meaning of modifier  
    pub fn get_current_modifiers(&self) -> Modifiers {
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

        if self.mod_is_active(xkb::MOD_NAME_CAPS) {
            res |= Modifiers::CAPS;
        }
        if self.mod_is_active(xkb::MOD_NAME_NUM) {
            res |= Modifiers::NUM;
        }
        // todo: check
        if self.mod_is_active(MOD_NAME_ISO_LEVEL3_SHIFT) {
            res |= Modifiers::ALT_GR;
        }
        res
    }

    pub fn device_id(&self) -> u8 {
        self.device_id
    }

    pub fn get_keycode_by_keysym(&self, keysym: u32) -> Option<u32> {
        let keysym_map = self.keysym_map.borrow();
        if !keysym_map.contains_key(&keysym) {
            None
        } else {
            keysym_map.get(&keysym).copied()
        }
    }

    pub fn get_keycode_by_phys(&self, phys: PhysKeyCode) -> Option<u32> {
        let keysym_map = self.phys_code_map.borrow();
        if !keysym_map.contains_key(&phys) {
            None
        } else {
            keysym_map.get(&phys).copied()
        }
    }

    pub fn get_phys_by_keycode(&self, keycode: xkb::Keycode) -> Option<PhysKeyCode> {
        let keysym_map = self.code_phys_map.borrow();
        if !keysym_map.contains_key(&keycode) {
            None
        } else {
            keysym_map.get(&keycode).copied()
        }
    }

    pub fn get_key_event_by_char(&self, chr: char) -> Option<KeyEvent> {
        let char_map = self.char_event_map.borrow();
        if !char_map.contains_key(&chr) {
            None
        } else {
            char_map.get(&chr).clone().cloned()
        }
    }

    pub fn keysym_is_dead_key(&self, keysym: xkb::Keysym) -> bool {
        let name = xkb::keysym_get_name(keysym);
        dbg!(&name);
        name.starts_with("dead")
    }

    pub fn get_active_layout_name(&self) -> String {
        let layout_name = self.keymap.layout_get_name(self.layout_index);
        layout_name.to_owned()
    }

    fn get_active_layout_index(&self) -> u32 {
        self.layout_index
    }

    fn mod_is_active(&self, modifier: &str) -> bool {
        self.state
            .borrow()
            .mod_name_is_active(modifier, xkb::STATE_MODS_EFFECTIVE)
    }
}
