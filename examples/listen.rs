#[cfg(target_os = "windows")]
use keyboarder::platform_impl::Listener;
use keyboarder::types::KeyEvent;

fn dbg_event(key_event: &KeyEvent) -> anyhow::Result<()> {
    dbg!(key_event);
    Ok(())
}

fn main() -> anyhow::Result<()> {
    env_logger::init();
    std::env::set_var("DISPLAY", ":0");
    std::env::set_var("RUST_LOG", "trace");
    #[cfg(target_os = "windows")]
    let mut listener = Listener::new()?;
    #[cfg(target_os = "windows")]
    listener.run_loop(dbg_event)?;

    Ok(())
}
