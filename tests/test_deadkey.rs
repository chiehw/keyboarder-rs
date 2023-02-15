use event::*;
use keyboarder::{
    event::{self, KeyboardEvent},
    x11::{XConnection, XSimulator},
};

/// â
/// ^
#[test]
fn test_deadkey_ampersand() {
    env_logger::init();
    std::env::set_var("DISPLAY", ":0");

    let conn = XConnection::create_new().unwrap();

    let mut simulator = XSimulator::new(&conn);

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
