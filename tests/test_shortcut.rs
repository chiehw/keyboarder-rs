pub use event::*;
pub use hello_rust::event::{self, KeyboardEvent};

// / # shortcut
// / 1. alt + o
// / 2. Ctrl + shift + L
// / 3. Shift + Del
#[test]
fn test_shortcut() {}
//  fn test_shortcut() {}
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
