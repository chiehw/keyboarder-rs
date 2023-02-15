use keyboarder::{
    event::{KeyCode, KeyEvent, KeyboardEvent},
    x11::{Modifiers, PhysKeyCode, XConnection, XSimulator},
};

fn main() -> anyhow::Result<()> {
    env_logger::init();
    std::env::set_var("DISPLAY", ":0");

    let conn = XConnection::create_new()?;
    let mut simulator = XSimulator::new(&conn);

    // shift + delete = 1 in French
    simulator.simulate_keyboard_event(&KeyboardEvent {
        key_event: KeyEvent {
            key: KeyCode::Physical(PhysKeyCode::Delete),
            press: true,
            modifiers: Modifiers::SHIFT,
        },
        pressed_keys: vec![],
    });
    simulator.simulate_keyboard_event(&KeyboardEvent {
        key_event: KeyEvent {
            key: KeyCode::Physical(PhysKeyCode::Delete),
            press: false,
            modifiers: Modifiers::SHIFT,
        },
        pressed_keys: vec![],
    });

    Ok(())
}
