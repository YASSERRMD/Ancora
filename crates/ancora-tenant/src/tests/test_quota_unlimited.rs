use crate::ResourceQuota;
#[test]
fn unlimited_quota_has_max_values() {
    let q = ResourceQuota::unlimited();
    assert_eq!(q.max_agents, u64::MAX);
    assert_eq!(q.max_tasks, u64::MAX);
    assert_eq!(q.max_memory_mb, u64::MAX);
}
