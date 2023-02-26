use keyboarder::types::{KeyCode, KeyEvent, Modifiers, PhysKeyCode};

#[test]
fn test_mod_try_from() {
    let mods = Modifiers::try_from("SHIFT".to_string()).unwrap();
    assert_eq!(mods, Modifiers::SHIFT, "modifier try from error");

    let mods = Modifiers::try_from("SHIFT | ALT".to_string()).unwrap();
    assert_eq!(
        mods,
        Modifiers::SHIFT | Modifiers::ALT,
        "modifier try from error"
    );
}

#[test]
fn test_diff_mod() {
    let modifiers = Modifiers::NONE;
    let target_mod = Modifiers::SHIFT;

    let v = modifiers.diff_modifiers(&target_mod);

    assert_eq!(
        v,
        [KeyEvent {
            key: KeyCode::Physical(PhysKeyCode::ShiftLeft,),
            press: true,
            modifiers: Modifiers::NONE,
            raw_event: None,
        },]
    );
}
#[test]
fn test_trans_mod() {
    let modifiers = Modifiers::LEFT_CTRL;

    let v = modifiers.trans_positional_mods();

    assert_eq!(v, Modifiers::CTRL);
}
