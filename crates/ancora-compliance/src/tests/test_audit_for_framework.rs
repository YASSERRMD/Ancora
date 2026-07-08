use crate::{AssessmentRecord, ComplianceAuditLog, ControlId, ControlStatus, Framework};
#[test]
fn for_framework_filters_correctly() {
    let mut log = ComplianceAuditLog::new();
    log.record(AssessmentRecord::new(
        1,
        "t1",
        ControlId::new("CC6.1"),
        Framework::Soc2,
        ControlStatus::NotAssessed,
        ControlStatus::Compliant,
        "a",
    ));
    log.record(AssessmentRecord::new(
        2,
        "t1",
        ControlId::new("A.5.1"),
        Framework::Iso27001,
        ControlStatus::NotAssessed,
        ControlStatus::Compliant,
        "b",
    ));
    assert_eq!(log.for_framework(&Framework::Soc2).len(), 1);
    assert_eq!(log.for_framework(&Framework::Fedramp).len(), 0);
}
