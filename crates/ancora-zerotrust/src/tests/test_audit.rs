use crate::audit::{ZtAction, ZtAuditEntry, ZtAuditLog};

#[test]
fn audit_record_and_count() {
    let mut log = ZtAuditLog::new();
    log.record(ZtAuditEntry::new(
        1,
        "t1",
        "i1",
        "res",
        ZtAction::AccessGranted,
        true,
        "",
    ));
    log.record(ZtAuditEntry::new(
        2,
        "t1",
        "i1",
        "res",
        ZtAction::AccessDenied,
        false,
        "blocked",
    ));
    assert_eq!(log.count(), 2);
}

#[test]
fn audit_for_tenant() {
    let mut log = ZtAuditLog::new();
    log.record(ZtAuditEntry::new(
        1,
        "t1",
        "i1",
        "res",
        ZtAction::AccessGranted,
        true,
        "",
    ));
    log.record(ZtAuditEntry::new(
        2,
        "t2",
        "i2",
        "res",
        ZtAction::AccessGranted,
        true,
        "",
    ));
    assert_eq!(log.for_tenant("t1").len(), 1);
}

#[test]
fn audit_denied() {
    let mut log = ZtAuditLog::new();
    log.record(ZtAuditEntry::new(
        1,
        "t1",
        "i1",
        "res",
        ZtAction::AccessGranted,
        true,
        "",
    ));
    log.record(ZtAuditEntry::new(
        2,
        "t1",
        "i1",
        "res",
        ZtAction::AccessDenied,
        false,
        "blocked",
    ));
    assert_eq!(log.denied().len(), 1);
}

#[test]
fn audit_for_identity() {
    let mut log = ZtAuditLog::new();
    log.record(ZtAuditEntry::new(
        1,
        "t1",
        "i1",
        "res",
        ZtAction::SessionCreated,
        true,
        "",
    ));
    log.record(ZtAuditEntry::new(
        2,
        "t1",
        "i2",
        "res",
        ZtAction::PolicyEvaluated,
        true,
        "",
    ));
    assert_eq!(log.for_identity("i1").len(), 1);
}
