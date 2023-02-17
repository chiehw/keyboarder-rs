use std::os::raw::c_int;
use std::ptr::null_mut;

use winapi::shared::minwindef::{LPARAM, LRESULT, WPARAM};

use winapi::shared::windef::HHOOK;

use winapi::um::winuser::{
    CallNextHookEx, GetMessageA, SetWindowsHookExA, HC_ACTION, HOOKPROC, KBDLLHOOKSTRUCT,
    VK_PROCESSKEY, WH_KEYBOARD_LL, WM_KEYDOWN, WM_KEYUP, WM_SYSKEYDOWN, WM_SYSKEYUP,
};

use crate::types::{KeyCode, KeyEvent};

#[allow(dead_code)]
pub const TRUE: i32 = 1;
#[allow(dead_code)]
pub const FALSE: i32 = 0;

pub static mut HOOK: HHOOK = null_mut();

type RawCallback = unsafe extern "system" fn(code: c_int, param: WPARAM, lpdata: LPARAM) -> LRESULT;

pub fn listen_keyboard() -> anyhow::Result<()> {
    set_key_hook(Some(keyboard_proc))?;

    Ok(())
}

fn set_key_hook(hook_proc: HOOKPROC) -> anyhow::Result<()> {
    unsafe {
        let hook = SetWindowsHookExA(WH_KEYBOARD_LL, hook_proc, null_mut(), 0);

        anyhow::ensure!(!hook.is_null(), "Failed to set hook");
        HOOK = hook;

        GetMessageA(null_mut(), null_mut(), 0, 0);
    }
    Ok(())
}

/// Process keyboard event in windows
///
/// # Safety
///
/// Use it in Hook
/// refs:
/// https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowshookexa
/// https://learn.microsoft.com/en-us/previous-versions/windows/desktop/legacy/ms644984(v=vs.85)
unsafe extern "system" fn keyboard_proc(code: i32, wparam: WPARAM, lpdata: LPARAM) -> LRESULT {
    if code == HC_ACTION {
        let key_event: KeyEvent = KeyEvent::with_lpdata_wparam(wparam, lpdata);
        dbg!(&key_event);
    }

    CallNextHookEx(HOOK, code, wparam, lpdata)
}

impl KeyEvent {
    pub fn with_lpdata_wparam(wparam: WPARAM, lparam: LPARAM) -> Self {
        let vk_code = Self::get_vkcode_by_lparam(lparam);
        let _scan_code = Self::get_scan_code_by_lparam(lparam);
        let press = Self::is_press_by_wparam(wparam);

        Self {
            key: KeyCode::RawCode(vk_code),
            press,
            ..Default::default()
        }
    }
    #[inline]
    fn is_press_by_wparam(wparam: WPARAM) -> bool {
        match wparam.try_into() {
            Ok(WM_KEYDOWN) | Ok(WM_SYSKEYDOWN) => true,
            Ok(WM_KEYUP) | Ok(WM_SYSKEYUP) => false,
            _ => false,
        }
    }

    #[inline]
    fn get_vkcode_by_lparam(lparam: LPARAM) -> u32 {
        let kb = unsafe { *(lparam as *const KBDLLHOOKSTRUCT) };
        kb.vkCode
    }

    #[inline]
    fn get_scan_code_by_lparam(lpdata: LPARAM) -> u32 {
        let kb = unsafe { *(lpdata as *const KBDLLHOOKSTRUCT) };
        // https://learn.microsoft.com/en-us/windows/win32/inputdev/about-keyboard-input#:~:text=The%20right%2Dhand%20SHIFT%20key%20is%20not%20considered%20an%20extended%2Dkey%2C%20it%20has%20a%20separate%20scan%20code%20instead.
        // The right-hand SHIFT key is not considered an extended-key, it has a separate scan code instead.
        match kb.scanCode {
            0x36 | 0x45 => kb.scanCode,
            _ => {
                if (kb.flags & 0x01) == 0x01 {
                    0xE0 << 8 | kb.scanCode
                } else {
                    kb.scanCode
                }
            }
        }
    }
}

unsafe fn key(label: &str, wparam: WPARAM, lparam: LPARAM) {
    // let kb_struct = convert_lparam_to_kb(lpdata)
    let scan_code = 1;
    let releasing = (lparam & (1 << 31)) != 0;
    let ime_active = wparam == VK_PROCESSKEY as WPARAM;

    let alt_pressed = (lparam & (1 << 29)) != 0;
    let is_extended = (lparam & (1 << 24)) != 0;
    let was_down = (lparam & (1 << 30)) != 0;

    log::trace!(
        "{} c=`{}` scan={} is_extended={} alt_pressed={} was_down={} \
             releasing={} IME={}",
        label,
        wparam,
        scan_code,
        is_extended,
        alt_pressed,
        was_down,
        releasing,
        ime_active,
    );
}
