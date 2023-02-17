use keyboarder::{
    event::{KeyCode, KeyEvent},
    types::{Modifiers, PhysKeyCode},
    utils::KeyEventFile,
};

fn main() -> anyhow::Result<()> {
    env_logger::init();
    std::env::set_var("DISPLAY", ":0");

    let key_event = KeyEvent {
        key: KeyCode::Physical(PhysKeyCode::KeyQ),
        press: false,
        modifiers: Modifiers::SHIFT,
        click: false,
    };
    // write in [%date].kbd
    let mut key_event_file = KeyEventFile::create_new(&key_event)?;
    key_event_file.write()?;
    drop(key_event_file);

    // read in [%data].kbd
    let key_event_file = KeyEventFile::create_with_file_by_default()?;
    anyhow::ensure!(
        key_event_file.get_key_event()? == key_event,
        "key event is not equal"
    );

    Ok(())
}
