use crate::{AuditEntry, AuditQuery, ImmutableAuditLog, Outcome, Severity};
#[test]
fn query_by_tenant_and_subject() {
    let mut log = ImmutableAuditLog::new();
    log.append(AuditEntry::new(
        0,
        1,
        "t1",
        "alice",
        "read",
        "r",
        Outcome::Success,
        Severity::Info,
    ));
    log.append(AuditEntry::new(
        0,
        2,
        "t2",
        "alice",
        "write",
        "r",
        Outcome::Success,
        Severity::Info,
    ));
    log.append(AuditEntry::new(
        0,
        3,
        "t1",
        "bob",
        "read",
        "r",
        Outcome::Success,
        Severity::Info,
    ));
    let results = AuditQuery::new()
        .tenant("t1")
        .subject("alice")
        .run(log.entries());
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].tick, 1);
}
#[test]
fn query_by_outcome_filters() {
    let mut log = ImmutableAuditLog::new();
    log.append(AuditEntry::new(
        0,
        1,
        "t1",
        "a",
        "op",
        "r",
        Outcome::Success,
        Severity::Info,
    ));
    log.append(AuditEntry::new(
        0,
        2,
        "t1",
        "b",
        "op",
        "r",
        Outcome::Failure,
        Severity::Warning,
    ));
    log.append(AuditEntry::new(
        0,
        3,
        "t1",
        "c",
        "op",
        "r",
        Outcome::Failure,
        Severity::Error,
    ));
    let results = AuditQuery::new()
        .outcome(Outcome::Failure)
        .run(log.entries());
    assert_eq!(results.len(), 2);
}
#[test]
fn query_by_tick_range() {
    let mut log = ImmutableAuditLog::new();
    for tick in 1u64..=10 {
        log.append(AuditEntry::new(
            0,
            tick,
            "t1",
            "a",
            "op",
            "r",
            Outcome::Success,
            Severity::Info,
        ));
    }
    let results = AuditQuery::new().tick_from(4).tick_to(7).run(log.entries());
    assert_eq!(results.len(), 4);
}
