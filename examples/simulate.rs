use keyboarder::platform_impl::{XConnection, XSimulator};

fn main() -> anyhow::Result<()> {
    env_logger::init();
    std::env::set_var("DISPLAY", ":0");

    let conn = XConnection::create_new()?;
    let mut simulator = XSimulator::new(&conn);

    simulator.simulate_char_without_modifiers('¹');
    simulator.simulate_char_without_modifiers('¡');

    // todo:
    // keyevent(keycode, char);
    // test char event map
    // test modifiers for altgr, to string

    Ok(())
}
