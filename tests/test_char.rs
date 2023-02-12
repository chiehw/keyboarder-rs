use event::*;
use keyboarder::event::{self, KeyboardEvent};
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
