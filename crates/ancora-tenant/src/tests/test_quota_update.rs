use crate::{QuotaUpdate, ResourceQuota};
#[test]
fn quota_update_applies_agent_change() {
    let mut quota = ResourceQuota::standard();
    let original = quota.max_agents;
    QuotaUpdate::new().agents(999).apply(&mut quota);
    assert_eq!(quota.max_agents, 999);
    assert_ne!(quota.max_agents, original);
}
#[test]
fn quota_update_leaves_unset_fields_unchanged() {
    let mut quota = ResourceQuota::standard();
    let original_tasks = quota.max_tasks;
    QuotaUpdate::new().agents(100).apply(&mut quota);
    assert_eq!(quota.max_tasks, original_tasks);
}
#[test]
fn quota_update_applies_multiple_fields() {
    let mut quota = ResourceQuota::standard();
    QuotaUpdate::new().agents(50).tasks(500).memory_mb(8192).apply(&mut quota);
    assert_eq!(quota.max_agents, 50);
    assert_eq!(quota.max_tasks, 500);
    assert_eq!(quota.max_memory_mb, 8192);
}
