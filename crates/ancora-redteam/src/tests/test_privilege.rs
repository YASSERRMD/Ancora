use crate::{privilege_scenarios, AttackCategory};

#[test]
fn privilege_dataset_has_blocked_scenarios() {
    let scenarios = privilege_scenarios();
    let blocked: Vec<_> = scenarios.iter().filter(|s| s.expected_blocked).collect();
    assert!(!blocked.is_empty());
}

#[test]
fn privilege_dataset_has_safe_baseline() {
    let scenarios = privilege_scenarios();
    let safe: Vec<_> = scenarios.iter().filter(|s| !s.expected_blocked).collect();
    assert!(!safe.is_empty());
}

#[test]
fn privilege_scenarios_categorized_correctly() {
    let scenarios = privilege_scenarios();
    assert!(scenarios.iter().all(|s| s.category == AttackCategory::PrivilegeEscalation));
}

#[test]
fn privilege_ids_start_with_priv() {
    let scenarios = privilege_scenarios();
    assert!(scenarios.iter().all(|s| s.id.starts_with("priv-")));
}
