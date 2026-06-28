use crate::{ResourceQuota, Tenant, TenantRegistry};
#[test]
fn require_active_returns_tenant_when_active() {
    let mut registry = TenantRegistry::new();
    registry.register(Tenant::new("t1", "A", 1), ResourceQuota::standard()).unwrap();
    assert!(registry.require_active("t1").is_ok());
}
#[test]
fn require_active_fails_when_suspended() {
    let mut registry = TenantRegistry::new();
    registry.register(Tenant::new("t1", "A", 1), ResourceQuota::standard()).unwrap();
    registry.get_mut("t1").unwrap().suspend();
    assert!(registry.require_active("t1").is_err());
}
