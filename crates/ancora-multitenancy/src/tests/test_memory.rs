#[cfg(test)]
mod tests {
    use crate::memory_scope::{memory_key, memory_namespace};
    use crate::{TenantConfig, TenantContext, TenantRegistry};

    #[test]
    fn memory_namespaces_differ_across_tenants() {
        let mut reg = TenantRegistry::new();
        let a = reg.create("a", TenantConfig::default());
        let b = reg.create("b", TenantConfig::default());
        assert_ne!(
            memory_namespace(&TenantContext::new(a)),
            memory_namespace(&TenantContext::new(b))
        );
    }

    #[test]
    fn memory_keys_differ_across_tenants() {
        let mut reg = TenantRegistry::new();
        let a = reg.create("a", TenantConfig::default());
        let b = reg.create("b", TenantConfig::default());
        assert_ne!(
            memory_key(&TenantContext::new(a), "session"),
            memory_key(&TenantContext::new(b), "session")
        );
    }
}
