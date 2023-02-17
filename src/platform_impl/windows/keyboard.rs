use std::collections::HashMap;

use winapi::{
    shared::minwindef::HKL,
    um::winuser::{
        GetKeyboardLayout,
        GetKeyboardState,
        ToUnicode,
        VK_CONTROL,
        VK_LCONTROL,
        VK_LSHIFT,
        VK_LWIN,
        VK_MENU,
        VK_PACKET,
        VK_RCONTROL,
        VK_RSHIFT,
        VK_RWIN,
        VK_SHIFT,
    },
};

use crate::types::Modifiers;

#[derive(Debug)]
struct DeadKey {
    dead_char: char,
    _vk: u8,
    _mods: Modifiers,
    map: HashMap<(Modifiers, u8), char>,
}

pub struct WinKeyboard {
    layout: HKL,
    has_alt_gr: bool,
    dead_keys: HashMap<(Modifiers, u8), DeadKey>,
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
    /// This is done by synthesizing a keyboard state with control and alt
    /// pressed and then testing the virtual key presses.  If we find that
    /// one of these yields a single unicode character output then we assume that
    /// it does have AltGr.
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
            let len = ToUnicode(vk, 0, state.as_ptr(), buff.as_mut_ptr(), buff.len() as i32, 0);

            if len == 1 {
                self.has_alt_gr = true;
                break;
            }

            if len == -1 {
                // Dead key.
                // keep clocking the state to clear buff its effects
                while
                    ToUnicode(vk, 0, state.as_ptr(), buff.as_mut_ptr(), buff.len() as i32, 0) < 0
                {}
            }
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
        // self.probe_dead_keys();
        log::trace!("dead_keys: {:#x?}", self.dead_keys);

        // SetKeyboardState(saved_state.as_mut_ptr());
        self.layout = current_layout;
    }

    pub fn has_alt_gr(&mut self) -> bool {
        unsafe {
            self.update();
        }
        self.has_alt_gr
    }

    pub fn get_current_modifiers(&mut self) {
        let mut keys = [0u8; 256];
        unsafe {
            GetKeyboardState(keys.as_mut_ptr());
        }

        let mut modifiers = Modifiers::default();

        for (vk_code, modifier) in [
            (VK_SHIFT, Modifiers::SHIFT),
            (VK_LSHIFT, Modifiers::LEFT_SHIFT),
            (VK_RSHIFT, Modifiers::RIGHT_SHIFT),
            (VK_CONTROL, Modifiers::SHIFT),
            (VK_LCONTROL, Modifiers::LEFT_CTRL),
            (VK_RCONTROL, Modifiers::RIGHT_CTRL),
            (VK_LWIN, Modifiers::META),
            (VK_RWIN, Modifiers::META),
        ] {
            if Self::is_pressed(&keys, vk_code) {
                modifiers |= modifier;
            }
        }
        self.has_alt_gr();

        dbg!(modifiers);
    }

    fn is_pressed(keys: &[u8], vk_code: i32) -> bool {
        (keys[vk_code as usize] & 0x80) != 0
    }
}