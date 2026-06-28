#[cfg(test)]
mod tests {
    use crate::audit::{SupplyChainAuditEntry, SupplyChainAuditLog, SupplyChainEvent};

    fn make_entry(tick: u64, tenant_id: &str, component_id: &str) -> SupplyChainAuditEntry {
        SupplyChainAuditEntry::new(
            tick,
            tenant_id,
            component_id,
            SupplyChainEvent::ComponentAdded,
            "subject",
            true,
        )
    }

    #[test]
    fn test_for_tenant_returns_only_matching_entries() {
        let mut log = SupplyChainAuditLog::new();
        log.record(make_entry(1, "tenant-a", "comp-1"));
        log.record(make_entry(2, "tenant-a", "comp-2"));
        log.record(make_entry(3, "tenant-b", "comp-3"));

        let entries = log.for_tenant("tenant-a");
        assert_eq!(entries.len(), 2);
    }

    #[test]
    fn test_for_tenant_returns_empty_for_unknown_tenant() {
        let mut log = SupplyChainAuditLog::new();
        log.record(make_entry(1, "tenant-a", "comp-1"));

        assert!(log.for_tenant("tenant-z").is_empty());
    }

    #[test]
    fn test_for_tenant_does_not_include_other_tenants() {
        let mut log = SupplyChainAuditLog::new();
        log.record(make_entry(1, "tenant-a", "comp-1"));
        log.record(make_entry(2, "tenant-b", "comp-2"));

        let entries = log.for_tenant("tenant-b");
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].tenant_id, "tenant-b");
    }
}
