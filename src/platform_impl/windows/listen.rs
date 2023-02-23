use super::keyboard::WinKeyboard;
use crate::types::{KeyCode, RawKeyEvent};
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
    CallNextHookEx, GetMessageA, SetWindowsHookExA, HC_ACTION, HOOKPROC, KBDLLHOOKSTRUCT,
    KF_ALTDOWN, KF_EXTENDED, KF_UP, WH_KEYBOARD_LL, WM_KEYDOWN, WM_KEYUP, WM_SYSKEYDOWN,
    WM_SYSKEYUP,
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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ProcEvent {
    wparam: WPARAM,
    lparam: LPARAM,
}

pub struct WinListener {
    event_recv: Receiver<ProcEvent>,
    keyboard: WinKeyboard,
    last_modifiers: RefCell<Modifiers>,
    modifier_map: RefCell<HashMap<PhysKeyCode, bool>>,
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
            m.insert(PhysKeyCode::ShiftLeft, false);
            m.insert(PhysKeyCode::ShiftRight, false);

            m.insert(PhysKeyCode::ControlLeft, false);
            m.insert(PhysKeyCode::ControlRight, false);

            m.insert(PhysKeyCode::AltLeft, false);
            m.insert(PhysKeyCode::AltRight, false);

            m
        };

        Ok(Self {
            event_recv,
            keyboard: WinKeyboard::create_new(),
            last_modifiers: RefCell::new(Modifiers::NONE),
            modifier_map: RefCell::new(modifier_map),
        })
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

    pub fn run_loop(&mut self) -> anyhow::Result<()> {
        loop {
            let proc_event = self.event_recv.recv()?;
            let (lparam, wparam) = (proc_event.lparam, proc_event.wparam);

            let vk_code = Self::get_vkcode(lparam);
            let scan = Self::get_scan(lparam);
            let press = Self::is_press(wparam);
            let phys_key = self.keyboard.scan_to_phys(scan);

            // Avoid to repeat process modifier key(long press)
            if let Some(phys) = phys_key {
                if phys.is_modifier() {
                    if self.is_long_press(phys, press) {
                        continue;
                    }
                    self.update_modifiers_map(phys, press);
                }
            }

            let modifiers = self.keyboard.get_current_modifiers();
            *self.last_modifiers.borrow_mut() = modifiers;

            let _raw_key_event = RawKeyEvent {
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
                None
            };

            dbg!(key);
        }
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
