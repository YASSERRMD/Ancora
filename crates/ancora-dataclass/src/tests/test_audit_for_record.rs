use crate::{AccessKind, ClassificationAuditEntry, ClassificationAuditLog, EnforcementDecision, SensitivityLevel};
#[test]
fn for_record_returns_only_that_records_entries() {
    let mut log = ClassificationAuditLog::new();
    log.record(ClassificationAuditEntry::from(1, "t1", "a", "r1", SensitivityLevel::Internal, AccessKind::Read, &EnforcementDecision::Allow));
    log.record(ClassificationAuditEntry::from(2, "t1", "b", "r2", SensitivityLevel::Internal, AccessKind::Read, &EnforcementDecision::Allow));
    assert_eq!(log.for_record("r1").len(), 1);
    assert_eq!(log.for_record("r2").len(), 1);
    assert_eq!(log.for_record("r3").len(), 0);
}
