use crate::{TenantBuilder, ResourceQuota};
#[test]
fn builder_produces_active_tenant_with_standard_quota() {
    let (tenant, quota) = TenantBuilder::new("t1", "Acme", 100).build();
    assert_eq!(tenant.id, "t1");
    assert_eq!(tenant.name, "Acme");
    assert!(tenant.is_active());
    assert_eq!(quota.max_agents, ResourceQuota::standard().max_agents);
}
#[test]
fn builder_attaches_metadata() {
    let (tenant, _) = TenantBuilder::new("t2", "Beta", 200)
        .metadata("plan", "starter")
        .build();
    assert_eq!(tenant.metadata.get("plan").unwrap(), "starter");
}
#[test]
fn builder_accepts_custom_quota() {
    let custom_quota = ResourceQuota::restricted();
    let (_, quota) = TenantBuilder::new("t3", "Gamma", 300)
        .quota(custom_quota)
        .build();
    assert_eq!(quota.max_agents, ResourceQuota::restricted().max_agents);
}
