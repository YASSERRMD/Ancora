use crate::{AuditEntry, Outcome, Severity};
#[test]
fn entry_checksum_is_nonzero() {
    let e = AuditEntry::new(1, 100, "t1", "alice", "login", "auth", Outcome::Success, Severity::Info);
    assert_ne!(e.checksum, 0);
}
#[test]
fn entry_verifies() {
    let e = AuditEntry::new(1, 100, "t1", "alice", "login", "auth", Outcome::Success, Severity::Info);
    assert!(e.verify());
}
