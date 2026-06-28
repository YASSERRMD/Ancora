use crate::audit::{SupplyChainAuditEntry, SupplyChainAuditLog, SupplyChainEvent};
fn make_entry(tick: u64, tenant_id: &str, component_id: &str, event: SupplyChainEvent) -> SupplyChainAuditEntry {
    SupplyChainAuditEntry::new(tick, tenant_id, component_id, event, "subject", true)
}
#[test]
fn for_component_returns_only_matching_entries() {
    let mut log = SupplyChainAuditLog::new();
    log.record(make_entry(1, "t1", "comp-a", SupplyChainEvent::ComponentAdded));
    log.record(make_entry(2, "t1", "comp-a", SupplyChainEvent::ComponentVerified));
    log.record(make_entry(3, "t1", "comp-b", SupplyChainEvent::ComponentAdded));
    assert_eq!(log.for_component("comp-a").len(), 2);
}
#[test]
fn for_component_empty_for_unknown() {
    let mut log = SupplyChainAuditLog::new();
    log.record(make_entry(1, "t1", "c1", SupplyChainEvent::ComponentAdded));
    assert!(log.for_component("unknown").is_empty());
}
