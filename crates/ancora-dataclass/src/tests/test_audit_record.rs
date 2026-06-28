use crate::{AccessKind, ClassificationAuditEntry, ClassificationAuditLog, EnforcementDecision, SensitivityLevel};
#[test]
fn audit_log_records_entry() {
    let mut log = ClassificationAuditLog::new();
    let entry = ClassificationAuditEntry::from(
        1, "t1", "alice", "r1", SensitivityLevel::Confidential, AccessKind::Read, &EnforcementDecision::Allow,
    );
    log.record(entry);
    assert_eq!(log.count(), 1);
}
