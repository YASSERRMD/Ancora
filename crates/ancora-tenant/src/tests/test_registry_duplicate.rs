use crate::{ResourceQuota, Tenant, TenantRegistry};
#[test]
fn duplicate_registration_fails() {
    let mut registry = TenantRegistry::new();
    registry
        .register(Tenant::new("t1", "Acme", 1), ResourceQuota::standard())
        .unwrap();
    let result = registry.register(Tenant::new("t1", "Acme2", 2), ResourceQuota::standard());
    assert!(result.is_err());
}
