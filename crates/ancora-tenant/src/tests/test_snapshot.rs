use crate::{ResourceQuota, ResourceUsage, Tenant, TenantSnapshot};
#[test]
fn snapshot_captures_usage_and_quota() {
    let tenant = Tenant::new("t1", "A", 1);
    let usage = ResourceUsage { agents: 3, tasks: 15, ..Default::default() };
    let quota = ResourceQuota::standard();
    let snap = TenantSnapshot::capture(100, &tenant, &usage, &quota);
    assert_eq!(snap.agents, 3);
    assert_eq!(snap.tasks, 15);
    assert_eq!(snap.max_agents, quota.max_agents);
}
#[test]
fn snapshot_agent_headroom() {
    let tenant = Tenant::new("t1", "A", 1);
    let usage = ResourceUsage { agents: 4, ..Default::default() };
    let quota = ResourceQuota::new(10, 100, 4096, 4000, 50, 100_000);
    let snap = TenantSnapshot::capture(50, &tenant, &usage, &quota);
    assert_eq!(snap.agent_headroom(), 6);
}
#[test]
fn snapshot_near_limit_detection() {
    let tenant = Tenant::new("t1", "A", 1);
    let usage = ResourceUsage { agents: 9, ..Default::default() };
    let quota = ResourceQuota::new(10, 100, 4096, 4000, 50, 100_000);
    let snap = TenantSnapshot::capture(75, &tenant, &usage, &quota);
    assert!(snap.is_near_agent_limit(0.85));
    assert!(!snap.is_near_agent_limit(0.95));
}
