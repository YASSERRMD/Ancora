use crate::{QuotaSummary, ResourceQuota, ResourceUsage};
#[test]
fn quota_summary_computes_utilization() {
    let quota = ResourceQuota::new(10, 100, 4096, 4000, 50, 100_000);
    let usage = ResourceUsage {
        agents: 5,
        tasks: 25,
        ..Default::default()
    };
    let summary = QuotaSummary::from(&usage, &quota);
    assert!((summary.agent_utilization_pct - 50.0).abs() < 1e-10);
    assert!((summary.task_utilization_pct - 25.0).abs() < 1e-10);
}
