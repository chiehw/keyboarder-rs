use keyboarder::{
    platform_impl::Simulator,
    simulate::Simulate,
    types::{KeyCode, KeyEvent, Modifiers, PhysKeyCode, ServerMode},
};

fn main() -> anyhow::Result<()> {
    env_logger::init();
    std::env::set_var("DISPLAY", ":0");

    let key_event = KeyEvent {
        key: KeyCode::Physical(PhysKeyCode::KeyQ),
        press: true,
        modifiers: Modifiers::SHIFT,
        raw_event: None,
    };

    let handle = Simulator::spawn_server(ServerMode::Map)?;
    Simulator::event_to_server(&key_event)?;

    handle.join().unwrap();

    Ok(())
}
