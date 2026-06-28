use crate::{apply_overrides, get_override, research_assistant};

#[test]
fn multiple_overrides_all_applied() {
    let preset = research_assistant();
    let modified = apply_overrides(
        preset,
        vec![
            ("depth".to_string(), "5".to_string()),
            ("timeout".to_string(), "60s".to_string()),
            ("model".to_string(), "fast".to_string()),
        ],
    );
    assert_eq!(get_override(&modified, "depth"), Some("5"));
    assert_eq!(get_override(&modified, "timeout"), Some("60s"));
    assert_eq!(get_override(&modified, "model"), Some("fast"));
}

#[test]
fn overrides_preserve_preset_name() {
    let preset = research_assistant();
    let modified = apply_overrides(preset, vec![("x".to_string(), "y".to_string())]);
    assert_eq!(modified.name, "research-assistant");
}
