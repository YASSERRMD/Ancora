use crate::{ResourceQuota, Tenant, TenantRegistry};
#[test]
fn register_tenant_succeeds() {
    let mut registry = TenantRegistry::new();
    let t = Tenant::new("t1", "Acme", 1);
    let result = registry.register(t, ResourceQuota::standard());
    assert!(result.is_ok());
    assert!(registry.get("t1").is_some());
}
#[test]
fn registry_count_increments() {
    let mut registry = TenantRegistry::new();
    registry.register(Tenant::new("t1", "A", 1), ResourceQuota::standard()).unwrap();
    registry.register(Tenant::new("t2", "B", 2), ResourceQuota::standard()).unwrap();
    assert_eq!(registry.count(), 2);
}
