use crate::{apply_overrides, get_override, research_assistant};

#[test]
fn replacing_override_updates_value() {
    let preset = research_assistant().with_override("budget", "100");
    assert_eq!(get_override(&preset, "budget"), Some("100"));

    let updated = apply_overrides(preset, vec![("budget".to_string(), "500".to_string())]);
    assert_eq!(get_override(&updated, "budget"), Some("500"));
    // Confirm only one entry for the key
    let count = updated
        .overrides
        .iter()
        .filter(|(k, _)| k == "budget")
        .count();
    assert_eq!(
        count, 1,
        "should have exactly one budget entry after replace"
    );
}

#[test]
fn with_override_builder_method() {
    let preset = research_assistant()
        .with_override("a", "1")
        .with_override("b", "2");
    assert_eq!(get_override(&preset, "a"), Some("1"));
    assert_eq!(get_override(&preset, "b"), Some("2"));
}
