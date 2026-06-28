use crate::{AdmissionController, AdmissionDecision, ResourceQuota, ResourceUsage};
#[test]
fn admit_log_entries_within_quota() {
    let quota = ResourceQuota::new(10, 100, 4096, 4000, 50, 1000);
    let usage = ResourceUsage { log_entries: 500, ..Default::default() };
    assert_eq!(AdmissionController::check_log_entries(&quota, &usage, 400), AdmissionDecision::Allow);
}
#[test]
fn deny_log_entries_over_quota() {
    let quota = ResourceQuota::new(10, 100, 4096, 4000, 50, 1000);
    let usage = ResourceUsage { log_entries: 1000, ..Default::default() };
    let d = AdmissionController::check_log_entries(&quota, &usage, 1);
    assert!(matches!(d, AdmissionDecision::Deny(_)));
}
