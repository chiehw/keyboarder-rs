#[cfg(target_os = "win")]
use keyboarder::platform_impl::Listener;

fn main() -> anyhow::Result<()> {
    env_logger::init();
    std::env::set_var("DISPLAY", ":0");
    std::env::set_var("RUST_LOG", "trace");
    #[cfg(target_os = "win")]
    let mut listener = Listener::new()?;
    #[cfg(target_os = "win")]
    listener.run_loop()?;

    Ok(())
}
