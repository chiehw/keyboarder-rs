use keyboarder::platform_impl::Listener;

fn main() -> anyhow::Result<()> {
    env_logger::init();
    std::env::set_var("DISPLAY", ":0");
    std::env::set_var("RUST_LOG", "trace");

    let mut listener = Listener::new()?;
    dbg!("new");
    listener.run_loop()?;

    Ok(())

    // unsafe {
    //     let current_window_thread_id = GetWindowThreadProcessId(GetForegroundWindow(), null_mut());
    //     let _hkl = GetKeyboardLayout(current_window_thread_id);
    // }

    // Keyboard::new().get_current_modifiers();
}
