use event::*;
use keyboarder::event::{self, KeyboardEvent};

/// â
/// ^
#[test]
fn test_dead_key() {
    let _key_event_1 = KeyboardEvent::new(Code::KeySym(226), State::Press, vec![]);
    let _key_event_2 = KeyboardEvent::new(Code::KeySym(226), State::Release, vec![]);

    let _key_event_1 = KeyboardEvent::new(Code::KeySym(65106), State::Press, vec![]);
    let _key_event_2 = KeyboardEvent::new(Code::KeySym(65106), State::Release, vec![]);
}

/// # diacritic
/// â
/// 1. LeftBracket, Delete, Q
/// 2. LeftBracket, Q
/// 3. LeftBracket, Space
fn test_dead_with_control_key() {}
