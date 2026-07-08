use crate::{BootAuditEntry, BootAuditLog, BootEvent};
#[test]
fn audit_log_records_and_counts() {
    let mut log = BootAuditLog::new();
    log.record(BootAuditEntry::new(
        0,
        "t1",
        "n1",
        BootEvent::MeasurementAdded,
        "alice",
        true,
        "ok",
    ));
    assert_eq!(log.count(), 1);
}
