use crate::{AuditEntry, Outcome, Severity};
#[test]
fn tampered_entry_fails_verify() {
    let mut e = AuditEntry::new(
        1,
        100,
        "t1",
        "alice",
        "login",
        "auth",
        Outcome::Success,
        Severity::Info,
    );
    e.operation = "delete".into();
    assert!(!e.verify());
}
