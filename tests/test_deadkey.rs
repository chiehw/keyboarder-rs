use keyboarder::{
    platform_impl::{Connection, Simulator},
    Simulate,
};

/// â
/// ^
#[test]
fn test_deadkey_ampersand() {
    std::env::set_var("DISPLAY", ":0");

    let conn = Connection::create_new().unwrap();

    let mut simulator = Simulator::new(&conn);

    // dead_circumflex(^) in French
    simulator.simulate_keysym(65106, true); // &
    simulator.simulate_keysym(65106, false); // &
}

/// # diacritic
/// â
/// 1. LeftBracket, Delete, Q
/// 2. LeftBracket, Q
/// 3. LeftBracket, Space
fn test_dead_with_control_key() {}
