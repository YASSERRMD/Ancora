use crate::{AuditEntry, ImmutableAuditLog, Outcome, Severity};
#[test]
fn count_reflects_number_of_entries() {
    let mut log = ImmutableAuditLog::new();
    assert_eq!(log.count(), 0);
    for i in 1u64..=5 {
        log.append(AuditEntry::new(0, i, "t1", "alice", "op", "r", Outcome::Success, Severity::Info));
    }
    assert_eq!(log.count(), 5);
}
#[test]
fn count_decreases_when_evicted_by_max_size() {
    let mut log = ImmutableAuditLog::new().with_max_size(2);
    for i in 1u64..=4 {
        log.append(AuditEntry::new(0, i, "t1", "alice", "op", "r", Outcome::Success, Severity::Info));
    }
    assert_eq!(log.count(), 2);
}
