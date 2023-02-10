use crate::event::{Event, KeyboardEvent};
#[warn(unused_imports)]
use common::*;
use hello_rust::{
    action_dispatcher::ActionDispatcher,
    event::{Code, State},
    event_handler::EventHandler,
    *,
};
use nix::sys::timerfd::{ClockId, TimerFd, TimerFlags};
use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};
use tfc::{traits::*, Context, Key as FatKey};

lazy_static::lazy_static! {
    static ref KBD_CONTEXT: Mutex<Context> = Mutex::new(Context::new().expect("error"));
}

// /// # single char
// /// 1. a
// /// 2. â
// /// 3. 你
// fn test_single_char() {
//     let key_event_1 = KeyboardEvent {
//         modifiers: vec![],
//         code: ' ' as u32,
//         state: true,
//     };
//     let key_event_2 = KeyboardEvent {
//         state: false,
//         modifiers: vec![],
//         code: ' ' as u32,
//     };

//     {
//         // test case 1: a
//         let key_event_1 = KeyboardEvent {
//             code: 'a' as u32,
//             ..key_event_1.clone()
//         };
//         let key_event_2 = KeyboardEvent {
//             code: 'a' as u32,
//             ..key_event_2.clone()
//         };

//         simulate(&key_event_1);
//         simulate(&key_event_2);
//     }

//     {
//         // test case 2: â
//         let key_event_1 = KeyboardEvent {
//             code: 'â' as u32,
//             ..key_event_1.clone()
//         };
//         let key_event_2 = KeyboardEvent {
//             code: 'â' as u32,
//             ..key_event_2.clone()
//         };
//         simulate(&key_event_1);
//         simulate(&key_event_2);
//     }

//     {
//         // test case 3: 你
//         let key_event_1 = KeyboardEvent {
//             code: '你' as u32,
//             ..key_event_1.clone()
//         };
//         let key_event_2 = KeyboardEvent {
//             code: '你' as u32,
//             ..key_event_2.clone()
//         };
//         simulate(&key_event_1);
//         simulate(&key_event_2);
//     }
// }

// /// # char + AltGr/Shift
// /// 1. shift + a
// /// 2. AltGr + a
// ///
// fn test_char_shift() {
//     let key_event_1 = KeyboardEvent {
//         state: true,
//         modifiers: vec![],
//         code: ' ' as u32,
//     };
//     let key_event_2 = KeyboardEvent {
//         state: false,
//         modifiers: vec![],
//         code: ' ' as u32,
//     };

//     {
//         // test case 1: shift + a => A
//         let key_event_1 = KeyboardEvent {
//             code: 'a' as u32,
//             ..key_event_1.clone()
//         };
//         let key_event_2 = KeyboardEvent {
//             code: 'a' as u32,
//             ..key_event_2.clone()
//         };
//         KBD_CONTEXT.lock().unwrap().key_down(FatKey::Shift);
//         simulate(&key_event_1);
//         simulate(&key_event_2);
//         KBD_CONTEXT.lock().unwrap().key_up(FatKey::Shift);
//     }

//     {
//         // test case 1: shift + ^(fr), a => ä（加号表示同时, 逗号表示下一次按下）
//         let key_event_1 = KeyboardEvent {
//             code: 'a' as u32,
//             ..key_event_1.clone()
//         };
//         let key_event_2 = KeyboardEvent {
//             code: 'a' as u32,
//             ..key_event_2.clone()
//         };
//         KBD_CONTEXT.lock().unwrap().key_down(FatKey::Shift);
//         simulate(&key_event_1);
//         simulate(&key_event_2);
//         KBD_CONTEXT.lock().unwrap().key_up(FatKey::Shift);
//     }
// }

// /// # diacritic
// /// â
// /// 1. LeftBracket, Delete, Q
// /// 2. LeftBracket, Q
// /// 3. LeftBracket, Space
// fn test_diacritic() {}

/// # shortcut
/// 1. alt + o
/// 2. Ctrl + shift + L
/// 3. Shift + Del
///
///
// fn test_shortcut() {}

// fn simulate(key_event: &KeyboardEvent) {
//     let code = key_event.code;
//     if let Some(chr) = std::char::from_u32(code) {
//         match key_event.code_state {
//             true => {
//                 KBD_CONTEXT.lock().unwrap().unicode_char_down(chr);
//             }
//             false => {
//                 KBD_CONTEXT.lock().unwrap().unicode_char_up(chr);
//             }
//         }
//     }
// }

fn handle_events(
    handler: &mut EventHandler,
    // dispatcher: &mut ActionDispatcher,
    event: &Event,
) -> anyhow::Result<()> {
    let actions = handler
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
    // test_single_char();
    // test_char_shift();

    // // 65106 ^: dead key
    // KeyboardEvent {
    //     down: true,
    //     modifiers: vec![],
    //     code: 65106u32,
    // }
    // .simulate();
    // KeyboardEvent {
    //     down: false,
    //     modifiers: vec![],
    //     code: 65106u32,
    // }
    // .simulate();

    // // 65514: Alt_R
    // KeyboardEvent {
    //     down: true,
    //     modifiers: vec![],
    //     code: 65514u32,
    // }
    // .simulate();
    // KeyboardEvent {
    //     down: false,
    //     modifiers: vec![],
    //     code: 65514u32,
    // }
    // .simulate();

    // // 65027: XK_ISO_Level3_Shift(AltGr)
    // KeyboardEvent {
    //     down: true,
    //     modifiers: vec![],
    //     code: 65027u32,
    // }
    // .simulate();
    // KeyboardEvent {
    //     down: false,
    //     modifiers: vec![],
    //     code: 65027u32,
    // }
    // .simulate();

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
