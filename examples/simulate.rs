use keyboarder::platform_impl::{Connection, Simulator};

fn main() -> anyhow::Result<()> {
    env_logger::init();
    std::env::set_var("DISPLAY", ":0");

    let conn = Connection::create_new()?;
    let _simulator = Simulator::new(&conn);

    // todo:
    // keyevent(keycode, char);
    // test char event map
    // test modifiers for altgr, to string

    Ok(())
}
