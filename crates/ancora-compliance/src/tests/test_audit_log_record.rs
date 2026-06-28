use crate::{AssessmentRecord, ComplianceAuditLog, ControlId, ControlStatus, Framework};
#[test]
fn audit_log_records_entry() {
    let mut log = ComplianceAuditLog::new();
    log.record(AssessmentRecord::new(
        1, "t1", ControlId::new("CC6.1"), Framework::Soc2,
        ControlStatus::NotAssessed, ControlStatus::Compliant, "alice",
    ));
    assert_eq!(log.count(), 1);
}
