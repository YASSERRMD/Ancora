#[cfg(test)]
mod tests {
    use crate::{TenantConfig, TenantId, TenantIsolation, TenantRegistry};

    fn registry_with_two_tenants() -> (TenantRegistry, TenantId, TenantId) {
        let mut reg = TenantRegistry::new();
        let a = reg.create("tenant-a", TenantConfig::default());
        let b = reg.create("tenant-b", TenantConfig::default());
        (reg, a, b)
    }

    #[test]
    fn cross_tenant_access_denied() {
        let (reg, a, b) = registry_with_two_tenants();
        let iso = TenantIsolation::new(&reg);
        assert!(iso.assert_owns(&a, &b).is_err());
    }

    #[test]
    fn same_tenant_access_allowed() {
        let (reg, a, _b) = registry_with_two_tenants();
        let iso = TenantIsolation::new(&reg);
        assert!(iso.assert_owns(&a, &a).is_ok());
    }

    #[test]
    fn active_tenant_asserted_ok() {
        let (reg, a, _) = registry_with_two_tenants();
        let iso = TenantIsolation::new(&reg);
        assert!(iso.assert_active(&a).is_ok());
    }

    #[test]
    fn unknown_tenant_returns_not_found() {
        let reg = TenantRegistry::new();
        let iso = TenantIsolation::new(&reg);
        let unknown = TenantId::from_raw("ghost");
        assert!(iso.assert_active(&unknown).is_err());
    }
}
