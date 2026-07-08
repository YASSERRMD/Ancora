use crate::{KeyAuditEntry, KeyAuditLog, KeyOperation};
#[test]
fn for_tenant_filters_correctly() {
    let mut log = KeyAuditLog::new();
    log.record(KeyAuditEntry::new(
        1,
        "t1",
        "k1",
        1,
        KeyOperation::Read,
        "a",
        true,
    ));
    log.record(KeyAuditEntry::new(
        2,
        "t2",
        "k2",
        1,
        KeyOperation::Read,
        "b",
        true,
    ));
    assert_eq!(log.for_tenant("t1").len(), 1);
    assert_eq!(log.for_tenant("t2").len(), 1);
}
