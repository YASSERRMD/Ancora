use crate::audit::{HsmAuditEntry, HsmAuditLog, HsmOperation};

#[test]
fn by_operation_filters() {
    let mut log = HsmAuditLog::new();
    log.record(HsmAuditEntry::new(1, 0, HsmOperation::Sign, true, ""));
    log.record(HsmAuditEntry::new(2, 0, HsmOperation::Encrypt, true, ""));
    log.record(HsmAuditEntry::new(3, 0, HsmOperation::Sign, true, ""));
    let signs = log.by_operation(&HsmOperation::Sign);
    assert_eq!(signs.len(), 2);
}

#[test]
fn by_operation_empty() {
    let log = HsmAuditLog::new();
    assert_eq!(log.by_operation(&HsmOperation::WrapKey).len(), 0);
}
