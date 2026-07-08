use crate::{AdmissionController, AdmissionDecision, ResourceQuota, ResourceUsage};
#[test]
fn admit_agent_within_quota() {
    let quota = ResourceQuota::new(5, 100, 4096, 4000, 50, 100_000);
    let usage = ResourceUsage {
        agents: 3,
        ..Default::default()
    };
    assert_eq!(
        AdmissionController::check_agents(&quota, &usage, 1),
        AdmissionDecision::Allow
    );
}
#[test]
fn deny_agent_over_quota() {
    let quota = ResourceQuota::new(5, 100, 4096, 4000, 50, 100_000);
    let usage = ResourceUsage {
        agents: 5,
        ..Default::default()
    };
    let decision = AdmissionController::check_agents(&quota, &usage, 1);
    assert!(matches!(decision, AdmissionDecision::Deny(_)));
}
