use crate::event::{Event, KeyboardEvent};
#[warn(unused_imports)]
use common::*;
use hello_rust::{
    event::{Code, State},
    event_handler::EventHandler,
    *,
};

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
    /// 1. convert modifier state to modifier
    /// 2. modifier to action.
    /// 3. sync keyboard state.
    // 49 1: dead key
    use rdev::Key;
    let key_event_down = KeyboardEvent::new(Code::KeySym(49), State::Press, vec![Key::ShiftLeft]);
    let key_event_up = KeyboardEvent::new(Code::KeySym(49), State::Release, vec![Key::ShiftLeft]);

    let mut handler = EventHandler::new()?;
    handle_events(&mut handler, &Event::KeyboardEvent(key_event_down))?;
    handle_events(&mut handler, &Event::KeyboardEvent(key_event_up))?;

    handler.check_modifiers();

    Ok(())
}
