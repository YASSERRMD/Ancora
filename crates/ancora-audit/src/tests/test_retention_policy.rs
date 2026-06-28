use crate::{AuditEntry, ImmutableAuditLog, Outcome, RetentionPolicy, Severity};
#[test]
fn retention_identifies_expired_entries() {
    let mut log = ImmutableAuditLog::new();
    for tick in 1u64..=10 {
        log.append(AuditEntry::new(0, tick, "t1", "a", "op", "r", Outcome::Success, Severity::Info));
    }
    let policy = RetentionPolicy::new(5);
    let expired_ids = policy.evict(&log, 10);
    assert!(!expired_ids.is_empty());
    let expired_entries: Vec<_> = log.entries().filter(|e| expired_ids.contains(&e.id)).collect();
    assert!(expired_entries.iter().all(|e| e.tick < 10 - 5));
}
#[test]
fn retention_count_expired() {
    let mut log = ImmutableAuditLog::new();
    for tick in 1u64..=10 {
        log.append(AuditEntry::new(0, tick, "t1", "a", "op", "r", Outcome::Success, Severity::Info));
    }
    let policy = RetentionPolicy::new(5);
    let count = policy.count_expired(&log, 10);
    assert_eq!(count, 4);
}
