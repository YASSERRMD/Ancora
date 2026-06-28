use crate::procedure::ProcedureStepStatus;

#[test]
fn display_pending() {
    assert_eq!(format!("{}", ProcedureStepStatus::Pending), "PENDING");
}

#[test]
fn display_completed() {
    assert_eq!(format!("{}", ProcedureStepStatus::Completed), "COMPLETED");
}

#[test]
fn display_skipped() {
    assert_eq!(format!("{}", ProcedureStepStatus::Skipped), "SKIPPED");
}

#[test]
fn display_failed() {
    assert_eq!(format!("{}", ProcedureStepStatus::Failed), "FAILED");
}
