use crate::{AdmissionController, AdmissionDecision, ResourceQuota, ResourceUsage};
#[test]
fn admit_secret_within_quota() {
    let quota = ResourceQuota::new(10, 100, 4096, 4000, 20, 100_000);
    let usage = ResourceUsage {
        secrets: 15,
        ..Default::default()
    };
    assert_eq!(
        AdmissionController::check_secrets(&quota, &usage, 4),
        AdmissionDecision::Allow
    );
}
#[test]
fn deny_secret_over_quota() {
    let quota = ResourceQuota::new(10, 100, 4096, 4000, 20, 100_000);
    let usage = ResourceUsage {
        secrets: 20,
        ..Default::default()
    };
    let d = AdmissionController::check_secrets(&quota, &usage, 1);
    assert!(matches!(d, AdmissionDecision::Deny(_)));
}
