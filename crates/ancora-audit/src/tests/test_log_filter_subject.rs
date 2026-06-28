use crate::{AuditEntry, ImmutableAuditLog, Outcome, Severity};
#[test]
fn filter_by_subject_isolates_user() {
    let mut log = ImmutableAuditLog::new();
    log.append(AuditEntry::new(0, 1, "t1", "alice", "read", "r", Outcome::Success, Severity::Info));
    log.append(AuditEntry::new(0, 2, "t1", "bob", "write", "r", Outcome::Success, Severity::Info));
    log.append(AuditEntry::new(0, 3, "t1", "alice", "delete", "r", Outcome::Failure, Severity::Warning));
    let alice = log.filter_by_subject("alice");
    assert_eq!(alice.len(), 2);
    assert!(alice.iter().all(|e| e.subject == "alice"));
}
