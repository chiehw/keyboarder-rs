use keyboarder::{
    event::{self, KeyCode, KeyEvent},
    x11::{Modifiers, PhysKeyCode, XConnection, XSimulator},
};
/// 1
/// a
/// 你
#[test]
fn test_char_keycode() {
    std::env::set_var("DISPLAY", ":0");

    let conn = XConnection::create_new().unwrap();

    let mut simulator = XSimulator::new(&conn);

    // & in French, 1 in US
    simulator.simulate_keycode(10, true);
    simulator.simulate_keycode(10, false);

    // & in French
    simulator.simulate_keycode(10, true);
    simulator.simulate_keycode(10, false);
}

/// # char + AltGr/Shift
/// 1. shift + 1: 1(French), !(US)
/// 2. AltGr + a
#[test]
fn test_char_keycode_with_modifier() {
    std::env::set_var("DISPLAY", ":0");

    let conn = XConnection::create_new().unwrap();
    let mut simulator = XSimulator::new(&conn);

    // shift + & = 1 in French
    simulator.simulate_key_event(&KeyEvent {
        key: KeyCode::RawCode(10),
        press: true,
        modifiers: Modifiers::SHIFT,
        click: false,
    });
    simulator.simulate_key_event(&KeyEvent {
        key: KeyCode::RawCode(10),
        press: false,
        modifiers: Modifiers::SHIFT,
        click: false,
    });
}

#[test]
fn test_char_by_phys() {
    std::env::set_var("DISPLAY", ":0");

    let conn = XConnection::create_new().unwrap();
    let mut simulator = XSimulator::new(&conn);

    // shift + KeyQ = "Q" in French
    // shift + KeyA = "A" in US
    simulator.simulate_key_event(&KeyEvent {
        key: KeyCode::Physical(PhysKeyCode::KeyQ),
        press: true,
        modifiers: Modifiers::SHIFT,
        click: false,
    });
    simulator.simulate_key_event(&KeyEvent {
        key: KeyCode::Physical(PhysKeyCode::KeyQ),
        press: false,
        modifiers: Modifiers::SHIFT,
        click: false,
    });
}
