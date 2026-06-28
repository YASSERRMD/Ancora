use crate::{AuditEntry, ImmutableAuditLog, Outcome, Severity};
#[test]
fn filter_by_tick_range_inclusive() {
    let mut log = ImmutableAuditLog::new();
    for tick in 10u64..=20 {
        log.append(AuditEntry::new(0, tick, "t1", "alice", "op", "r", Outcome::Success, Severity::Info));
    }
    let range = log.filter_by_tick_range(13, 17);
    assert_eq!(range.len(), 5);
    assert!(range.iter().all(|e| e.tick >= 13 && e.tick <= 17));
}
#[test]
fn filter_by_tick_range_excludes_outside() {
    let mut log = ImmutableAuditLog::new();
    log.append(AuditEntry::new(0, 5, "t1", "a", "op", "r", Outcome::Success, Severity::Info));
    log.append(AuditEntry::new(0, 15, "t1", "a", "op", "r", Outcome::Success, Severity::Info));
    log.append(AuditEntry::new(0, 25, "t1", "a", "op", "r", Outcome::Success, Severity::Info));
    let range = log.filter_by_tick_range(10, 20);
    assert_eq!(range.len(), 1);
    assert_eq!(range[0].tick, 15);
}
