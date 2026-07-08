use crate::{AuditEntry, ImmutableAuditLog, Outcome, Severity};
#[test]
fn ids_are_monotonically_increasing() {
    let mut log = ImmutableAuditLog::new();
    let mut prev = 0u64;
    for tick in 1u64..=10 {
        let id = log.append(AuditEntry::new(
            0,
            tick,
            "t1",
            "alice",
            "op",
            "r",
            Outcome::Success,
            Severity::Info,
        ));
        assert!(id > prev);
        prev = id;
    }
}
#[test]
fn ids_never_repeat_after_eviction() {
    let mut log = ImmutableAuditLog::new().with_max_size(2);
    let id1 = log.append(AuditEntry::new(
        0,
        1,
        "t1",
        "a",
        "op",
        "r",
        Outcome::Success,
        Severity::Info,
    ));
    let id2 = log.append(AuditEntry::new(
        0,
        2,
        "t1",
        "b",
        "op",
        "r",
        Outcome::Success,
        Severity::Info,
    ));
    let id3 = log.append(AuditEntry::new(
        0,
        3,
        "t1",
        "c",
        "op",
        "r",
        Outcome::Success,
        Severity::Info,
    ));
    assert!(id3 > id2);
    assert!(id2 > id1);
    assert!(log.get(id1).is_none());
}
