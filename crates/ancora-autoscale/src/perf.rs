use crate::metrics::AutoscaleMetrics;
use crate::policy::ScalePolicy;
use std::time::Instant;

/// Measure autoscaling decision latency in microseconds.
pub fn measure_decision_latency(policy: &mut ScalePolicy, iterations: usize) -> u64 {
    let m = AutoscaleMetrics {
        queue_depth: 10,
        worker_count: 4,
        active_runs: 3,
        concurrency_per_worker: 4,
        last_run_latency_ms: 100,
        utilization: 0.75,
    };
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = policy.evaluate(&m);
    }
    let elapsed = start.elapsed().as_micros() as u64;
    elapsed / iterations as u64
}
