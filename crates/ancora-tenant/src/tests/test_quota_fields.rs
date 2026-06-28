use crate::ResourceQuota;
#[test]
fn quota_new_stores_all_fields() {
    let q = ResourceQuota::new(5, 50, 2048, 2000, 25, 50_000);
    assert_eq!(q.max_agents, 5);
    assert_eq!(q.max_tasks, 50);
    assert_eq!(q.max_memory_mb, 2048);
    assert_eq!(q.max_cpu_millicores, 2000);
    assert_eq!(q.max_secrets, 25);
    assert_eq!(q.max_log_entries, 50_000);
}
