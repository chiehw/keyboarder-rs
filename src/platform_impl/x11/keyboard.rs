use crate::{
    platform_impl::platform::keycodes::build_phys_keycode_map,
    types::{GroupIndex, KeyCode, KeyEvent},
    types::{Modifiers, PhysKeyCode},
};
use std::{
    cell::RefCell,
    collections::HashMap,
    ffi::{CStr, OsStr},
    os::unix::prelude::OsStrExt,
};

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
                    raw_event: None,
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
    keymap: RefCell<xkb::Keymap>,
    device_id: u8,
    group_index: RefCell<GroupIndex>,
    context: xkb::Context,
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

        let group_index = get_active_group_index(&state, &keymap);
        {
            // Set the keyboard events that need to be monitored.
            let map_parts = xcb::xkb::MapPart::KEY_TYPES
                | xcb::xkb::MapPart::KEY_SYMS
                | xcb::xkb::MapPart::MODIFIER_MAP
                | xcb::xkb::MapPart::EXPLICIT_COMPONENTS
                | xcb::xkb::MapPart::KEY_ACTIONS
                | xcb::xkb::MapPart::KEY_BEHAVIORS
                | xcb::xkb::MapPart::VIRTUAL_MODS
                | xcb::xkb::MapPart::VIRTUAL_MOD_MAP;

            let events = xcb::xkb::EventType::NEW_KEYBOARD_NOTIFY
                | xcb::xkb::EventType::MAP_NOTIFY
                | xcb::xkb::EventType::STATE_NOTIFY;
            connection.check_request(connection.send_request_checked(&xcb::xkb::SelectEvents {
                device_spec: device_id as u16,
                affect_which: events,
                clear: xcb::xkb::EventType::empty(),
                select_all: events,
                affect_map: map_parts,
                map: map_parts,
                details: &[],
            }))?;
        }

        let char_event_map: HashMap<char, KeyEvent> =
            build_char_event_map(&keymap, min_keycode, max_keycode, group_index.into());

        Ok(Self {
            phys_code_map: RefCell::new(phys_code_map),
            code_phys_map: RefCell::new(code_phys_map),
            keysym_map: RefCell::new(keysym_map),
            char_event_map: RefCell::new(char_event_map),
            unused_keycodes: RefCell::new(unused_keycodes),
            state: RefCell::new(state),
            keymap: RefCell::new(keymap),
            device_id: device_id as _,
            group_index: RefCell::new(group_index),
            context,
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

    pub fn get_device_id(&self) -> u8 {
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
            char_map.get(&chr).cloned()
        }
    }

    pub fn keysym_is_dead_key(&self, keysym: xkb::Keysym) -> bool {
        let name = xkb::keysym_get_name(keysym);
        dbg!(&name);
        name.starts_with("dead")
    }

    pub fn get_active_layout_name(&self) -> String {
        let index = self.group_index.borrow().to_owned();
        let keymap_binding = self.keymap.borrow();

        let layout_name = keymap_binding.layout_get_name(index.into());
        layout_name.to_owned()
    }

    pub fn update_state(&self, ev: &xcb::xkb::StateNotifyEvent) {
        self.state.borrow_mut().update_mask(
            ev.base_mods().bits(),
            ev.latched_mods().bits(),
            ev.locked_mods().bits(),
            ev.base_group() as xkb::LayoutIndex,
            ev.latched_group() as xkb::LayoutIndex,
            ev.locked_group() as u32,
        );
    }

    pub fn update_keymap(
        &self,
        current_keymap: &xkb::Keymap,
        current_state: &xkb::State,
    ) -> anyhow::Result<()> {
        let (code_phys_map, phys_code_map) = build_phys_keycode_map(current_keymap);
        let mut new_keysym_map = HashMap::new();
        let mut new_unused_keycodes: Vec<xkb::Keycode> = vec![];

        let min_keycode = current_keymap.min_keycode();
        let max_keycode = current_keymap.max_keycode();

        for keycode in min_keycode..max_keycode {
            let keysym = current_state.key_get_one_sym(keycode);
            if keysym == 0 {
                new_unused_keycodes.push(keysym);
            } else {
                new_keysym_map.insert(keysym, keycode);
            }
        }

        let new_group_index = get_active_group_index(current_state, current_keymap);
        let new_char_event_map: HashMap<char, KeyEvent> = build_char_event_map(
            current_keymap,
            min_keycode,
            max_keycode,
            new_group_index.into(),
        );

        self.phys_code_map.replace(phys_code_map);
        self.code_phys_map.replace(code_phys_map);
        self.char_event_map.replace(new_char_event_map);
        self.keysym_map.replace(new_keysym_map);
        self.unused_keycodes.replace(new_unused_keycodes);

        // todo & warning: why group_index didn't change before and after replace?
        self.group_index.replace(new_group_index);

        Ok(())
    }

    pub fn update_keymaps(&self, connection: &xcb::Connection) -> anyhow::Result<()> {
        let new_keymap = xkb::x11::keymap_new_from_device(
            &self.context,
            connection,
            self.get_device_id().into(),
            xkb::KEYMAP_COMPILE_NO_FLAGS,
        );
        anyhow::ensure!(
            !new_keymap.get_raw_ptr().is_null(),
            "problem with new keymap"
        );
        let new_state =
            xkb::x11::state_new_from_device(&new_keymap, connection, self.get_device_id().into());
        anyhow::ensure!(!new_state.get_raw_ptr().is_null(), "problem with new state");

        self.update_keymap(&new_keymap, &new_state)?;

        self.state.replace(new_state);
        self.keymap.replace(new_keymap);

        Ok(())
    }

    pub fn process_xkb_event(
        &self,
        connection: &xcb::Connection,
        event: &xcb::Event,
    ) -> anyhow::Result<()> {
        log::trace!("{:?}", event);

        match event {
            xcb::Event::Xkb(xcb::xkb::Event::StateNotify(ev)) => {
                let new_group_index = GroupIndex::from(ev.group());
                let cur_group_index = self.group_index.borrow().to_owned();

                if new_group_index != cur_group_index {
                    self.update_keymap(&self.keymap.borrow(), &self.state.borrow())?;
                }

                self.update_state(ev);
            }
            xcb::Event::Xkb(
                xcb::xkb::Event::MapNotify(_) | xcb::xkb::Event::NewKeyboardNotify(_),
            ) => {
                self.update_keymaps(connection)?;
            }
            _ => {}
        }
        Ok(())
    }

    fn mod_is_active(&self, modifier: &str) -> bool {
        self.state
            .borrow()
            .mod_name_is_active(modifier, xkb::STATE_MODS_EFFECTIVE)
    }
}

pub fn get_active_group_index(state: &xkb::State, keymap: &xkb::Keymap) -> GroupIndex {
    let layout_num = keymap.num_layouts();
    let mut group_id = 0;
    for idx in 0..layout_num {
        let res = state.layout_index_is_active(idx, xkb::STATE_LAYOUT_LOCKED);
        if res {
            group_id = idx;
        }
    }
    GroupIndex::from(group_id)
}

impl From<u32> for GroupIndex {
    fn from(group_id: u32) -> Self {
        match group_id {
            0 => Self::N1,
            1 => Self::N2,
            2 => Self::N3,
            3 => Self::N4,
            _ => Self::N4,
        }
    }
}

impl From<GroupIndex> for u32 {
    fn from(group_index: GroupIndex) -> Self {
        match group_index {
            GroupIndex::N1 => 0,
            GroupIndex::N2 => 1,
            GroupIndex::N3 => 2,
            GroupIndex::N4 => 3,
        }
    }
}

impl From<xcb::xkb::Group> for GroupIndex {
    fn from(group: xcb::xkb::Group) -> Self {
        match group {
            xcb::xkb::Group::N1 => Self::N1,
            xcb::xkb::Group::N2 => Self::N2,
            xcb::xkb::Group::N3 => Self::N3,
            xcb::xkb::Group::N4 => Self::N4,
        }
    }
}
