use crate::runbook::StepStatus;

#[test]
fn step_status_display() {
    assert_eq!(format!("{}", StepStatus::Pending), "PENDING");
    assert_eq!(format!("{}", StepStatus::InProgress), "IN_PROGRESS");
    assert_eq!(format!("{}", StepStatus::Completed), "COMPLETED");
    assert_eq!(format!("{}", StepStatus::Skipped), "SKIPPED");
    assert_eq!(format!("{}", StepStatus::Failed), "FAILED");
}
