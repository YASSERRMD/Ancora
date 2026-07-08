use crate::audit::{HsmAuditEntry, HsmAuditLog, HsmOperation};

#[test]
fn audit_record_and_count() {
    let mut log = HsmAuditLog::new();
    log.record(HsmAuditEntry::new(
        1,
        0,
        HsmOperation::GenerateKey,
        true,
        "gen",
    ));
    log.record(HsmAuditEntry::new(2, 0, HsmOperation::Sign, true, "signed"));
    assert_eq!(log.count(), 2);
}

#[test]
fn audit_for_slot() {
    let mut log = HsmAuditLog::new();
    log.record(HsmAuditEntry::new(
        1,
        0,
        HsmOperation::GenerateKey,
        true,
        "",
    ));
    log.record(HsmAuditEntry::new(
        2,
        1,
        HsmOperation::GenerateKey,
        true,
        "",
    ));
    assert_eq!(log.for_slot(0).len(), 1);
}

#[test]
fn audit_failures() {
    let mut log = HsmAuditLog::new();
    log.record(HsmAuditEntry::new(1, 0, HsmOperation::Sign, true, "ok"));
    log.record(HsmAuditEntry::new(
        2,
        0,
        HsmOperation::Encrypt,
        false,
        "fail",
    ));
    assert_eq!(log.failures().len(), 1);
}

#[test]
fn audit_all() {
    let mut log = HsmAuditLog::new();
    for i in 0..3 {
        log.record(HsmAuditEntry::new(i, 0, HsmOperation::Decrypt, true, ""));
    }
    assert_eq!(log.all().count(), 3);
}
