use keyboarder::keysyms::CHAR_KEYSYM_MAP;

#[test]
fn test_keysyms() {
    // cargo test --package keyboarder --test test_keysyms -- test_keysyms --exact --nocapture
    assert_eq!(
        Some(226),
        CHAR_KEYSYM_MAP.char_to_keysym.get(&('â' as u32)).copied()
    );
}
