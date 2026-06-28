use crate::{AuditEntry, AuditStats, ImmutableAuditLog, Outcome, Severity};
#[test]
fn failure_rate_is_correct() {
    let mut log = ImmutableAuditLog::new();
    for _ in 0..3 {
        log.append(AuditEntry::new(0, 1, "t1", "a", "op", "r", Outcome::Failure, Severity::Warning));
    }
    for _ in 0..7 {
        log.append(AuditEntry::new(0, 2, "t1", "a", "op", "r", Outcome::Success, Severity::Info));
    }
    let stats = AuditStats::from_entries(log.entries());
    let rate = stats.failure_rate();
    assert!((rate - 0.3).abs() < 1e-10);
}
#[test]
fn failure_rate_zero_on_empty() {
    let stats = AuditStats::from_entries(std::iter::empty());
    assert_eq!(stats.failure_rate(), 0.0);
}
