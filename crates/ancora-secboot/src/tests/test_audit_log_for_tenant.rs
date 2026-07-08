use crate::{BootAuditEntry, BootAuditLog, BootEvent};
#[test]
fn for_tenant_filters_entries() {
    let mut log = BootAuditLog::new();
    log.record(BootAuditEntry::new(
        0,
        "t1",
        "n1",
        BootEvent::PolicyChecked,
        "a",
        true,
        "ok",
    ));
    log.record(BootAuditEntry::new(
        1,
        "t2",
        "n2",
        BootEvent::PolicyChecked,
        "b",
        true,
        "ok",
    ));
    assert_eq!(log.for_tenant("t1").len(), 1);
    assert_eq!(log.for_tenant("t2").len(), 1);
}
