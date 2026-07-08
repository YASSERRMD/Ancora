use crate::{AdmissionController, AdmissionDecision, ResourceQuota, ResourceUsage};
#[test]
fn admit_task_within_quota() {
    let quota = ResourceQuota::new(10, 50, 4096, 4000, 50, 100_000);
    let usage = ResourceUsage {
        tasks: 40,
        ..Default::default()
    };
    assert_eq!(
        AdmissionController::check_tasks(&quota, &usage, 5),
        AdmissionDecision::Allow
    );
}
#[test]
fn deny_task_over_quota() {
    let quota = ResourceQuota::new(10, 50, 4096, 4000, 50, 100_000);
    let usage = ResourceUsage {
        tasks: 50,
        ..Default::default()
    };
    let d = AdmissionController::check_tasks(&quota, &usage, 1);
    assert!(matches!(d, AdmissionDecision::Deny(_)));
}
