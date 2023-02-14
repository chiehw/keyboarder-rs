use keyboarder::x11::{XConnection, XSimulator};
use std::rc::Rc;

fn main() -> anyhow::Result<()> {
    env_logger::init();
    std::env::set_var("DISPLAY", ":0");

    let conn = XConnection::create_new()?;
    let simulator = XSimulator::new(&conn);
    simulator.simulate_keycode(10, true);
    simulator.simulate_keycode(10, false);

    Ok(())
}
