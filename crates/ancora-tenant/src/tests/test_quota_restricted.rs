use crate::ResourceQuota;
#[test]
fn restricted_quota_is_lower_than_standard() {
    let standard = ResourceQuota::standard();
    let restricted = ResourceQuota::restricted();
    assert!(restricted.max_agents < standard.max_agents);
    assert!(restricted.max_tasks < standard.max_tasks);
    assert!(restricted.max_memory_mb < standard.max_memory_mb);
}
