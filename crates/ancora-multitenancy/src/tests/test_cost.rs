#[cfg(test)]
mod tests {
    use crate::{CostLedger, TenantConfig, TenantId, TenantRegistry};

    #[test]
    fn cost_attributed_to_correct_tenant() {
        let mut reg = TenantRegistry::new();
        let a = reg.create("a", TenantConfig::default());
        let b = reg.create("b", TenantConfig::default());
        let mut ledger = CostLedger::new();
        ledger.record(&a, 1.5);
        ledger.record(&b, 3.0);
        assert!((ledger.total_for(&a) - 1.5).abs() < 1e-9);
        assert!((ledger.total_for(&b) - 3.0).abs() < 1e-9);
    }

    #[test]
    fn cost_accumulates_for_same_tenant() {
        let mut reg = TenantRegistry::new();
        let a = reg.create("a", TenantConfig::default());
        let mut ledger = CostLedger::new();
        ledger.record(&a, 1.0);
        ledger.record(&a, 2.0);
        assert!((ledger.total_for(&a) - 3.0).abs() < 1e-9);
    }

    #[test]
    fn unknown_tenant_cost_is_zero() {
        let ledger = CostLedger::new();
        let ghost = TenantId::from_str("ghost");
        assert_eq!(ledger.total_for(&ghost), 0.0);
    }
}
