fn main() -> anyhow::Result<()> {
    env_logger::init();
    std::env::set_var("DISPLAY", ":0");

    // let kbd = Keyboard::create_new()?;

    Ok(())
}
