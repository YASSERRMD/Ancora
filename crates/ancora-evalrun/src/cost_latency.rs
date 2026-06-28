/// Cost and latency analysis per eval run.

use crate::rollout::RolloutResult;

/// Cost and latency statistics for an eval run.
#[derive(Debug, Clone)]
pub struct CostLatencyStats {
    pub total_tokens: u64,
    pub mean_tokens_per_rollout: f64,
    pub max_tokens_per_rollout: u64,
    pub min_tokens_per_rollout: u64,
    pub mean_latency_ms: f64,
    pub max_latency_ms: u64,
    pub min_latency_ms: u64,
    pub p50_latency_ms: f64,
    pub p95_latency_ms: f64,
    pub p99_latency_ms: f64,
}

/// Compute cost and latency stats from a list of rollout results.
pub fn compute_cost_latency(rollouts: &[RolloutResult]) -> CostLatencyStats {
    let all_results: Vec<_> = rollouts
        .iter()
        .flat_map(|r| r.results.iter())
        .collect();

    if all_results.is_empty() {
        return CostLatencyStats {
            total_tokens: 0,
            mean_tokens_per_rollout: 0.0,
            max_tokens_per_rollout: 0,
            min_tokens_per_rollout: 0,
            mean_latency_ms: 0.0,
            max_latency_ms: 0,
            min_latency_ms: 0,
            p50_latency_ms: 0.0,
            p95_latency_ms: 0.0,
            p99_latency_ms: 0.0,
        };
    }

    let n = all_results.len();
    let total_tokens: u64 = all_results.iter().map(|r| r.cost_tokens).sum();
    let mean_tokens_per_rollout = total_tokens as f64 / n as f64;
    let max_tokens_per_rollout = all_results.iter().map(|r| r.cost_tokens).max().unwrap_or(0);
    let min_tokens_per_rollout = all_results.iter().map(|r| r.cost_tokens).min().unwrap_or(0);

    let mean_latency_ms = all_results.iter().map(|r| r.latency_ms as f64).sum::<f64>() / n as f64;
    let max_latency_ms = all_results.iter().map(|r| r.latency_ms).max().unwrap_or(0);
    let min_latency_ms = all_results.iter().map(|r| r.latency_ms).min().unwrap_or(0);

    let mut latencies: Vec<f64> = all_results.iter().map(|r| r.latency_ms as f64).collect();
    latencies.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let p50_latency_ms = percentile(&latencies, 50.0);
    let p95_latency_ms = percentile(&latencies, 95.0);
    let p99_latency_ms = percentile(&latencies, 99.0);

    CostLatencyStats {
        total_tokens,
        mean_tokens_per_rollout,
        max_tokens_per_rollout,
        min_tokens_per_rollout,
        mean_latency_ms,
        max_latency_ms,
        min_latency_ms,
        p50_latency_ms,
        p95_latency_ms,
        p99_latency_ms,
    }
}

fn percentile(sorted: &[f64], p: f64) -> f64 {
    if sorted.is_empty() {
        return 0.0;
    }
    let idx = (p / 100.0 * (sorted.len() - 1) as f64).round() as usize;
    sorted[idx.min(sorted.len() - 1)]
}

/// Cost budget check: returns true if cost is within budget tokens.
pub fn within_budget(stats: &CostLatencyStats, budget_tokens: u64) -> bool {
    stats.total_tokens <= budget_tokens
}

/// Latency SLA check: returns true if p95 latency is within the SLA in ms.
pub fn meets_latency_sla(stats: &CostLatencyStats, sla_ms: f64) -> bool {
    stats.p95_latency_ms <= sla_ms
}
