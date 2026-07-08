use crate::{AssessmentRecord, ComplianceAuditLog, ControlId, ControlStatus, Framework};
#[test]
fn for_control_returns_matching_entries() {
    let mut log = ComplianceAuditLog::new();
    let id = ControlId::new("CC6.1");
    log.record(AssessmentRecord::new(
        1,
        "t1",
        id.clone(),
        Framework::Soc2,
        ControlStatus::NotAssessed,
        ControlStatus::Compliant,
        "a",
    ));
    log.record(AssessmentRecord::new(
        2,
        "t1",
        ControlId::new("CC7.1"),
        Framework::Soc2,
        ControlStatus::NotAssessed,
        ControlStatus::NonCompliant,
        "b",
    ));
    assert_eq!(log.for_control(&id).len(), 1);
}
