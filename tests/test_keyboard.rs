use std::borrow::Borrow;

use keyboarder::{
    connection::ConnectionOps,
    platform_impl::{Connection, Simulator},
    simulate::Simulate,
    types::{KeyCode, KeyEvent, Modifiers, PhysKeyCode},
};

#[test]
#[cfg(target_os = "linux")]
fn test_kbd_char_keysym() {
    env_logger::init();
    std::env::set_var("DISPLAY", ":0");

    let conn = Connection::init().unwrap();
    let mut simulator = Simulator::new(&conn);

    let state = conn.keyboard.state.borrow();
    let chr = '1' as u32;
    dbg!(chr);
    // 65106 -> "^": translate dead key to char.
    let res = unsafe { xkbcommon::xkb::ffi::xkb_keysym_to_utf32(chr) };
    dbg!(res);
}

#[test]
#[cfg(target_os = "linux")]
fn test_kbd_keysym_map() {
    env_logger::init();
    std::env::set_var("DISPLAY", ":0");

    let conn = Connection::init().unwrap();

    let kbd = conn.keyboard.borrow();
    let keysym_map = kbd.keysym_map.borrow();

    let keycode = keysym_map.get(&49);
    assert_eq!(keycode, Some(&10));
}

#[test]
#[cfg(target_os = "linux")]
fn test_keyboard_when_simulate() {
    env_logger::init();
    std::env::set_var("DISPLAY", ":0");

    let conn = Connection::init().unwrap();
    let mut simulator = Simulator::new(&conn);

    simulator.simulate_phys(PhysKeyCode::ShiftLeft, true);
    simulator.simulate_phys(PhysKeyCode::ControlLeft, true);
    simulator.simulate_phys(PhysKeyCode::AltLeft, true);
    simulator.simulate_phys(PhysKeyCode::KeyQ, true);

    assert_eq!(
        // keyboard.get_current_modifiers will not update when simulate
        simulator.get_current_modifiers(),
        Modifiers::SHIFT | Modifiers::CTRL | Modifiers::ALT
    );

    simulator.simulate_phys(PhysKeyCode::KeyQ, false);
    simulator.simulate_phys(PhysKeyCode::AltLeft, false);
    simulator.simulate_phys(PhysKeyCode::ControlLeft, false);
    simulator.simulate_phys(PhysKeyCode::ShiftLeft, false);
}

#[test]
#[cfg(target_os = "linux")]
fn test_keyboard_altgr_when_simulate() {
    // test it in French keyboard.
    env_logger::init();
    std::env::set_var("DISPLAY", ":0");

    let conn = Connection::init().unwrap();
    let mut simulator = Simulator::new(&conn);

    simulator.simulate_phys(PhysKeyCode::AltRight, true);
    simulator.simulate_phys(PhysKeyCode::KeyQ, true);

    assert_eq!(simulator.get_current_modifiers(), Modifiers::ALT_GR);

    simulator.simulate_phys(PhysKeyCode::KeyQ, false);
    simulator.simulate_phys(PhysKeyCode::AltRight, false);
}

#[test]
#[cfg(target_os = "linux")]
fn test_keyboard_event_by_char() {
    env_logger::init();
    std::env::set_var("DISPLAY", ":0");

    let conn = Connection::init().unwrap();

    let kbd = conn.keyboard.borrow();

    assert_eq!(
        Some(KeyEvent {
            key: KeyCode::RawCode(10,),
            press: false,
            modifiers: Modifiers::SHIFT,
            raw_event: None,
        }),
        kbd.get_key_event_by_char('!')
    );
}

#[test]
#[cfg(target_os = "linux")]
fn test_keyboard_keycode() {
    env_logger::init();
    std::env::set_var("DISPLAY", ":0");

    let conn = Connection::init().unwrap();

    let kbd = conn.keyboard.borrow();

    let code = kbd.get_keycode_by_phys(PhysKeyCode::KpDelete);
    assert_eq!(code, Some(119))
}
