/// Per-case breakdown of eval run results.

use crate::rollout::RolloutResult;

/// Detailed breakdown for a single eval case.
#[derive(Debug, Clone)]
pub struct CaseBreakdown {
    pub case_id: String,
    pub n_rollouts: usize,
    pub n_pass: usize,
    pub n_fail: usize,
    pub pass_rate: f64,
    pub mean_latency_ms: f64,
    pub total_cost_tokens: u64,
    pub fail_reasons: Vec<String>,
}

impl CaseBreakdown {
    pub fn from_rollout(r: &RolloutResult) -> Self {
        let n_rollouts = r.results.len();
        let n_pass = r.pass_count();
        let n_fail = r.fail_count();
        let pass_rate = r.pass_rate();
        let mean_latency_ms = r.mean_latency_ms();
        let total_cost_tokens = r.total_cost_tokens();

        let fail_reasons: Vec<String> = r
            .results
            .iter()
            .filter_map(|c| {
                if let crate::executor::Outcome::Fail { reason } = &c.outcome {
                    Some(reason.clone())
                } else {
                    None
                }
            })
            .collect();

        Self {
            case_id: r.case_id.clone(),
            n_rollouts,
            n_pass,
            n_fail,
            pass_rate,
            mean_latency_ms,
            total_cost_tokens,
            fail_reasons,
        }
    }
}

/// Compute per-case breakdowns for the entire suite.
pub fn compute_breakdown(rollouts: &[RolloutResult]) -> Vec<CaseBreakdown> {
    rollouts.iter().map(CaseBreakdown::from_rollout).collect()
}

/// Sort breakdown by pass rate ascending (worst cases first).
pub fn sort_by_pass_rate_asc(breakdowns: &mut Vec<CaseBreakdown>) {
    breakdowns.sort_by(|a, b| a.pass_rate.partial_cmp(&b.pass_rate).unwrap());
}

/// Sort breakdown by pass rate descending (best cases first).
pub fn sort_by_pass_rate_desc(breakdowns: &mut Vec<CaseBreakdown>) {
    breakdowns.sort_by(|a, b| b.pass_rate.partial_cmp(&a.pass_rate).unwrap());
}

/// Filter breakdowns to only include cases with pass_rate below a threshold.
pub fn filter_below_pass_rate(
    breakdowns: &[CaseBreakdown],
    threshold: f64,
) -> Vec<&CaseBreakdown> {
    breakdowns
        .iter()
        .filter(|b| b.pass_rate < threshold)
        .collect()
}

/// Compute overall pass rate from breakdowns.
pub fn overall_pass_rate(breakdowns: &[CaseBreakdown]) -> f64 {
    if breakdowns.is_empty() {
        return 0.0;
    }
    let total_pass: usize = breakdowns.iter().map(|b| b.n_pass).sum();
    let total: usize = breakdowns.iter().map(|b| b.n_rollouts).sum();
    if total == 0 {
        0.0
    } else {
        total_pass as f64 / total as f64
    }
}
