use keyboarder::{
    platform_impl::Simulator,
    simulate::Simulate,
    types::{KeyCode, KeyEvent, Modifiers, ServerMode, SimEvent},
};

fn main() -> anyhow::Result<()> {
    env_logger::init();
    std::env::set_var("DISPLAY", ":0");

    let key_event = KeyEvent {
        key: KeyCode::Char('1'),
        press: true,
        modifiers: Modifiers::NONE,
        raw_event: None,
    };

    Simulator::spawn_server(ServerMode::Translate)?;
    Simulator::event_to_server(&SimEvent::Simulate(key_event))?;

    Ok(())
}
