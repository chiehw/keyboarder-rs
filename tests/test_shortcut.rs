use event::*;
use keyboarder::{
    event::{self},
    platform_impl::{Modifiers, XConnection, XSimulator},
};

// / # shortcut
// / 1. alt + o
// / 2. Ctrl + shift + L
// / 3. Shift + Del
#[test]
fn test_shortcut() {
    std::env::set_var("DISPLAY", ":0");

    let conn = XConnection::create_new().unwrap();
    let mut simulator = XSimulator::new(&conn);
    // shift + delete = 1 in French
    simulator.simulate_key_event(&KeyEvent {
        key: KeyCode::KeySym(49),
        press: true,
        modifiers: Modifiers::SHIFT,
        click: false,
    });
    simulator.simulate_key_event(&KeyEvent {
        key: KeyCode::KeySym(49),
        press: false,
        modifiers: Modifiers::SHIFT,
        click: false,
    });
}
