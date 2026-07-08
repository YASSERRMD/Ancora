use crate::audit::{AirGapAction, AirGapAuditEntry, AirGapAuditLog};

#[test]
fn audit_record_and_count() {
    let mut log = AirGapAuditLog::new();
    log.record(AirGapAuditEntry::new(
        1,
        "t1",
        AirGapAction::TransferRequested,
        "alice",
        "req",
    ));
    log.record(AirGapAuditEntry::new(
        2,
        "t1",
        AirGapAction::TransferApproved,
        "bob",
        "approved",
    ));
    assert_eq!(log.count(), 2);
}

#[test]
fn audit_for_tenant() {
    let mut log = AirGapAuditLog::new();
    log.record(AirGapAuditEntry::new(
        1,
        "t1",
        AirGapAction::TransferRequested,
        "alice",
        "",
    ));
    log.record(AirGapAuditEntry::new(
        2,
        "t2",
        AirGapAction::TransferRequested,
        "bob",
        "",
    ));
    assert_eq!(log.for_tenant("t1").len(), 1);
}

#[test]
fn audit_all() {
    let mut log = AirGapAuditLog::new();
    for i in 0..5 {
        log.record(AirGapAuditEntry::new(
            i,
            "t1",
            AirGapAction::PolicyEvaluated,
            "sys",
            "",
        ));
    }
    assert_eq!(log.all().count(), 5);
}
