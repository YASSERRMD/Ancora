use crate::{AuditEntry, ImmutableAuditLog, Outcome, Severity};
#[test]
fn log_evicts_oldest_when_full() {
    let mut log = ImmutableAuditLog::new().with_max_size(3);
    for tick in 1u64..=5 {
        log.append(AuditEntry::new(0, tick, "t1", "alice", "op", "r", Outcome::Success, Severity::Info));
    }
    assert_eq!(log.count(), 3);
    let ticks: Vec<u64> = log.entries().map(|e| e.tick).collect();
    assert_eq!(ticks, vec![3, 4, 5]);
}
#[test]
fn log_without_max_size_grows_unbounded() {
    let mut log = ImmutableAuditLog::new();
    for tick in 1u64..=100 {
        log.append(AuditEntry::new(0, tick, "t1", "alice", "op", "r", Outcome::Success, Severity::Info));
    }
    assert_eq!(log.count(), 100);
}
