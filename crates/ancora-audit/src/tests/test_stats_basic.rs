use crate::{AuditEntry, AuditStats, ImmutableAuditLog, Outcome, Severity};
#[test]
fn stats_counts_outcomes() {
    let mut log = ImmutableAuditLog::new();
    log.append(AuditEntry::new(0, 1, "t1", "a", "op", "r", Outcome::Success, Severity::Info));
    log.append(AuditEntry::new(0, 2, "t1", "b", "op", "r", Outcome::Failure, Severity::Warning));
    log.append(AuditEntry::new(0, 3, "t1", "c", "op", "r", Outcome::Blocked, Severity::Info));
    let stats = AuditStats::from_entries(log.entries());
    assert_eq!(stats.total, 3);
    assert_eq!(stats.successes, 1);
    assert_eq!(stats.failures, 1);
    assert_eq!(stats.blocked, 1);
}
