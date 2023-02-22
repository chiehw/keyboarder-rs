use std::collections::HashMap;

use winapi::{
    shared::minwindef::HKL,
    um::winuser::{
        GetKeyboardLayout, GetKeyboardState, MapVirtualKeyW, SetKeyboardState, ToUnicode,
        MAPVK_VK_TO_VSC, VK_CONTROL, VK_DECIMAL, VK_LCONTROL, VK_LMENU, VK_LSHIFT, VK_LWIN,
        VK_MENU, VK_PACKET, VK_RCONTROL, VK_RMENU, VK_RSHIFT, VK_RWIN, VK_SHIFT,
    },
};

use crate::types::Modifiers;

pub type VirtualKey = u8;

#[derive(Debug)]
struct DeadKey {
    dead_char: char,
    _vk: VirtualKey,
    _mods: Modifiers,
    map: HashMap<(Modifiers, VirtualKey), char>,
}

pub struct WinKeyboard {
    layout: HKL,
    has_alt_gr: bool,
    dead_keys: HashMap<(Modifiers, VirtualKey), DeadKey>,
}

impl Default for WinKeyboard {
    fn default() -> Self {
        Self::new()
    }
}

impl WinKeyboard {
    pub fn new() -> Self {
        Self {
            layout: std::ptr::null_mut(),
            has_alt_gr: false,
            dead_keys: HashMap::new(),
        }
    }

    /// Probe to detect whether an AltGr key is present.
    ///
    /// This is done by synthesizing a keyboard state with control and alt
    /// pressed and then testing the virtual key presses.  If we find that
    /// one of these yields a single unicode character output then we assume that
    /// it does have AltGr.
    ///
    /// refs: <https://github.com/wez/wezterm/commit/7ddff705a422dcc9b0a607d1fabbe08aeddbc24a>
    unsafe fn probe_alt_gr(&mut self) {
        self.has_alt_gr = false;

        let mut state = [0u8; 256];
        state[VK_CONTROL as usize] = 0x80;
        state[VK_MENU as usize] = 0x80;

        for vk in 0..=255u32 {
            if vk == (VK_PACKET as u32) {
                // Avoid false positives
                continue;
            }

            let mut buff = [0u16; 16];
            let len = ToUnicode(
                vk,
                0,
                state.as_ptr(),
                buff.as_mut_ptr(),
                buff.len() as i32,
                0,
            );

            if len == 1 {
                self.has_alt_gr = true;
                break;
            }

            if len == -1 {
                // Dead key.
                // keep clocking the state to clear buff its effects
                while ToUnicode(
                    vk,
                    0,
                    state.as_ptr(),
                    buff.as_mut_ptr(),
                    buff.len() as i32,
                    0,
                ) < 0
                {}
            }
        }
    }

    /// Probe the keymap to figure out which keys are dead keys
    ///
    /// none, shift, altgr, shift + altgr generate char, traverse all possible
    /// generated characters.
    unsafe fn probe_dead_keys(&mut self) {
        self.dead_keys.clear();

        let shift_states = [
            Modifiers::NONE,
            Modifiers::SHIFT,
            Modifiers::ALT_GR,                    // AltGr
            Modifiers::SHIFT | Modifiers::ALT_GR, // shift + altgr
        ];

        for &mods in &shift_states {
            let mut state = [0u8; 256];
            Self::apply_mods(mods, &mut state);

            for vk in 0..=255u32 {
                if vk == (VK_PACKET as u32) {
                    // Avoid false positives
                    continue;
                }

                let scan = MapVirtualKeyW(vk, MAPVK_VK_TO_VSC);

                Self::clear_key_state();
                let mut out = [0u16; 16];
                let ret = ToUnicode(
                    vk,
                    scan,
                    state.as_ptr(),
                    out.as_mut_ptr(),
                    out.len() as i32,
                    0,
                );

                if ret != -1 {
                    continue;
                }

                // Found a Dead key.
                let dead_char = std::char::from_u32_unchecked(out[0] as u32);

                let mut map = HashMap::new();
                for &sec_mods in &shift_states {
                    let mut second_state = [0u8; 256];
                    Self::apply_mods(sec_mods, &mut second_state);

                    for sec_vk in 0..=255u32 {
                        if sec_vk == (VK_PACKET as u32) {
                            // Avoid false positives
                            continue;
                        }

                        // Re-initiate the dead key starting state
                        Self::clear_key_state();
                        if ToUnicode(
                            vk,
                            scan,
                            state.as_ptr(),
                            out.as_mut_ptr(),
                            out.len() as i32,
                            0,
                        ) != -1
                        {
                            continue;
                        }

                        let sec_scan = MapVirtualKeyW(sec_vk, MAPVK_VK_TO_VSC);

                        let ret = ToUnicode(
                            sec_vk,
                            sec_scan,
                            second_state.as_ptr(),
                            out.as_mut_ptr(),
                            out.len() as i32,
                            0,
                        );

                        if ret == 1 {
                            // Found a combination
                            let chr = std::char::from_u32_unchecked(out[0] as u32);

                            // clock through again to get the base char
                            ToUnicode(
                                sec_vk,
                                scan,
                                second_state.as_ptr(),
                                out.as_mut_ptr(),
                                out.len() as i32,
                                0,
                            );
                            let base_chr = std::char::from_u32_unchecked(out[0] as u32);

                            if ((sec_mods == Modifiers::CTRL)
                                || (sec_mods == Modifiers::CTRL | Modifiers::SHIFT))
                                && chr == base_chr
                                && (chr as u32) < 0x20
                            {
                                continue;
                            }
                            log::trace!(
                                "{:?}: ({:?} + {:?}) + ({:?} {:?}) => base={:?}, {:?}",
                                dead_char,
                                mods,
                                vk,
                                sec_mods,
                                sec_vk,
                                base_chr,
                                chr
                            );

                            map.insert((sec_mods, vk as u8), chr);
                        }
                    }
                }

                self.dead_keys.insert(
                    (mods, vk as u8),
                    DeadKey {
                        dead_char,
                        _mods: mods,
                        _vk: vk as u8,
                        map,
                    },
                );
            }
        }
    }

    /// keep clocking the state to clear out its effects
    ///
    /// Generate unicode is generated according to the state of
    /// the system keyboard.
    unsafe fn clear_key_state() {
        let mut out = [0u16; 16];
        let state = [0u8; 256];
        let scan = MapVirtualKeyW(VK_DECIMAL as _, MAPVK_VK_TO_VSC);

        while ToUnicode(
            VK_DECIMAL as _,
            scan,
            state.as_ptr(),
            out.as_mut_ptr(),
            out.len() as i32,
            0,
        ) < 0
        {}
    }

    fn apply_mods(mods: Modifiers, state: &mut [u8; 256]) {
        if mods.contains(Modifiers::SHIFT) {
            state[VK_SHIFT as usize] = 0x80;
        }
        if mods.contains(Modifiers::ALT_GR) {
            state[VK_CONTROL as usize] = 0x80;
            state[VK_MENU as usize] = 0x80;
        }
    }

    unsafe fn update(&mut self) {
        let current_layout = GetKeyboardLayout(0);
        // Avoid recomputing this if the layout hasn't changed
        if current_layout == self.layout {
            return;
        }

        let mut saved_state = [0u8; 256];
        if GetKeyboardState(saved_state.as_mut_ptr()) == 0 {
            return;
        }

        self.probe_alt_gr();
        self.probe_dead_keys();
        dbg!(self.has_alt_gr);
        log::trace!("dead_keys: {:#x?}", self.dead_keys);

        // Todo: Maybe SetKeyboardState can use in release key.
        SetKeyboardState(saved_state.as_mut_ptr());
        self.layout = current_layout;
    }

    pub fn has_alt_gr(&mut self) -> bool {
        unsafe {
            self.update();
        }
        self.has_alt_gr
    }

    pub fn get_current_modifiers(&mut self) {
        let mut states = [0u8; 256];
        unsafe {
            GetKeyboardState(states.as_mut_ptr());
        }

        let mut modifiers = Modifiers::default();

        for (vk_code, modifier) in [
            (VK_SHIFT, Modifiers::SHIFT),
            (VK_LSHIFT, Modifiers::LEFT_SHIFT),
            (VK_RSHIFT, Modifiers::RIGHT_SHIFT),
            (VK_CONTROL, Modifiers::CTRL),
            (VK_LCONTROL, Modifiers::LEFT_CTRL),
            (VK_RCONTROL, Modifiers::RIGHT_CTRL),
            (VK_MENU, Modifiers::ALT),
            (VK_LMENU, Modifiers::LEFT_ALT),
            (VK_RMENU, Modifiers::RIGHT_ALT),
            (VK_LWIN, Modifiers::META),
            (VK_RWIN, Modifiers::META),
        ] {
            if Self::is_pressed(&states, vk_code) {
                modifiers |= modifier;
            }
        }
        self.has_alt_gr();

        dbg!(modifiers);
    }

    fn is_pressed(states: &[u8], vk_code: i32) -> bool {
        (states[vk_code as usize] & 0x80) != 0
    }

    pub fn is_dead_key_leader(&mut self, mods: Modifiers, vk: u32) -> Option<char> {
        unsafe {
            self.update();
        }
        if vk <= u8::MAX.into() {
            self.dead_keys
                .get(&(Self::fixup_mods(mods), vk as u8))
                .map(|dead| dead.dead_char)
        } else {
            None
        }
    }

    fn fixup_mods(mods: Modifiers) -> Modifiers {
        mods - (Modifiers::LEFT_SHIFT
            | Modifiers::RIGHT_SHIFT
            | Modifiers::LEFT_CTRL
            | Modifiers::RIGHT_CTRL
            | Modifiers::LEFT_ALT)
    }
}
