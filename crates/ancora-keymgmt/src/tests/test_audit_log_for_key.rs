use crate::{KeyAuditEntry, KeyAuditLog, KeyOperation};
#[test]
fn for_key_filters_correctly() {
    let mut log = KeyAuditLog::new();
    log.record(KeyAuditEntry::new(
        1,
        "t1",
        "k1",
        1,
        KeyOperation::Create,
        "alice",
        true,
    ));
    log.record(KeyAuditEntry::new(
        2,
        "t1",
        "k2",
        1,
        KeyOperation::Create,
        "bob",
        true,
    ));
    assert_eq!(log.for_key("k1").len(), 1);
    assert_eq!(log.for_key("k2").len(), 1);
    assert_eq!(log.for_key("k3").len(), 0);
}
