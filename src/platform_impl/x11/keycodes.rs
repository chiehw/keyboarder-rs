use std::collections::HashMap;

use xkbcommon::xkb;

use crate::types::PhysKeyCode;

pub fn build_phys_keycode_map(
    keymap: &xkb::Keymap,
) -> (
    HashMap<xkb::Keycode, PhysKeyCode>,
    HashMap<PhysKeyCode, xkb::Keycode>,
) {
    let mut phys_code_map = HashMap::new();
    let mut code_phys_map = HashMap::new();

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
        ("PRSC", PhysKeyCode::PrintScreen),
        ("SCLK", PhysKeyCode::ScrollLock),
        ("PAUS", PhysKeyCode::Pause),
        ("TLDE", PhysKeyCode::BackQuote),
        ("AE01", PhysKeyCode::Num1),
        ("AE02", PhysKeyCode::Num2),
        ("AE03", PhysKeyCode::Num3),
        ("AE04", PhysKeyCode::Num4),
        ("AE05", PhysKeyCode::Num5),
        ("AE06", PhysKeyCode::Num6),
        ("AE07", PhysKeyCode::Num7),
        ("AE08", PhysKeyCode::Num8),
        ("AE09", PhysKeyCode::Num9),
        ("AE10", PhysKeyCode::Num0),
        ("AE11", PhysKeyCode::Minus),
        ("AE12", PhysKeyCode::Equal),
        ("BKSL", PhysKeyCode::BackSlash),
        ("BKSP", PhysKeyCode::Backspace),
        ("INS", PhysKeyCode::Insert),
        ("HOME", PhysKeyCode::Home),
        ("PGUP", PhysKeyCode::PageUp),
        ("NMLK", PhysKeyCode::NumLock),
        ("KPDV", PhysKeyCode::KpDivide),
        ("KPMU", PhysKeyCode::KpMultiply),
        ("KPSU", PhysKeyCode::KpMinus),
        ("TAB", PhysKeyCode::Tab),
        ("AD01", PhysKeyCode::KeyQ),
        ("AD02", PhysKeyCode::KeyW),
        ("AD03", PhysKeyCode::KeyE),
        ("AD04", PhysKeyCode::KeyR),
        ("AD05", PhysKeyCode::KeyT),
        ("AD06", PhysKeyCode::KeyY),
        ("AD07", PhysKeyCode::KeyU),
        ("AD08", PhysKeyCode::KeyI),
        ("AD09", PhysKeyCode::KeyO),
        ("AD10", PhysKeyCode::KeyP),
        ("AD11", PhysKeyCode::LeftBracket),
        ("AD12", PhysKeyCode::RightBracket),
        ("DELE", PhysKeyCode::Delete),
        ("END", PhysKeyCode::End),
        ("PGDN", PhysKeyCode::PageDown),
        ("KP7", PhysKeyCode::Kp7),
        ("KP8", PhysKeyCode::Kp8),
        ("KP9", PhysKeyCode::Kp9),
        ("KPAD", PhysKeyCode::KpPlus),
        ("CAPS", PhysKeyCode::CapsLock),
        ("AC01", PhysKeyCode::KeyA),
        ("AC02", PhysKeyCode::KeyS),
        ("AC03", PhysKeyCode::KeyD),
        ("AC04", PhysKeyCode::KeyF),
        ("AC05", PhysKeyCode::KeyG),
        ("AC06", PhysKeyCode::KeyH),
        ("AC07", PhysKeyCode::KeyJ),
        ("AC08", PhysKeyCode::KeyK),
        ("AC09", PhysKeyCode::KeyL),
        ("AC10", PhysKeyCode::SemiColon),
        ("AC11", PhysKeyCode::Quote),
        ("RTRN", PhysKeyCode::Return),
        ("KP4", PhysKeyCode::Kp4),
        ("KP5", PhysKeyCode::Kp5),
        ("KP6", PhysKeyCode::Kp6),
        ("LFSH", PhysKeyCode::ShiftLeft),
        ("AB01", PhysKeyCode::KeyZ),
        ("AB02", PhysKeyCode::KeyX),
        ("AB03", PhysKeyCode::KeyC),
        ("AB04", PhysKeyCode::KeyV),
        ("AB05", PhysKeyCode::KeyB),
        ("AB06", PhysKeyCode::KeyN),
        ("AB07", PhysKeyCode::KeyM),
        ("AB08", PhysKeyCode::Comma),
        ("AB09", PhysKeyCode::Dot),
        ("AB10", PhysKeyCode::Slash),
        ("RTSH", PhysKeyCode::ShiftRight),
        ("UP", PhysKeyCode::UpArrow),
        ("KP1", PhysKeyCode::Kp1),
        ("KP2", PhysKeyCode::Kp2),
        ("KP3", PhysKeyCode::Kp3),
        ("KPEN", PhysKeyCode::KpEnter),
        ("LCTL", PhysKeyCode::ControlLeft),
        ("LALT", PhysKeyCode::AltLeft),
        ("SPCE", PhysKeyCode::Space),
        ("RALT", PhysKeyCode::AltRight),
        ("RCTL", PhysKeyCode::ControlRight),
        ("LEFT", PhysKeyCode::LeftArrow),
        ("DOWN", PhysKeyCode::DownArrow),
        ("RGHT", PhysKeyCode::RightArrow),
        ("KP0", PhysKeyCode::Kp0),
        ("KPDL", PhysKeyCode::KpDelete),
        ("LWIN", PhysKeyCode::MetaLeft),
        ("RWIN", PhysKeyCode::MetaRight),
        ("MUTE", PhysKeyCode::VolumeMute),
        ("VOL-", PhysKeyCode::VolumeDown),
        ("VOL+", PhysKeyCode::VolumeUp),
        ("HELP", PhysKeyCode::Help),
    ] {
        if let Some(code) = keymap.key_by_name(name) {
            code_phys_map.insert(code, *phys);
            phys_code_map.insert(*phys, code);
        }
    }

    (code_phys_map, phys_code_map)
}
