#[cfg(target_os = "windows")]
use keyboarder::platform_impl::Listener;
use keyboarder::types::KeyEvent;
use std::{io::Write, net::TcpStream};

fn send_key_event(key_event: &KeyEvent) -> anyhow::Result<()> {
    log::debug!("key_event: {:?}", key_event);
    
    let mut stream = TcpStream::connect(("192.168.59.128", 7878))?;
    let raw_data = key_event.to_u8_vec()?;

    stream.write_all(&raw_data)?;
    stream.flush()?;

    std::thread::sleep(std::time::Duration::from_millis(10));

    Ok(())
}

fn main() -> anyhow::Result<()> {
    env_logger::init();
    std::env::set_var("DISPLAY", ":0");
    std::env::set_var("RUST_LOG", "trace");

    #[cfg(target_os = "windows")]
    let mut listener = Listener::new()?;
    #[cfg(target_os = "windows")]
    listener.run_loop(send_key_event)?;

    Ok(())
}
