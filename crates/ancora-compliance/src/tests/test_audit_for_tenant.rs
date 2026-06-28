use crate::{AssessmentRecord, ComplianceAuditLog, ControlId, ControlStatus, Framework};
#[test]
fn for_tenant_filters_by_tenant_id() {
    let mut log = ComplianceAuditLog::new();
    log.record(AssessmentRecord::new(1, "t1", ControlId::new("CC6.1"), Framework::Soc2, ControlStatus::NotAssessed, ControlStatus::Compliant, "a"));
    log.record(AssessmentRecord::new(2, "t2", ControlId::new("CC6.1"), Framework::Soc2, ControlStatus::NotAssessed, ControlStatus::NonCompliant, "b"));
    assert_eq!(log.for_tenant("t1").len(), 1);
    assert_eq!(log.for_tenant("t2").len(), 1);
}
