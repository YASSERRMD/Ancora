use serde::{Deserialize, Serialize};

/// Snapshot of runtime metrics used to drive scaling decisions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoscaleMetrics {
    pub queue_depth: usize,
    pub worker_count: usize,
    pub active_runs: usize,
    pub concurrency_per_worker: usize,
    /// Latency of the last completed run in milliseconds.
    pub last_run_latency_ms: u64,
    /// Fraction of total capacity currently in use (0.0 to 1.0).
    pub utilization: f64,
}

impl AutoscaleMetrics {
    pub fn compute_utilization(active: usize, workers: usize, concurrency: usize) -> f64 {
        let capacity = workers * concurrency;
        if capacity == 0 {
            return 1.0;
        }
        active as f64 / capacity as f64
    }
}

/// Tenant-scoped metrics for per-tenant scaling caps.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantMetrics {
    pub tenant_id: String,
    pub queue_depth: usize,
    pub active_runs: usize,
    pub worker_cap: usize,
}
