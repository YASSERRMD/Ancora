use crate::ResourceQuota;
#[test]
fn standard_quota_has_sensible_limits() {
    let q = ResourceQuota::standard();
    assert!(q.max_agents > 0 && q.max_agents < u64::MAX);
    assert!(q.max_tasks > 0);
    assert!(q.max_memory_mb > 0);
}
