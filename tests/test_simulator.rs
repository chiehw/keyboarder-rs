use keyboarder::{
    connection::ConnectionOps,
    platform_impl::{Connection, Simulator},
    simulate::Simulate,
    types::{KeyCode, KeyEvent, Modifiers, PhysKeyCode},
};
/// 1
/// a
/// 你
#[test]
fn test_char_keycode() {
    std::env::set_var("DISPLAY", ":0");

    let conn = Connection::init().unwrap();

    let mut simulator = Simulator::new(&conn);

    // & in French, 1 in US
    simulator.simulate_keycode(10, true);
    simulator.simulate_keycode(10, false);

    // & in French
    simulator.simulate_keycode(10, true);
    simulator.simulate_keycode(10, false);

    simulator.simulate_char_without_modifiers('1', true);
    simulator.simulate_char_without_modifiers('1', false);
    simulator.simulate_char_without_modifiers('!', true);
    simulator.simulate_char_without_modifiers('!', false);

    simulator.simulate_char_without_modifiers('¹', true);
    simulator.simulate_char_without_modifiers('¹', false);
    simulator.simulate_char_without_modifiers('¡', true);
    simulator.simulate_char_without_modifiers('¡', false);
}

/// # char + AltGr/Shift
/// 1. shift + 1: 1(French), !(US)
/// 2. AltGr + a
#[test]
fn test_char_keycode_with_modifier() {
    std::env::set_var("DISPLAY", ":0");

    let conn = Connection::init().unwrap();
    let mut simulator = Simulator::new(&conn);

    // shift + & = 1 in French
    simulator.simulate_key_event(&KeyEvent {
        key: KeyCode::RawCode(10),
        press: true,
        modifiers: Modifiers::SHIFT,
        raw_event: None,
    });
    simulator.simulate_key_event(&KeyEvent {
        key: KeyCode::RawCode(10),
        press: false,
        modifiers: Modifiers::SHIFT,
        raw_event: None,
    });
}

#[test]
fn test_char_by_phys() {
    std::env::set_var("DISPLAY", ":0");

    let conn = Connection::init().unwrap();
    let mut simulator = Simulator::new(&conn);

    // shift + KeyQ = "Q" in French
    // shift + KeyA = "A" in US
    simulator.simulate_key_event(&KeyEvent {
        key: KeyCode::Physical(PhysKeyCode::KeyQ),
        press: true,
        modifiers: Modifiers::SHIFT,
        raw_event: None,
    });
    simulator.simulate_key_event(&KeyEvent {
        key: KeyCode::Physical(PhysKeyCode::KeyQ),
        press: false,
        modifiers: Modifiers::SHIFT,
        raw_event: None,
    });
}

#[test]
fn test_keysym() {
    std::env::set_var("DISPLAY", ":0");

    let conn = Connection::init().unwrap();

    let mut simulator = Simulator::new(&conn);

    // & in French, 1 in US
    simulator.simulate_phys(PhysKeyCode::ControlLeft, true);
    simulator.simulate_keysym(97, true);
    simulator.simulate_keysym(97, false);
    simulator.simulate_phys(PhysKeyCode::ControlLeft, false);
}


#[test]
fn test_simulate_get_modifier() {
    std::env::set_var("DISPLAY", ":0");

    let conn = Connection::init().unwrap();
    let mut simulator = Simulator::new(&conn);
    simulator.simulate_phys(PhysKeyCode::ControlLeft, true);
    assert_eq!(Modifiers::CTRL, simulator.get_current_modifiers());
    simulator.simulate_phys(PhysKeyCode::ControlLeft, false);

    simulator.simulate_phys(PhysKeyCode::ControlLeft, true);
    dbg!(simulator.get_current_modifiers());
    
    

}