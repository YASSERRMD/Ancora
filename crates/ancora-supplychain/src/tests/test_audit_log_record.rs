use crate::audit::{SupplyChainAuditEntry, SupplyChainAuditLog, SupplyChainEvent};
fn make_entry(
    tick: u64,
    tenant_id: &str,
    component_id: &str,
    success: bool,
) -> SupplyChainAuditEntry {
    SupplyChainAuditEntry::new(
        tick,
        tenant_id,
        component_id,
        SupplyChainEvent::ComponentAdded,
        "subject",
        success,
    )
}
#[test]
fn new_log_has_zero_count() {
    let log = SupplyChainAuditLog::new();
    assert_eq!(log.count(), 0);
}
#[test]
fn record_increments_count() {
    let mut log = SupplyChainAuditLog::new();
    log.record(make_entry(1, "t1", "c1", true));
    assert_eq!(log.count(), 1);
}
#[test]
fn all_returns_all_entries() {
    let mut log = SupplyChainAuditLog::new();
    log.record(make_entry(1, "t1", "c1", true));
    log.record(make_entry(2, "t2", "c2", false));
    assert_eq!(log.all().count(), 2);
}
