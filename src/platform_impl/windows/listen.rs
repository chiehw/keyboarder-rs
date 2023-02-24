use super::keyboard::WinKeyboard;
use crate::types::{KeyCode, KeyEvent, RawKeyEvent, ResolvedDeadKey};
use crate::types::{Modifiers, PhysKeyCode};
use parking_lot::Mutex;
use std::cell::RefCell;
use std::collections::HashMap;

use std::ptr::null_mut;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::Arc;
use std::time::Duration;
use winapi::shared::minwindef::{LPARAM, LRESULT, WPARAM};
use winapi::shared::windef::HHOOK;
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::winuser::{
    CallNextHookEx, GetKeyboardState, GetMessageA, SetWindowsHookExA, ToUnicode, HC_ACTION,
    HOOKPROC, KBDLLHOOKSTRUCT, KF_ALTDOWN, KF_EXTENDED, KF_UP, WH_KEYBOARD_LL, WM_KEYDOWN,
    WM_KEYUP, WM_SYSKEYDOWN, WM_SYSKEYUP,
};

#[allow(dead_code)]
pub const TRUE: i32 = 1;
#[allow(dead_code)]
pub const FALSE: i32 = 0;

lazy_static::lazy_static! {
    static ref EVENT_SENDER: Arc<Mutex<Option<Sender<ProcEvent>>>> = Default::default();
}

pub static mut KBD_HOOK: HHOOK = null_mut();
#[allow(unused)]
const LLKHF_EXTENDED: u32 = (KF_EXTENDED >> 8) as u32;
#[allow(unused)]
const LLKHF_LOWER_IL_INJECTED: u32 = 0x00000002;
#[allow(unused)]
const LLKHF_INJECTED: u32 = 0x00000010;
#[allow(unused)]
const LLKHF_ALTDOWN: u32 = (KF_ALTDOWN >> 8) as u32;
#[allow(unused)]
const LLKHF_UP: u32 = (KF_UP >> 8) as u32;
// <https://github.com/rustdesk/rustdesk/issues/1371>
#[allow(unused)]
const SC_FAKE_LSHIFT: u32 = 0x22A;
#[allow(unused)]
const SC_FAKE_RSHIFT: u32 = 0x236;
#[allow(unused)]
const SC_FAKE_LCTRL: u32 = 0x21D;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ProcEvent {
    wparam: WPARAM,
    lparam: LPARAM,
}

pub struct WinListener {
    event_recv: Receiver<ProcEvent>,
    keyboard: WinKeyboard,
    last_modifiers: RefCell<Modifiers>,
    // Record the stae of the modifer when long press.
    modifier_map: RefCell<HashMap<PhysKeyCode, bool>>,
    dead_pending: Option<(Modifiers, u32)>,
}

impl WinListener {
    pub fn new() -> anyhow::Result<WinListener> {
        let (event_sender, event_recv) = mpsc::channel();
        EVENT_SENDER.lock().replace(event_sender);

        std::thread::spawn(|| unsafe {
            set_hook_proc(Some(Self::keyboard_proc));
        });
        std::thread::sleep(Duration::from_millis(10));
        unsafe {
            if KBD_HOOK.is_null() {
                anyhow::bail!("Failed to set hook: {:?}", GetLastError());
            }
        }

        let modifier_map = {
            let mut m = HashMap::new();
            for phys in [
                PhysKeyCode::ShiftLeft,
                PhysKeyCode::ShiftRight,
                PhysKeyCode::ControlLeft,
                PhysKeyCode::ControlRight,
                PhysKeyCode::AltLeft,
                PhysKeyCode::AltRight,
                PhysKeyCode::MetaLeft,
                PhysKeyCode::MetaRight,
            ] {
                m.insert(phys, false);
            }

            m
        };

        Ok(Self {
            event_recv,
            keyboard: WinKeyboard::create_new(),
            last_modifiers: RefCell::new(Modifiers::NONE),
            modifier_map: RefCell::new(modifier_map),
            dead_pending: None,
        })
    }

    pub fn run_loop(&mut self) -> anyhow::Result<()> {
        loop {
            let proc_event = self.event_recv.recv()?;
            let (lparam, wparam) = (proc_event.lparam, proc_event.wparam);

            let vk_code = Self::get_vkcode(lparam);
            let scan = Self::get_scan(lparam);
            let press = Self::is_press(wparam);
            let phys_key = self.keyboard.scan_to_phys(scan);

            //  FIXME: it will be treated as LeftControl. Actually 0x001D is LeftControl.
            // <https://github.com/rustdesk/rustdesk/issues/1371>
            if scan == SC_FAKE_LCTRL {
                continue;
            }

            // Avoid to repeat process modifier key(long press)
            if let Some(phys) = phys_key {
                if phys.is_modifier() {
                    if self.is_long_press(phys, press) {
                        continue;
                    }
                    self.update_modifiers_map(phys, press);
                }
            }
            // todo: optimize this
            let mut key_states = [0u8; 256];
            unsafe { GetKeyboardState(key_states.as_mut_ptr()) };

            let modifiers = self.keyboard.get_current_modifiers();
            *self.last_modifiers.borrow_mut() = modifiers;

            let raw_key_event = RawKeyEvent {
                key: match phys_key {
                    Some(phys) => KeyCode::Physical(phys),
                    None => KeyCode::RawCode(vk_code),
                },
                press,
                phys_key,
                raw_code: vk_code,
                scan_code: scan,
                modifiers,
            };

            let is_modifier_only = phys_key.map(|p| p.is_modifier()).unwrap_or(false);
            let key = if is_modifier_only {
                phys_key.map(|p| p.to_key_code())
            } else {
                if !press && self.dead_pending.is_some() {
                    // Don't care about key-up events while processing dead keys
                    continue;
                }

                let dead: Option<KeyCode> =
                    if let Some((last_modifiers, last_vk_code)) = self.dead_pending.take() {
                        match self
                            .keyboard
                            .resolve_dead_key((last_modifiers, last_vk_code), (modifiers, vk_code))
                        {
                            ResolvedDeadKey::InvalidDeadKey => None,
                            ResolvedDeadKey::Combined(chr) => Some(KeyCode::Char(chr)),
                            ResolvedDeadKey::InvalidCombination(chr) => {
                                // dead_^ + dead_^ => dead_^ (French)
                                // They pressed the same dead key twice,
                                // emit the underlying char again and call
                                // it done.
                                if let Some(new_dead_char) =
                                    self.keyboard.is_dead_key_leader(modifiers, vk_code)
                                {
                                    if new_dead_char != chr {
                                        // Happens to be the start of its own new,
                                        // different, dead key sequence
                                        self.dead_pending.replace((modifiers, vk_code));
                                        continue;
                                    }
                                }

                                // ^ + $ => ^ (French)
                                Some(KeyCode::Char(chr))
                            }
                        }
                    } else if self
                        .keyboard
                        .is_dead_key_leader(modifiers, vk_code)
                        .is_some()
                    {
                        self.dead_pending.replace((modifiers, vk_code));
                        continue;
                    } else {
                        None
                    };
                if dead.is_some() {
                    dead
                } else {
                    // We perform conversion to unicode for ourselves,
                    // rather than calling TranslateMessage to do it for us,
                    let mut out = [0u16; 16];
                    let res = unsafe {
                        ToUnicode(
                            vk_code as u32,
                            scan as u32,
                            key_states.as_ptr(),
                            out.as_mut_ptr(),
                            out.len() as i32,
                            0,
                        )
                    };
                    match res {
                        1 => {
                            let chr = unsafe { std::char::from_u32_unchecked(out[0] as u32) };
                            Some(KeyCode::Char(chr))
                        }
                        // No mapping, so use our raw info
                        0 => {
                            log::trace!(
                                "ToUnicode had no mapping for {:?} wparam={}",
                                phys_key,
                                wparam
                            );
                            phys_key.map(|p| p.to_key_code())
                        }
                        _ => {
                            // dead key: if our dead key mapping in KeyboardLayoutInfo was
                            // correct, we shouldn't be able to get here as we should have
                            // landed in the dead key case above.
                            // If somehow we do get here, we don't have a valid mapping
                            // as -1 indicates the start of a dead key sequence,
                            // and any other n > 1 indicates an ambiguous expansion.
                            // Either way, indicate that we don't have a valid result.

                            log::error!(
                                "unexpected dead key expansion: \
                                 modifiers={:?} vk={:?} res={} pressing={} {:?}",
                                modifiers,
                                vk_code,
                                res,
                                press,
                                out
                            );
                            unsafe { WinKeyboard::clear_key_state() };
                            continue;
                        }
                    }
                }
            };

            let key = key.map(|k| {
                KeyEvent {
                    key: k,
                    press,
                    modifiers,
                    raw_event: Some(raw_key_event),
                }
                .normalize_ctrl()
            });
        }
    }

    pub fn update_modifiers_map(&self, phys: PhysKeyCode, press: bool) {
        let mut modifier_map = self.modifier_map.borrow_mut();
        if press {
            modifier_map.insert(phys, true);
        } else {
            modifier_map.insert(phys, false);
        };
    }

    pub fn is_long_press(&self, phys: PhysKeyCode, press: bool) -> bool {
        let modifier_map = self.modifier_map.borrow();
        if let Some(&state) = modifier_map.get(&phys) {
            if state && press {
                return true;
            }
        }
        false
    }

    #[inline]
    fn is_press(wparam: WPARAM) -> bool {
        match wparam.try_into() {
            Ok(WM_KEYDOWN) | Ok(WM_SYSKEYDOWN) => true,
            Ok(WM_KEYUP) | Ok(WM_SYSKEYUP) => false,
            _ => false,
        }
    }

    #[inline]
    fn get_vkcode(lparam: LPARAM) -> u32 {
        let kb = unsafe { *(lparam as *const KBDLLHOOKSTRUCT) };
        kb.vkCode
    }

    /// Get sancode with the extended-key flag
    ///
    /// The right-hand SHIFT key is not considered an extended-key, it has a separate scan code instead.
    /// refs:
    /// https://learn.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-kbdllhookstruct
    /// https://learn.microsoft.com/en-us/windows/win32/inputdev/about-keyboard-input#:~:text=The%20right%2Dhand%20SHIFT%20key%20is%20not%20considered%20an%20extended%2Dkey%2C%20it%20has%20a%20separate%20scan%20code%20instead.
    #[inline]
    fn get_scan(lpdata: LPARAM) -> u32 {
        let kb = unsafe { *(lpdata as *const KBDLLHOOKSTRUCT) };
        match kb.scanCode {
            0x36 | 0x45 => kb.scanCode,
            _ => {
                if (kb.flags & LLKHF_EXTENDED) != 0 {
                    0xE0 << 8 | kb.scanCode
                } else {
                    kb.scanCode
                }
            }
        }
    }

    /// Process keyboard event in windows
    ///
    ///  If code is less than zero, the hook procedure must pass the message to the CallNextHookEx function without further processing
    ///
    /// refs:
    /// https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowshookexa
    /// https://learn.microsoft.com/en-us/previous-versions/windows/desktop/legacy/ms644984(v=vs.85)
    unsafe extern "system" fn keyboard_proc(code: i32, wparam: WPARAM, lpdata: LPARAM) -> LRESULT {
        if code == HC_ACTION {
            let proc_event: ProcEvent = ProcEvent {
                wparam,
                lparam: lpdata,
            };

            if let Some(sender) = EVENT_SENDER.lock().as_ref() {
                sender
                    .send(proc_event)
                    .map_err(|err| log::error!("keyboard_proc send key event failed: {}", err))
                    .ok();
            }
        }

        CallNextHookEx(null_mut(), code, wparam, lpdata)
    }
}

/// Set hook proc
///
/// Safety:
/// Use it by std::thread::spawn
unsafe fn set_hook_proc(hook_proc: HOOKPROC) {
    let hook = SetWindowsHookExA(WH_KEYBOARD_LL, hook_proc, null_mut(), 0);
    KBD_HOOK = hook;

    GetMessageA(null_mut(), null_mut(), 0, 0);
}
