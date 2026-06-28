use crate::{exfiltration_scenarios, AttackCategory};

#[test]
fn exfiltration_dataset_contains_blocked_scenarios() {
    let scenarios = exfiltration_scenarios();
    let blocked: Vec<_> = scenarios.iter().filter(|s| s.expected_blocked).collect();
    assert!(!blocked.is_empty());
}

#[test]
fn exfiltration_dataset_has_safe_baseline() {
    let scenarios = exfiltration_scenarios();
    let safe: Vec<_> = scenarios.iter().filter(|s| !s.expected_blocked).collect();
    assert!(!safe.is_empty());
}

#[test]
fn exfiltration_scenarios_categorized_correctly() {
    let scenarios = exfiltration_scenarios();
    assert!(scenarios.iter().all(|s| s.category == AttackCategory::DataExfiltration));
}

#[test]
fn exfiltration_ids_unique() {
    let scenarios = exfiltration_scenarios();
    let mut ids: Vec<&str> = scenarios.iter().map(|s| s.id.as_str()).collect();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), scenarios.len());
}
