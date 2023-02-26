use keyboarder::{
    connection::ConnectionOps,
    platform_impl::{Connection, Simulator},
    simulate::Simulate,
};

fn main() -> anyhow::Result<()> {
    env_logger::init();
    std::env::set_var("DISPLAY", ":0");

    let conn = Connection::init()?;
    let mut simulator = Simulator::new(&conn);

    // & in French
    simulator.simulate_keycode(10, true);
    simulator.simulate_keycode(10, false);

    simulator.simulate_char_without_modifiers('1');
    simulator.simulate_char_without_modifiers('1');
    simulator.simulate_char_without_modifiers('!');
    simulator.simulate_char_without_modifiers('!');

    simulator.simulate_char_without_modifiers('¹');
    simulator.simulate_char_without_modifiers('¹');
    simulator.simulate_char_without_modifiers('¡');
    simulator.simulate_char_without_modifiers('¡');

    Ok(())
}
