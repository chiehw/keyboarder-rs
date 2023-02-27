#[test]
fn test_keysyms() {
    // cargo test --package keyboarder --test test_keysyms -- test_keysyms --exact --nocapture
    println!("\\u{:04x} \u{037A}", '^' as u32);
    println!("\\u{:04} \u{037A}", 'â' as u32);
}
