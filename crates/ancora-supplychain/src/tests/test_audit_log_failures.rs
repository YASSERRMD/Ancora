#[cfg(test)]
mod tests {
    use crate::audit::{SupplyChainAuditEntry, SupplyChainAuditLog, SupplyChainEvent};

    fn make_entry(tick: u64, success: bool) -> SupplyChainAuditEntry {
        SupplyChainAuditEntry::new(
            tick,
            "tenant-1",
            "comp-1",
            SupplyChainEvent::PolicyChecked,
            "subject",
            success,
        )
    }

    #[test]
    fn test_failures_returns_only_failed_entries() {
        let mut log = SupplyChainAuditLog::new();
        log.record(make_entry(1, true));
        log.record(make_entry(2, false));
        log.record(make_entry(3, false));
        log.record(make_entry(4, true));

        let failures = log.failures();
        assert_eq!(failures.len(), 2);
    }

    #[test]
    fn test_failures_returns_empty_when_all_succeed() {
        let mut log = SupplyChainAuditLog::new();
        log.record(make_entry(1, true));
        log.record(make_entry(2, true));

        assert!(log.failures().is_empty());
    }

    #[test]
    fn test_failures_returns_all_when_all_fail() {
        let mut log = SupplyChainAuditLog::new();
        log.record(make_entry(1, false));
        log.record(make_entry(2, false));
        log.record(make_entry(3, false));

        assert_eq!(log.failures().len(), 3);
    }

    #[test]
    fn test_failures_have_success_false() {
        let mut log = SupplyChainAuditLog::new();
        log.record(make_entry(1, false));

        let failures = log.failures();
        assert!(!failures[0].success);
    }
}
