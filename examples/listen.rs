use keyboarder::platform_impl::Listener;

fn main() -> anyhow::Result<()> {
    env_logger::init();
    std::env::set_var("DISPLAY", ":0");
    std::env::set_var("RUST_LOG", "trace");

    let mut listener = Listener::new()?;
    listener.run_loop()?;

    Ok(())
}
