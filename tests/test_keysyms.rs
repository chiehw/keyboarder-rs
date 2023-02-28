use keyboarder::keysyms::{char_to_keysym, CHAR_KEYSYM_MAP};

#[test]
fn test_keysyms() {
    // cargo test --package keyboarder --test test_keysyms -- test_keysyms --exact --nocapture
    assert_eq!(226, char_to_keysym('â'));

    let keysym = char_to_keysym('€');
    assert_eq!(keysym, 8364);

    let keysym = char_to_keysym('ü');
    assert_eq!(keysym, 252);
}
