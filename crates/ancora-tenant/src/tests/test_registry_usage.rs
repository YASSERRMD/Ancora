use crate::{ResourceQuota, Tenant, TenantRegistry};
#[test]
fn registry_provides_mutable_usage() {
    let mut registry = TenantRegistry::new();
    registry
        .register(Tenant::new("t1", "A", 1), ResourceQuota::standard())
        .unwrap();
    let usage = registry.usage_mut("t1").unwrap();
    usage.agents = 5;
    usage.tasks = 20;
    let usage_read = registry.usage("t1").unwrap();
    assert_eq!(usage_read.agents, 5);
    assert_eq!(usage_read.tasks, 20);
}
