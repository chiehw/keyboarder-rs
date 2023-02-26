use keyboarder::{
    connection::ConnectionOps,
    platform_impl::{Connection, Simulator},
    simulate::Simulate,
};

/// â
/// ^
#[test]
fn test_deadkey_ampersand() {
    std::env::set_var("DISPLAY", ":0");

    let conn = Connection::init().unwrap();

    let mut simulator = Simulator::new(&conn);

    // dead_circumflex(^) in French
    // XK_acircumflex = 0x00e2 + a
    simulator.simulate_keysym(65106, true); // &
    simulator.simulate_keysym(65106, false); // &
}

/// # diacritic
/// â
/// 1. LeftBracket, Delete, Q
/// 2. LeftBracket, Q
/// 3. LeftBracket, Space
fn test_dead_with_control_key() {}
