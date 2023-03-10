use std::collections::HashMap;

use crate::types::{PhysKeyCode, Scancode};

pub fn build_phys_keycode_map() -> (
    HashMap<Scancode, PhysKeyCode>,
    HashMap<PhysKeyCode, Scancode>,
) {
    let mut code_phys_map: HashMap<Scancode, PhysKeyCode> = HashMap::new();
    let mut phys_code_map: HashMap<PhysKeyCode, Scancode> = HashMap::new();

    // <https://github.com/fufesou/rdev/blob/master/src/windows/keycodes.rs>
    for (scan_code, phys) in &[
        (0x01, PhysKeyCode::Escape),
        (0x3B, PhysKeyCode::F1),
        (0x3C, PhysKeyCode::F2),
        (0x3D, PhysKeyCode::F3),
        (0x3E, PhysKeyCode::F4),
        (0x3F, PhysKeyCode::F5),
        (0x40, PhysKeyCode::F6),
        (0x41, PhysKeyCode::F7),
        (0x42, PhysKeyCode::F8),
        (0x43, PhysKeyCode::F9),
        (0x44, PhysKeyCode::F10),
        (0x57, PhysKeyCode::F11),
        (0x58, PhysKeyCode::F12),
        (0xE037, PhysKeyCode::PrintScreen),
        (0x46, PhysKeyCode::ScrollLock),
        (0x0000, PhysKeyCode::Pause),
        (0x29, PhysKeyCode::BackQuote),
        (0x02, PhysKeyCode::Num1),
        (0x03, PhysKeyCode::Num2),
        (0x04, PhysKeyCode::Num3),
        (0x05, PhysKeyCode::Num4),
        (0x06, PhysKeyCode::Num5),
        (0x07, PhysKeyCode::Num6),
        (0x08, PhysKeyCode::Num7),
        (0x09, PhysKeyCode::Num8),
        (0x0A, PhysKeyCode::Num9),
        (0x0B, PhysKeyCode::Num0),
        (0x0C, PhysKeyCode::Minus),
        (0x0D, PhysKeyCode::Equal),
        (0x2B, PhysKeyCode::BackSlash),
        (0x0E, PhysKeyCode::Backspace),
        (0xE052, PhysKeyCode::Insert),
        (0xE047, PhysKeyCode::Home),
        (0xE049, PhysKeyCode::PageUp),
        (0x45, PhysKeyCode::NumLock),
        (0xE035, PhysKeyCode::KpDivide),
        (0x37, PhysKeyCode::KpMultiply),
        (0x4A, PhysKeyCode::KpMinus),
        (0x0F, PhysKeyCode::Tab),
        (0x10, PhysKeyCode::KeyQ),
        (0x11, PhysKeyCode::KeyW),
        (0x12, PhysKeyCode::KeyE),
        (0x13, PhysKeyCode::KeyR),
        (0x14, PhysKeyCode::KeyT),
        (0x15, PhysKeyCode::KeyY),
        (0x16, PhysKeyCode::KeyU),
        (0x17, PhysKeyCode::KeyI),
        (0x18, PhysKeyCode::KeyO),
        (0x19, PhysKeyCode::KeyP),
        (0x1A, PhysKeyCode::LeftBracket),
        (0x1B, PhysKeyCode::RightBracket),
        (0xE053, PhysKeyCode::Delete),
        (0xE04F, PhysKeyCode::End),
        (0xE051, PhysKeyCode::PageDown),
        (0x47, PhysKeyCode::Kp7),
        (0x48, PhysKeyCode::Kp8),
        (0x49, PhysKeyCode::Kp9),
        (0x4E, PhysKeyCode::KpPlus),
        (0x3A, PhysKeyCode::CapsLock),
        (0x1E, PhysKeyCode::KeyA),
        (0x1F, PhysKeyCode::KeyS),
        (0x20, PhysKeyCode::KeyD),
        (0x21, PhysKeyCode::KeyF),
        (0x22, PhysKeyCode::KeyG),
        (0x23, PhysKeyCode::KeyH),
        (0x24, PhysKeyCode::KeyJ),
        (0x25, PhysKeyCode::KeyK),
        (0x26, PhysKeyCode::KeyL),
        (0x27, PhysKeyCode::SemiColon),
        (0x28, PhysKeyCode::Quote),
        (0x1C, PhysKeyCode::Return),
        (0x4B, PhysKeyCode::Kp4),
        (0x4C, PhysKeyCode::Kp5),
        (0x4D, PhysKeyCode::Kp6),
        (0x2A, PhysKeyCode::ShiftLeft),
        (0x2C, PhysKeyCode::KeyZ),
        (0x2D, PhysKeyCode::KeyX),
        (0x2E, PhysKeyCode::KeyC),
        (0x2F, PhysKeyCode::KeyV),
        (0x30, PhysKeyCode::KeyB),
        (0x31, PhysKeyCode::KeyN),
        (0x32, PhysKeyCode::KeyM),
        (0x33, PhysKeyCode::Comma),
        (0x34, PhysKeyCode::Dot),
        (0x35, PhysKeyCode::Slash),
        (0x36, PhysKeyCode::ShiftRight),
        (0xE048, PhysKeyCode::UpArrow),
        (0x4F, PhysKeyCode::Kp1),
        (0x50, PhysKeyCode::Kp2),
        (0x51, PhysKeyCode::Kp3),
        (0xE01C, PhysKeyCode::KpReturn),
        (0x1D, PhysKeyCode::ControlLeft),
        (0x38, PhysKeyCode::AltLeft),
        (0x39, PhysKeyCode::Space),
        // FIXME: scan = 0x021d(541) | 0xE038(57400)
        (0xE038, PhysKeyCode::AltRight),
        (0xE01D, PhysKeyCode::ControlRight),
        (0xE04B, PhysKeyCode::LeftArrow),
        (0xE050, PhysKeyCode::DownArrow),
        (0xE04D, PhysKeyCode::RightArrow),
        (0x52, PhysKeyCode::Kp0),
        (0xE053, PhysKeyCode::KpDelete),
        (0xE05B, PhysKeyCode::MetaLeft),
        (0xE05C, PhysKeyCode::MetaRight),
        (0xE020, PhysKeyCode::VolumeMute),
        (0xE02E, PhysKeyCode::VolumeDown),
        (0xE030, PhysKeyCode::VolumeUp),
        (0xE05D, PhysKeyCode::Menu),
        (0x53, PhysKeyCode::KpDecimal),
        (0x0000, PhysKeyCode::Help), // todo
    ] {
        code_phys_map.insert(*scan_code, *phys);
        phys_code_map.insert(*phys, *scan_code);
    }

    (code_phys_map, phys_code_map)
}
