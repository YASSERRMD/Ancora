use crate::{ResourceQuota, Tenant, TenantRegistry};
#[test]
fn suspended_tenants_lists_suspended_only() {
    let mut registry = TenantRegistry::new();
    registry
        .register(Tenant::new("t1", "A", 1), ResourceQuota::standard())
        .unwrap();
    registry
        .register(Tenant::new("t2", "B", 2), ResourceQuota::standard())
        .unwrap();
    registry.get_mut("t1").unwrap().suspend();
    let suspended = registry.suspended_tenants();
    assert_eq!(suspended.len(), 1);
    assert_eq!(suspended[0].id, "t1");
}
