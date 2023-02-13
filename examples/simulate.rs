use keyboarder::event::Event;

use keyboarder::event_handler::EventHandler;
#[warn(unused_imports)]
use keyboarder::x11::{build_physkeycode_map, query_lc_ctype};

use keyboarder::common::*;
use keyboarder::x11::Keyboard;
use std::sync::Mutex;
use tfc::Context;

lazy_static::lazy_static! {
    static ref KBD_CONTEXT: Mutex<Context> = Mutex::new(Context::new().expect("error"));
}

fn handle_events(
    handler: &mut EventHandler,
    // dispatcher: &mut ActionDispatcher,
    event: &Event,
) -> anyhow::Result<()> {
    let _actions = handler
        .on_event(event)
        .map_err(|e| anyhow!("Failed handling {event:?}:\n  {e:?}"))?;
    // for action in actions {
    //     dispatcher.on_action(action)?;
    // }
    Ok(())
}

fn main() -> anyhow::Result<()> {
    env_logger::init();
    std::env::set_var("DISPLAY", ":0");

    /// todo:
    /// 1. test modifiers
    /// 2. test get_key_modifiers
    use xcb;
    use xkbcommon::xkb;

    let (connection, _screen_num) =
        xcb::Connection::connect_with_xlib_display_and_extensions(&[xcb::Extension::Xkb], &[])?;

    let kbd = Keyboard::new(&connection)?;
    kbd.process_key_press_impl();

    let a = 0b10;
    println!("{:?}", a);
    Ok(())
}


