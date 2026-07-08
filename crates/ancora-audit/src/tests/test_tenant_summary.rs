use crate::{summarize_by_tenant, AuditEntry, ImmutableAuditLog, Outcome, Severity};
#[test]
fn summary_groups_by_tenant() {
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
        "bob",
        "read",
        "r",
        Outcome::Failure,
        Severity::Warning,
    ));
    log.append(AuditEntry::new(
        0,
        3,
        "t1",
        "alice",
        "write",
        "r",
        Outcome::Success,
        Severity::Info,
    ));
    let entries: Vec<&AuditEntry> = log.entries().collect();
    let summaries = summarize_by_tenant(&entries);
    assert_eq!(summaries.len(), 2);
    let t1 = summaries.iter().find(|s| s.tenant_id == "t1").unwrap();
    assert_eq!(t1.stats.total, 2);
    assert_eq!(t1.stats.successes, 2);
}
#[test]
fn summary_sorted_by_tenant_id() {
    let mut log = ImmutableAuditLog::new();
    log.append(AuditEntry::new(
        0,
        1,
        "zzz",
        "a",
        "op",
        "r",
        Outcome::Success,
        Severity::Info,
    ));
    log.append(AuditEntry::new(
        0,
        2,
        "aaa",
        "b",
        "op",
        "r",
        Outcome::Success,
        Severity::Info,
    ));
    let entries: Vec<&AuditEntry> = log.entries().collect();
    let summaries = summarize_by_tenant(&entries);
    assert_eq!(summaries[0].tenant_id, "aaa");
    assert_eq!(summaries[1].tenant_id, "zzz");
}
