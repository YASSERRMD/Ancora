use crate::{AuditEntry, ImmutableAuditLog, Outcome, Severity};
#[test]
fn filter_by_operation_returns_matching_ops() {
    let mut log = ImmutableAuditLog::new();
    log.append(AuditEntry::new(0, 1, "t1", "alice", "read", "r", Outcome::Success, Severity::Info));
    log.append(AuditEntry::new(0, 2, "t1", "bob", "write", "r", Outcome::Success, Severity::Info));
    log.append(AuditEntry::new(0, 3, "t1", "carol", "read", "r", Outcome::Success, Severity::Info));
    let reads = log.filter_by_operation("read");
    assert_eq!(reads.len(), 2);
    assert!(reads.iter().all(|e| e.operation == "read"));
}
#[test]
fn filter_by_operation_empty_on_no_match() {
    let mut log = ImmutableAuditLog::new();
    log.append(AuditEntry::new(0, 1, "t1", "alice", "read", "r", Outcome::Success, Severity::Info));
    assert!(log.filter_by_operation("delete").is_empty());
}
