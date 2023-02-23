use keyboarder::{
    connection::ConnectionOps,
    platform_impl::{Connection, Simulator},
};

fn main() -> anyhow::Result<()> {
    env_logger::init();
    std::env::set_var("DISPLAY", ":0");

    let conn = Connection::init()?;
    let _simulator = Simulator::new(&conn);

    //     if let Err(err) = conn.run_message_loop() {
    //         log::error!("Failed to process xcb event: {:?}", err);
    //     };
    // });

    // todo:
    // keyevent(keycode, char);
    // test char event map
    // test modifiers for altgr, to string

    Ok(())
}
