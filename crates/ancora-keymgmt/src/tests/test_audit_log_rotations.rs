use crate::{KeyAuditEntry, KeyAuditLog, KeyOperation};
#[test]
fn rotations_for_returns_only_rotate_events() {
    let mut log = KeyAuditLog::new();
    log.record(KeyAuditEntry::new(1, "t1", "k1", 1, KeyOperation::Create, "a", true));
    log.record(KeyAuditEntry::new(2, "t1", "k1", 2, KeyOperation::Rotate, "a", true));
    assert_eq!(log.rotations_for("k1").len(), 1);
}
