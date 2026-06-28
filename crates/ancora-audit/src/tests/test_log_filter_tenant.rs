use crate::{AuditEntry, ImmutableAuditLog, Outcome, Severity};
#[test]
fn filter_by_tenant_returns_only_matching() {
    let mut log = ImmutableAuditLog::new();
    log.append(AuditEntry::new(0, 1, "t1", "alice", "read", "r", Outcome::Success, Severity::Info));
    log.append(AuditEntry::new(0, 2, "t2", "bob", "read", "r", Outcome::Success, Severity::Info));
    log.append(AuditEntry::new(0, 3, "t1", "carol", "read", "r", Outcome::Success, Severity::Info));
    let t1 = log.filter_by_tenant("t1");
    assert_eq!(t1.len(), 2);
    assert!(t1.iter().all(|e| e.tenant_id == "t1"));
}
#[test]
fn filter_by_tenant_empty_when_no_match() {
    let mut log = ImmutableAuditLog::new();
    log.append(AuditEntry::new(0, 1, "t1", "alice", "read", "r", Outcome::Success, Severity::Info));
    assert!(log.filter_by_tenant("t999").is_empty());
}
