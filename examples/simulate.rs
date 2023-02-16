use keyboarder::platform_impl::{XConnection, XSimulator};

fn main() -> anyhow::Result<()> {
    env_logger::init();
    std::env::set_var("DISPLAY", ":0");

    let conn = XConnection::create_new()?;
    let _simulator = XSimulator::new(&conn);

    // todo:
    // keyevent(keycode, char);
    // test char event map
    // test modifiers for altgr, to string

    Ok(())
}
