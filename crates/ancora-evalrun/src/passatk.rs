/// Pass-at-k and pass-power-k metrics.
///
/// pass@k:  probability that at least 1 of k sampled rollouts passes.
/// pass^k:  probability that ALL k rollouts pass (consistency metric).
use crate::rollout::RolloutResult;

/// Pass-at-k estimate using the unbiased estimator:
/// pass@k = 1 - C(n-c, k) / C(n, k)
/// where n = total rollouts, c = passing rollouts.
///
/// Returns None if k > n (not enough rollouts to compute).
pub fn pass_at_k(rollout: &RolloutResult, k: usize) -> Option<f64> {
    let n = rollout.results.len();
    let c = rollout.pass_count();
    if k > n {
        return None;
    }
    // C(n-c, k) / C(n, k) computed in log space to avoid overflow.
    let numerator = log_comb(n - c, k);
    let denominator = log_comb(n, k);
    let prob_all_fail = (numerator - denominator).exp();
    Some(1.0 - prob_all_fail)
}

/// Pass-power-k: probability ALL k rollouts pass = (pass_rate)^k.
/// Uses empirical pass rate from rollouts.
pub fn pass_power_k(rollout: &RolloutResult, k: usize) -> f64 {
    rollout.pass_rate().powi(k as i32)
}

/// Compute log of binomial coefficient C(n, k).
fn log_comb(n: usize, k: usize) -> f64 {
    if k > n {
        return f64::NEG_INFINITY;
    }
    if k == 0 {
        return 0.0;
    }
    let k = k.min(n - k); // use symmetry
    (0..k)
        .fold(0.0_f64, |acc, i| acc + (n - i) as f64 / (i + 1) as f64)
        .ln()
}

/// Aggregate pass@k across a suite of rollouts (mean over cases).
pub fn suite_pass_at_k(rollouts: &[RolloutResult], k: usize) -> Option<f64> {
    let vals: Vec<f64> = rollouts.iter().filter_map(|r| pass_at_k(r, k)).collect();
    if vals.is_empty() {
        return None;
    }
    Some(vals.iter().sum::<f64>() / vals.len() as f64)
}

/// Aggregate pass-power-k across a suite of rollouts (mean over cases).
pub fn suite_pass_power_k(rollouts: &[RolloutResult], k: usize) -> f64 {
    let vals: Vec<f64> = rollouts.iter().map(|r| pass_power_k(r, k)).collect();
    if vals.is_empty() {
        return 0.0;
    }
    vals.iter().sum::<f64>() / vals.len() as f64
}

/// Per-case pass@k and pass-power-k summary.
#[derive(Debug, Clone)]
pub struct CasePassMetrics {
    pub case_id: String,
    pub pass_at_1: Option<f64>,
    pub pass_at_k: Option<f64>,
    pub pass_power_k: f64,
    pub k: usize,
}

/// Compute pass metrics for every case in the suite.
pub fn per_case_metrics(rollouts: &[RolloutResult], k: usize) -> Vec<CasePassMetrics> {
    rollouts
        .iter()
        .map(|r| CasePassMetrics {
            case_id: r.case_id.clone(),
            pass_at_1: pass_at_k(r, 1),
            pass_at_k: pass_at_k(r, k),
            pass_power_k: pass_power_k(r, k),
            k,
        })
        .collect()
}
