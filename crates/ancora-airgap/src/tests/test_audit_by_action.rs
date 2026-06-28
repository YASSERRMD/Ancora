use crate::audit::{AirGapAction, AirGapAuditEntry, AirGapAuditLog};

#[test]
fn by_action_filters() {
    let mut log = AirGapAuditLog::new();
    log.record(AirGapAuditEntry::new(1, "t1", AirGapAction::TransferRequested, "alice", ""));
    log.record(AirGapAuditEntry::new(2, "t1", AirGapAction::TransferApproved, "bob", ""));
    log.record(AirGapAuditEntry::new(3, "t1", AirGapAction::TransferRequested, "carol", ""));
    let requested = log.by_action(&AirGapAction::TransferRequested);
    assert_eq!(requested.len(), 2);
}

#[test]
fn by_action_empty() {
    let log = AirGapAuditLog::new();
    assert_eq!(log.by_action(&AirGapAction::ZoneCreated).len(), 0);
}
