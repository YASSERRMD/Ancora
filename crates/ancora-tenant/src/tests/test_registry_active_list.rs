use crate::{ResourceQuota, Tenant, TenantRegistry};
#[test]
fn active_tenants_excludes_suspended() {
    let mut registry = TenantRegistry::new();
    registry
        .register(Tenant::new("t1", "A", 1), ResourceQuota::standard())
        .unwrap();
    registry
        .register(Tenant::new("t2", "B", 2), ResourceQuota::standard())
        .unwrap();
    registry.get_mut("t2").unwrap().suspend();
    let active = registry.active_tenants();
    assert_eq!(active.len(), 1);
    assert_eq!(active[0].id, "t1");
}
