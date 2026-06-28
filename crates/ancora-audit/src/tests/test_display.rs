use crate::{AuditEntry, Outcome, Severity};
#[test]
fn display_contains_subject_and_operation() {
    let e = AuditEntry::new(5, 200, "tenant-a", "carol", "delete", "secret", Outcome::Blocked, Severity::Critical);
    let s = format!("{}", e);
    assert!(s.contains("carol"));
    assert!(s.contains("delete"));
    assert!(s.contains("secret"));
}
#[test]
fn display_contains_tick_and_id() {
    let e = AuditEntry::new(7, 999, "t1", "alice", "read", "r", Outcome::Success, Severity::Info);
    let s = format!("{}", e);
    assert!(s.contains("tick=999"));
    assert!(s.contains("id=7"));
}
