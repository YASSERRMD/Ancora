use crate::{BootAuditEntry, BootAuditLog, BootEvent};
#[test]
fn failures_returns_failed_entries() {
    let mut log = BootAuditLog::new();
    log.record(BootAuditEntry::new(0, "t1", "n1", BootEvent::ChainValidated, "a", true, "ok"));
    log.record(BootAuditEntry::new(1, "t1", "n1", BootEvent::ChainValidated, "a", false, "fail"));
    assert_eq!(log.failures().len(), 1);
}
