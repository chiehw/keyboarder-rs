use keyboarder::{
    connection::ConnectionOps,
    platform_impl::{Connection, Simulator},
    simulate::Simulate,
    types::{KeyCode, KeyEvent, Modifiers},
};

// / # shortcut
// / 1. alt + o
// / 2. Ctrl + shift + L
// / 3. Shift + Del
#[test]
fn test_shortcut() {
    std::env::set_var("DISPLAY", ":0");

    let conn = Connection::init().unwrap();
    let mut simulator = Simulator::new(&conn);
    // shift + delete = 1 in French
    simulator.simulate_key_event(&KeyEvent {
        key: KeyCode::KeySym(49),
        press: true,
        modifiers: Modifiers::SHIFT,
        raw_event: None,
    });

    simulator.simulate_key_event(&KeyEvent {
        key: KeyCode::KeySym(49),
        press: false,
        modifiers: Modifiers::SHIFT,
        raw_event: None,
    });
}
