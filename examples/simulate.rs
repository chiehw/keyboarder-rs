use keyboarder::event::{Event, KeyboardEvent};
use keyboarder::keycodes::PhysKeyCode;
#[warn(unused_imports)]
use keyboarder::x11::{anyhow, ensure};
use keyboarder::x11::{Key, ModifierState};
use keyboarder::{
    event::{Code, State},
    event_handler::EventHandler,
};
use libc;

use std::collections::HashMap;
use std::ffi::{CStr, OsStr};
use std::os::unix::prelude::OsStrExt;
use std::sync::Mutex;
use tfc::Context;
use xkbcommon::xkb::{self};

lazy_static::lazy_static! {
    static ref KBD_CONTEXT: Mutex<Context> = Mutex::new(Context::new().expect("error"));
}

fn handle_events(
    handler: &mut EventHandler,
    // dispatcher: &mut ActionDispatcher,
    event: &Event,
) -> anyhow::Result<()> {
    let _actions = handler
        .on_event(event)
        .map_err(|e| anyhow!("Failed handling {event:?}:\n  {e:?}"))?;
    // for action in actions {
    //     dispatcher.on_action(action)?;
    // }
    Ok(())
}

fn main() -> anyhow::Result<()> {
    env_logger::init();
    std::env::set_var("DISPLAY", ":0");

    /// todo:
    /// 1. return extra_modifiers, missing_modifiers
    /// 2. maintain modifer by myself
    /// 3. try xcb
    use xcb;
    use xkbcommon::xkb;

    let (connection, screen_num) =
        xcb::Connection::connect_with_xlib_display_and_extensions(&[xcb::Extension::Xkb], &[])?;
    let context = xkb::Context::new(xkb::CONTEXT_NO_FLAGS);
    let device_id = xkb::x11::get_core_keyboard_device_id(&connection);
    ensure!(device_id != -1, "Couldn't find core keyboard device");

    let keymap = xkb::x11::keymap_new_from_device(
        &context,
        &connection,
        device_id,
        xkb::KEYMAP_COMPILE_NO_FLAGS,
    );
    let state = xkb::x11::state_new_from_device(&keymap, &connection, device_id);
    /// why?
    let locale = query_lc_ctype()?;
    dbg!(locale);

    let table =
        xkb::compose::Table::new_from_locale(&context, locale, xkb::compose::COMPILE_NO_FLAGS)
            .map_err(|_| anyhow!("Failed to acquire compose table from locale"))?;
    let compose_state = xkb::compose::State::new(&table, xkb::compose::STATE_NO_FLAGS);

    let phys_code_map = build_physkeycode_map(&keymap);

    Ok(())
}

fn query_lc_ctype() -> anyhow::Result<&'static OsStr> {
    let ptr = unsafe { libc::setlocale(libc::LC_CTYPE, std::ptr::null()) };
    ensure!(!ptr.is_null(), "failed to query locale");

    let cstr = unsafe { CStr::from_ptr(ptr) };
    Ok(OsStr::from_bytes(cstr.to_bytes()))
}

fn build_physkeycode_map(keymap: &xkb::Keymap) -> HashMap<xkb::Keycode, PhysKeyCode> {
    let mut map = HashMap::new();

    // See <https://abaines.me.uk/updates/linux-x11-keys> for info on
    // these names and how they relate to the ANSI standard US keyboard
    // See also </usr/share/X11/xkb/keycodes/evdev> on a Linux system
    // to examine the mapping. FreeBSD and other unixes will use a different
    // set of keycode values.
    // We're using the symbolic names here to look up the keycodes that
    // correspond to the various key locations.
    for (name, phys) in &[
        ("ESC", PhysKeyCode::Escape),
        ("FK01", PhysKeyCode::F1),
        ("FK02", PhysKeyCode::F2),
        ("FK03", PhysKeyCode::F3),
        ("FK04", PhysKeyCode::F4),
        ("FK05", PhysKeyCode::F5),
        ("FK06", PhysKeyCode::F6),
        ("FK07", PhysKeyCode::F7),
        ("FK08", PhysKeyCode::F8),
        ("FK09", PhysKeyCode::F9),
        ("FK10", PhysKeyCode::F10),
        ("FK11", PhysKeyCode::F11),
        ("FK12", PhysKeyCode::F12),
        // ("PRSC", Print Screen),
        // ("SCLK", Scroll Lock),
        // ("PAUS", Pause),
        ("TLDE", PhysKeyCode::Grave),
        ("AE01", PhysKeyCode::K1),
        ("AE02", PhysKeyCode::K2),
        ("AE03", PhysKeyCode::K3),
        ("AE04", PhysKeyCode::K4),
        ("AE05", PhysKeyCode::K5),
        ("AE06", PhysKeyCode::K6),
        ("AE07", PhysKeyCode::K7),
        ("AE08", PhysKeyCode::K8),
        ("AE09", PhysKeyCode::K9),
        ("AE10", PhysKeyCode::K0),
        ("AE11", PhysKeyCode::Minus),
        ("AE12", PhysKeyCode::Equal),
        ("BKSL", PhysKeyCode::Backslash),
        ("BKSP", PhysKeyCode::Backspace),
        ("INS", PhysKeyCode::Insert),
        ("HOME", PhysKeyCode::Home),
        ("PGUP", PhysKeyCode::PageUp),
        ("NMLK", PhysKeyCode::NumLock),
        ("KPDV", PhysKeyCode::KeypadDivide),
        ("KPMU", PhysKeyCode::KeypadMultiply),
        ("KPSU", PhysKeyCode::KeypadSubtract),
        ("TAB", PhysKeyCode::Tab),
        ("AD01", PhysKeyCode::Q),
        ("AD02", PhysKeyCode::W),
        ("AD03", PhysKeyCode::E),
        ("AD04", PhysKeyCode::R),
        ("AD05", PhysKeyCode::T),
        ("AD06", PhysKeyCode::Y),
        ("AD07", PhysKeyCode::U),
        ("AD08", PhysKeyCode::I),
        ("AD09", PhysKeyCode::O),
        ("AD10", PhysKeyCode::P),
        ("AD11", PhysKeyCode::LeftBracket),
        ("AD12", PhysKeyCode::RightBracket),
        ("DELE", PhysKeyCode::Delete),
        ("END", PhysKeyCode::End),
        ("PGDN", PhysKeyCode::PageDown),
        ("KP7", PhysKeyCode::Keypad7),
        ("KP8", PhysKeyCode::Keypad8),
        ("KP9", PhysKeyCode::Keypad9),
        ("KPAD", PhysKeyCode::KeypadAdd),
        ("CAPS", PhysKeyCode::CapsLock),
        ("AC01", PhysKeyCode::A),
        ("AC02", PhysKeyCode::S),
        ("AC03", PhysKeyCode::D),
        ("AC04", PhysKeyCode::F),
        ("AC05", PhysKeyCode::G),
        ("AC06", PhysKeyCode::H),
        ("AC07", PhysKeyCode::J),
        ("AC08", PhysKeyCode::K),
        ("AC09", PhysKeyCode::L),
        ("AC10", PhysKeyCode::Semicolon),
        ("AC11", PhysKeyCode::Quote),
        ("RTRN", PhysKeyCode::Return),
        ("KP4", PhysKeyCode::Keypad4),
        ("KP5", PhysKeyCode::Keypad5),
        ("KP6", PhysKeyCode::Keypad6),
        ("LFSH", PhysKeyCode::LeftShift),
        ("AB01", PhysKeyCode::Z),
        ("AB02", PhysKeyCode::X),
        ("AB03", PhysKeyCode::C),
        ("AB04", PhysKeyCode::V),
        ("AB05", PhysKeyCode::B),
        ("AB06", PhysKeyCode::N),
        ("AB07", PhysKeyCode::M),
        ("AB08", PhysKeyCode::Comma),
        ("AB09", PhysKeyCode::Period),
        ("AB10", PhysKeyCode::Slash),
        ("RTSH", PhysKeyCode::RightShift),
        ("UP", PhysKeyCode::UpArrow),
        ("KP1", PhysKeyCode::Keypad1),
        ("KP2", PhysKeyCode::Keypad2),
        ("KP3", PhysKeyCode::Keypad3),
        ("KPEN", PhysKeyCode::KeypadEnter),
        ("LCTL", PhysKeyCode::LeftControl),
        ("LALT", PhysKeyCode::LeftAlt),
        ("SPCE", PhysKeyCode::Space),
        ("RALT", PhysKeyCode::RightAlt),
        ("RCTL", PhysKeyCode::RightControl),
        ("LEFT", PhysKeyCode::LeftArrow),
        ("DOWN", PhysKeyCode::DownArrow),
        ("RGHT", PhysKeyCode::RightArrow),
        ("KP0", PhysKeyCode::Keypad0),
        ("KPDL", PhysKeyCode::KeypadDelete),
        ("LWIN", PhysKeyCode::LeftWindows),
        ("RWIN", PhysKeyCode::RightWindows),
        ("MUTE", PhysKeyCode::VolumeMute),
        ("VOL-", PhysKeyCode::VolumeDown),
        ("VOL+", PhysKeyCode::VolumeUp),
        ("HELP", PhysKeyCode::Help),
    ] {
        if let Some(code) = keymap.key_by_name(name) {
            map.insert(code, *phys);
        }
    }

    map
}
