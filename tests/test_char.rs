pub use event::*;
pub use hello_rust::event::{self, KeyboardEvent};
/// 1
/// a
/// 你
#[test]
fn test_single_char() {
    let _key_event_1 = KeyboardEvent::new(Code::KeySym(49), State::Press, vec![]);
    let _key_event_2 = KeyboardEvent::new(Code::KeySym(49), State::Release, vec![]);

    let _key_event_1 = KeyboardEvent::new(Code::KeySym(97), State::Press, vec![]);
    let _key_event_2 = KeyboardEvent::new(Code::KeySym(97), State::Release, vec![]);
}

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
