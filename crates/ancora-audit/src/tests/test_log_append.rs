use crate::{AuditEntry, ImmutableAuditLog, Outcome, Severity};
#[test]
fn append_returns_sequential_ids() {
    let mut log = ImmutableAuditLog::new();
    let e1 = AuditEntry::new(
        0,
        1,
        "t1",
        "alice",
        "read",
        "res",
        Outcome::Success,
        Severity::Info,
    );
    let e2 = AuditEntry::new(
        0,
        2,
        "t1",
        "alice",
        "write",
        "res",
        Outcome::Success,
        Severity::Info,
    );
    let id1 = log.append(e1);
    let id2 = log.append(e2);
    assert_eq!(id1, 1);
    assert_eq!(id2, 2);
}
#[test]
fn append_assigns_id_to_entry() {
    let mut log = ImmutableAuditLog::new();
    let e = AuditEntry::new(
        0,
        10,
        "t1",
        "bob",
        "delete",
        "file",
        Outcome::Failure,
        Severity::Warning,
    );
    let id = log.append(e);
    assert_eq!(log.get(id).unwrap().id, id);
}
