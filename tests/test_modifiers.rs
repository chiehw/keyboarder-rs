use keyboarder::types::Modifiers;

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
