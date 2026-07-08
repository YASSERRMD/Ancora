/// Aggregate metrics with confidence intervals across a full eval run.
use crate::rollout::RolloutResult;

/// Wilson score interval for a proportion.
/// Returns (lower, upper) at the given z (e.g. z=1.96 for 95%).
pub fn wilson_interval(successes: usize, trials: usize, z: f64) -> (f64, f64) {
    if trials == 0 {
        return (0.0, 1.0);
    }
    let n = trials as f64;
    let p_hat = successes as f64 / n;
    let z2 = z * z;
    let center = (p_hat + z2 / (2.0 * n)) / (1.0 + z2 / n);
    let margin = (z / (1.0 + z2 / n)) * ((p_hat * (1.0 - p_hat) / n + z2 / (4.0 * n * n)).sqrt());
    ((center - margin).max(0.0), (center + margin).min(1.0))
}

/// Aggregate statistics for an entire eval run.
#[derive(Debug, Clone)]
pub struct AggregateMetrics {
    pub total_cases: usize,
    pub total_rollouts: usize,
    pub pass_rate: f64,
    pub ci_lower: f64,
    pub ci_upper: f64,
    pub mean_latency_ms: f64,
    pub p50_latency_ms: f64,
    pub p95_latency_ms: f64,
    pub total_cost_tokens: u64,
    pub mean_cost_tokens: f64,
}

/// Compute aggregate metrics from a list of rollout results.
pub fn compute_aggregate(rollouts: &[RolloutResult]) -> AggregateMetrics {
    let total_cases = rollouts.len();
    let total_rollouts: usize = rollouts.iter().map(|r| r.results.len()).sum();
    let total_passes: usize = rollouts.iter().map(|r| r.pass_count()).sum();

    let pass_rate = if total_rollouts == 0 {
        0.0
    } else {
        total_passes as f64 / total_rollouts as f64
    };

    let (ci_lower, ci_upper) = wilson_interval(total_passes, total_rollouts, 1.96);

    // Collect all latencies for percentile computation.
    let mut latencies: Vec<f64> = rollouts
        .iter()
        .flat_map(|r| r.results.iter().map(|c| c.latency_ms as f64))
        .collect();
    latencies.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mean_latency_ms = if latencies.is_empty() {
        0.0
    } else {
        latencies.iter().sum::<f64>() / latencies.len() as f64
    };

    let p50_latency_ms = percentile(&latencies, 50.0);
    let p95_latency_ms = percentile(&latencies, 95.0);

    let total_cost_tokens: u64 = rollouts.iter().map(|r| r.total_cost_tokens()).sum();
    let mean_cost_tokens = if total_rollouts == 0 {
        0.0
    } else {
        total_cost_tokens as f64 / total_rollouts as f64
    };

    AggregateMetrics {
        total_cases,
        total_rollouts,
        pass_rate,
        ci_lower,
        ci_upper,
        mean_latency_ms,
        p50_latency_ms,
        p95_latency_ms,
        total_cost_tokens,
        mean_cost_tokens,
    }
}

/// Interpolated percentile from a sorted slice.
fn percentile(sorted: &[f64], p: f64) -> f64 {
    if sorted.is_empty() {
        return 0.0;
    }
    let idx = (p / 100.0 * (sorted.len() - 1) as f64).round() as usize;
    sorted[idx.min(sorted.len() - 1)]
}

/// Format aggregate metrics as a human-readable summary string.
pub fn summary_string(m: &AggregateMetrics) -> String {
    format!(
        "cases={} rollouts={} pass_rate={:.3} ci=[{:.3},{:.3}] \
         lat_mean={:.1}ms lat_p50={:.1}ms lat_p95={:.1}ms \
         cost_total={} cost_mean={:.1}",
        m.total_cases,
        m.total_rollouts,
        m.pass_rate,
        m.ci_lower,
        m.ci_upper,
        m.mean_latency_ms,
        m.p50_latency_ms,
        m.p95_latency_ms,
        m.total_cost_tokens,
        m.mean_cost_tokens,
    )
}
