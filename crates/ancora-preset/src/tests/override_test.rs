use crate::{apply_overrides, get_override, research_assistant};

#[test]
fn override_applies_new_key() {
    let preset = research_assistant();
    let modified = apply_overrides(
        preset,
        vec![("max_citations".to_string(), "50".to_string())],
    );
    assert_eq!(get_override(&modified, "max_citations"), Some("50"));
}

#[test]
fn override_replaces_existing_key() {
    let preset = research_assistant()
        .with_override("max_citations", "10");
    let modified = apply_overrides(
        preset,
        vec![("max_citations".to_string(), "100".to_string())],
    );
    assert_eq!(get_override(&modified, "max_citations"), Some("100"));
}

#[test]
fn override_missing_key_returns_none() {
    let preset = research_assistant();
    assert_eq!(get_override(&preset, "nonexistent"), None);
}

#[test]
fn override_does_not_affect_capabilities() {
    use crate::Capability;
    let preset = research_assistant();
    let cap_count_before = preset.capabilities.len();
    let modified = apply_overrides(preset, vec![("k".to_string(), "v".to_string())]);
    assert_eq!(modified.capabilities.len(), cap_count_before);
}
