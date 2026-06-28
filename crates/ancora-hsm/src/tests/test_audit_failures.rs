use crate::audit::{HsmAuditEntry, HsmAuditLog, HsmOperation};

#[test]
fn no_failures_by_default() {
    let mut log = HsmAuditLog::new();
    log.record(HsmAuditEntry::new(1, 0, HsmOperation::Sign, true, "ok"));
    assert_eq!(log.failures().len(), 0);
}

#[test]
fn failure_recorded() {
    let mut log = HsmAuditLog::new();
    log.record(HsmAuditEntry::new(1, 0, HsmOperation::Decrypt, false, "bad key"));
    assert_eq!(log.failures().len(), 1);
    assert!(!log.failures()[0].success);
}

#[test]
fn multiple_failures() {
    let mut log = HsmAuditLog::new();
    log.record(HsmAuditEntry::new(1, 0, HsmOperation::Encrypt, false, "err"));
    log.record(HsmAuditEntry::new(2, 0, HsmOperation::Sign, true, "ok"));
    log.record(HsmAuditEntry::new(3, 0, HsmOperation::Decrypt, false, "err"));
    assert_eq!(log.failures().len(), 2);
}
