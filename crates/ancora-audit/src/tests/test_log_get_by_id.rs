use crate::{AuditEntry, ImmutableAuditLog, Outcome, Severity};
#[test]
fn get_by_id_returns_correct_entry() {
    let mut log = ImmutableAuditLog::new();
    log.append(AuditEntry::new(0, 1, "t1", "alice", "read", "r", Outcome::Success, Severity::Info));
    let id = log.append(AuditEntry::new(0, 2, "t1", "bob", "write", "r", Outcome::Failure, Severity::Warning));
    log.append(AuditEntry::new(0, 3, "t1", "carol", "delete", "r", Outcome::Blocked, Severity::Error));
    let entry = log.get(id).unwrap();
    assert_eq!(entry.subject, "bob");
    assert_eq!(entry.operation, "write");
}
#[test]
fn get_by_id_none_for_missing() {
    let log = ImmutableAuditLog::new();
    assert!(log.get(999).is_none());
}
