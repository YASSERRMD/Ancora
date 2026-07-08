#[cfg(test)]
mod tests {
    use crate::journal_scope::{cost_key, journal_key};
    use crate::{TenantConfig, TenantContext, TenantId, TenantRegistry};

    #[test]
    fn journal_keys_differ_across_tenants() {
        let mut reg = TenantRegistry::new();
        let a = reg.create("a", TenantConfig::default());
        let b = reg.create("b", TenantConfig::default());
        let ctx_a = TenantContext::new(a);
        let ctx_b = TenantContext::new(b);
        assert_ne!(journal_key(&ctx_a, "run-1"), journal_key(&ctx_b, "run-1"));
    }

    #[test]
    fn cost_keys_differ_across_tenants() {
        let mut reg = TenantRegistry::new();
        let a = reg.create("a", TenantConfig::default());
        let b = reg.create("b", TenantConfig::default());
        let ctx_a = TenantContext::new(a);
        let ctx_b = TenantContext::new(b);
        assert_ne!(cost_key(&ctx_a, "run-1"), cost_key(&ctx_b, "run-1"));
    }

    #[test]
    fn same_tenant_same_key() {
        let mut reg = TenantRegistry::new();
        let a = reg.create("a", TenantConfig::default());
        let ctx_a1 = TenantContext::new(a.clone());
        let ctx_a2 = TenantContext::new(a);
        assert_eq!(journal_key(&ctx_a1, "run-1"), journal_key(&ctx_a2, "run-1"));
    }
}
