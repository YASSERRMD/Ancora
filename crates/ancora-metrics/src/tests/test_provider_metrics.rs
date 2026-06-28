#[cfg(test)]
mod tests {
    use crate::provider_metrics::{ProviderErrorRate, TenantCostRate};

    #[test]
    fn provider_error_rate_computed() {
        let mut p = ProviderErrorRate::default();
        p.record_success("openai");
        p.record_success("openai");
        p.record_error("openai");
        let rate = p.error_rate("openai");
        assert!((rate - 1.0 / 3.0).abs() < 1e-9);
    }

    #[test]
    fn provider_error_rate_zero_when_all_success() {
        let mut p = ProviderErrorRate::default();
        p.record_success("openai");
        assert_eq!(p.error_rate("openai"), 0.0);
    }

    #[test]
    fn tenant_cost_rate_accumulates() {
        let mut c = TenantCostRate::default();
        c.add("t1", 0.01);
        c.add("t1", 0.02);
        assert!((c.total("t1") - 0.03).abs() < 1e-9);
    }

    #[test]
    fn tenant_cost_rate_is_tenant_scoped() {
        let mut c = TenantCostRate::default();
        c.add("a", 1.0);
        assert_eq!(c.total("b"), 0.0);
    }
}
