use crate::{AuditEntry, Outcome, Severity};
#[test]
fn entry_detail_stored_and_accessible() {
    let e = AuditEntry::new(
        1,
        100,
        "t1",
        "alice",
        "op",
        "res",
        Outcome::Success,
        Severity::Info,
    )
    .with_detail("ip", "10.0.0.1")
    .with_detail("user_agent", "rust-client/1.0");
    assert_eq!(e.details.get("ip").unwrap(), "10.0.0.1");
    assert_eq!(e.details.get("user_agent").unwrap(), "rust-client/1.0");
}
#[test]
fn entry_detail_recomputes_checksum() {
    let e1 = AuditEntry::new(
        1,
        100,
        "t1",
        "alice",
        "op",
        "res",
        Outcome::Success,
        Severity::Info,
    );
    let e2 = e1.clone().with_detail("key", "val");
    assert!(e1.verify());
    assert!(e2.verify());
}
