use crate::{AuditEntry, AuditStats, ImmutableAuditLog, Outcome, Severity};
#[test]
fn stats_counts_critical_and_error_severity() {
    let mut log = ImmutableAuditLog::new();
    log.append(AuditEntry::new(
        0,
        1,
        "t1",
        "a",
        "op",
        "r",
        Outcome::Success,
        Severity::Critical,
    ));
    log.append(AuditEntry::new(
        0,
        2,
        "t1",
        "b",
        "op",
        "r",
        Outcome::Failure,
        Severity::Error,
    ));
    log.append(AuditEntry::new(
        0,
        3,
        "t1",
        "c",
        "op",
        "r",
        Outcome::Success,
        Severity::Info,
    ));
    let stats = AuditStats::from_entries(log.entries());
    assert_eq!(stats.critical, 1);
    assert_eq!(stats.errors, 1);
}
#[test]
fn stats_warning_not_counted_as_error() {
    let mut log = ImmutableAuditLog::new();
    log.append(AuditEntry::new(
        0,
        1,
        "t1",
        "a",
        "op",
        "r",
        Outcome::Success,
        Severity::Warning,
    ));
    let stats = AuditStats::from_entries(log.entries());
    assert_eq!(stats.errors, 0);
    assert_eq!(stats.critical, 0);
}
