use keyboarder::types::{KeyCode, KeyEvent, Modifiers, PhysKeyCode};
use std::{io::Write, net::TcpStream};

fn send_key_event(key_event: &KeyEvent) -> anyhow::Result<()> {
    let mut stream = TcpStream::connect(("127.0.0.1", 7878))?;
    let raw_data = key_event.to_u8_vec()?;

    stream.write_all(&raw_data)?;
    stream.flush()?;

    std::thread::sleep(std::time::Duration::from_millis(10));

    Ok(())
}

fn simulate(phys: PhysKeyCode, press: bool) -> anyhow::Result<()> {
    let key_event = KeyEvent {
        key: KeyCode::Physical(phys),
        press,
        modifiers: Modifiers::NONE,
        raw_event: None,
    };
    send_key_event(&key_event)?;

    Ok(())
}

fn main() -> anyhow::Result<()> {
    simulate(PhysKeyCode::ShiftLeft, true)?;
    simulate(PhysKeyCode::KeyQ, true)?;

    simulate(PhysKeyCode::KeyQ, false)?;
    simulate(PhysKeyCode::ShiftLeft, false)?;

    Ok(())
}
