use crate::{
    AccessKind, ClassificationAuditEntry, ClassificationAuditLog, EnforcementDecision,
    SensitivityLevel,
};
#[test]
fn allowed_for_tenant_filters_correctly() {
    let mut log = ClassificationAuditLog::new();
    log.record(ClassificationAuditEntry::from(
        1,
        "t1",
        "a",
        "r1",
        SensitivityLevel::Internal,
        AccessKind::Read,
        &EnforcementDecision::Allow,
    ));
    log.record(ClassificationAuditEntry::from(
        2,
        "t1",
        "b",
        "r2",
        SensitivityLevel::Internal,
        AccessKind::Read,
        &EnforcementDecision::Deny("d".into()),
    ));
    assert_eq!(log.allowed_for_tenant("t1").len(), 1);
    assert_eq!(log.allowed_for_tenant("t1")[0].record_id, "r1");
}
