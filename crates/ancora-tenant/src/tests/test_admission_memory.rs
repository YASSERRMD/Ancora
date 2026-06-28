use crate::{AdmissionController, AdmissionDecision, ResourceQuota, ResourceUsage};
#[test]
fn admit_memory_within_quota() {
    let quota = ResourceQuota::new(10, 100, 1024, 4000, 50, 100_000);
    let usage = ResourceUsage { memory_mb: 500, ..Default::default() };
    assert_eq!(AdmissionController::check_memory(&quota, &usage, 200), AdmissionDecision::Allow);
}
#[test]
fn deny_memory_over_quota() {
    let quota = ResourceQuota::new(10, 100, 1024, 4000, 50, 100_000);
    let usage = ResourceUsage { memory_mb: 900, ..Default::default() };
    let d = AdmissionController::check_memory(&quota, &usage, 200);
    assert!(matches!(d, AdmissionDecision::Deny(_)));
}
