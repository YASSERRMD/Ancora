use crate::safety_e2e::{default_safety_monitor, KeywordRule, SafetyMonitor, Severity};

#[test]
fn safety_monitor_flags_an_unsafe_output() {
    let monitor = default_safety_monitor();
    let unsafe_text = "This message contains bomb instructions.";

    assert!(!monitor.is_safe(unsafe_text));

    let flags = monitor.inspect(unsafe_text);
    assert!(!flags.is_empty());

    let highest = monitor.highest_severity(unsafe_text).unwrap();
    assert_eq!(highest, Severity::Critical);
}

#[test]
fn safety_monitor_passes_safe_output() {
    let monitor = default_safety_monitor();
    let safe_text = "The weather is nice today.";
    assert!(monitor.is_safe(safe_text));
    assert!(monitor.inspect(safe_text).is_empty());
}

#[test]
fn safety_monitor_is_case_insensitive() {
    let monitor = default_safety_monitor();
    let text = "BOMB detected in area";
    assert!(!monitor.is_safe(text));
}

#[test]
fn custom_rule_detects_keyword() {
    let mut monitor = SafetyMonitor::new();
    monitor.add_rule(KeywordRule::new("T001", "exploit", Severity::High, "Security issue"));

    assert!(!monitor.is_safe("Found an exploit in the code."));
    assert!(monitor.is_safe("Nothing suspicious here."));
}

#[test]
fn multiple_flags_are_returned_for_multiple_matches() {
    let monitor = default_safety_monitor();
    let text = "bomb hack spam";
    let flags = monitor.inspect(text);
    assert!(flags.len() >= 3);
}
