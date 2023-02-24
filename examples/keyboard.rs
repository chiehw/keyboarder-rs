use keyboarder::platform_impl::Keyboard;

fn main() -> anyhow::Result<()> {
    env_logger::init();
    std::env::set_var("DISPLAY", ":0");

    dbg!(Keyboard::create_new().has_alt_gr());

    Ok(())
}
