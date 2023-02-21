use keyboarder::{platform_impl::Simulator, simulate::Simulate, types::SimulateEvent};

fn main() -> anyhow::Result<()> {
    env_logger::init();
    std::env::set_var("DISPLAY", ":0");

    let handle = Simulator::spawn_server()?;
    Simulator::simulate_event_to_server(SimulateEvent::CharNoModifi('a'))?;

    // if let Err(err) = conn.run_message_loop() {
    //     log::error!("Failed to process xcb event: {:?}", err);
    // };

    // todo:
    // update keymap.

    handle.join().unwrap();

    Ok(())
}
