use crate::{AccessKind, ClassificationAuditEntry, ClassificationAuditLog, EnforcementDecision, SensitivityLevel};
#[test]
fn denied_for_tenant_filters_correctly() {
    let mut log = ClassificationAuditLog::new();
    log.record(ClassificationAuditEntry::from(1, "t1", "a", "r1", SensitivityLevel::Restricted, AccessKind::Write, &EnforcementDecision::Deny("d".into())));
    log.record(ClassificationAuditEntry::from(2, "t1", "b", "r2", SensitivityLevel::Internal, AccessKind::Read, &EnforcementDecision::Allow));
    assert_eq!(log.denied_for_tenant("t1").len(), 1);
}
