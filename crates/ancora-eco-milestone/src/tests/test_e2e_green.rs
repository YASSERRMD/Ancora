use crate::e2e_status::{all_e2e_passing, ecosystem_e2e_scenarios};

#[test]
fn test_all_e2e_scenarios_pass() {
    let scenarios = ecosystem_e2e_scenarios();
    assert!(!scenarios.is_empty(), "should have e2e scenarios");
    assert!(all_e2e_passing(&scenarios), "all e2e scenarios should pass");
}

#[test]
fn test_e2e_durations_are_positive() {
    let scenarios = ecosystem_e2e_scenarios();
    for s in &scenarios {
        assert!(s.duration_ms > 0, "scenario {} should have positive duration", s.name);
    }
}
