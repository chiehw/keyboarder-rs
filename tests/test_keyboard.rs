use keyboarder::{
    connection::ConnectionOps,
    platform_impl::{Connection, Simulator},
    simulate::Simulate,
    types::{Modifiers, PhysKeyCode},
};

#[test]
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
