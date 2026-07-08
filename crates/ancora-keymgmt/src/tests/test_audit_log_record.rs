use crate::{KeyAuditEntry, KeyAuditLog, KeyOperation};
#[test]
fn audit_log_records_entry() {
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
    assert_eq!(log.count(), 1);
}
