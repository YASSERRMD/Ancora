use crate::{AdmissionController, AdmissionDecision, ResourceQuota, ResourceUsage};
#[test]
fn all_admission_checks_pass_on_empty_usage() {
    let quota = ResourceQuota::standard();
    let usage = ResourceUsage::new();
    assert_eq!(AdmissionController::check_agents(&quota, &usage, 1), AdmissionDecision::Allow);
    assert_eq!(AdmissionController::check_tasks(&quota, &usage, 1), AdmissionDecision::Allow);
    assert_eq!(AdmissionController::check_memory(&quota, &usage, 1), AdmissionDecision::Allow);
    assert_eq!(AdmissionController::check_secrets(&quota, &usage, 1), AdmissionDecision::Allow);
    assert_eq!(AdmissionController::check_log_entries(&quota, &usage, 1), AdmissionDecision::Allow);
}
