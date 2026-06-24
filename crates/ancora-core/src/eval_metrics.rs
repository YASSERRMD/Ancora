use crate::eval_runner::CaseResult;

/// pass@k: probability that at least one of k rollouts passes.
/// Computed as 1 - C(n-pass, k) / C(n, k) to avoid sampling bias.
pub fn pass_at_k(result: &CaseResult, k: usize) -> f64 {
    let n = result.n;
    let c = result.pass_count;
    if k > n {
        return if c > 0 { 1.0 } else { 0.0 };
    }
    if n == 0 || k == 0 {
        return 0.0;
    }
    let failures = n - c;
    if failures < k {
        return 1.0;
    }
    let numerator = falling_factorial(failures, k);
    let denominator = falling_factorial(n, k);
    1.0 - numerator as f64 / denominator as f64
}

/// pass^k: geometric mean of per-rollout scores raised to the k-th power.
/// Approximated as (pass_rate)^k.
pub fn pass_power_k(result: &CaseResult, k: usize) -> f64 {
    result.pass_rate().powi(k as i32)
}

/// Falling factorial n * (n-1) * ... * (n-k+1).
fn falling_factorial(n: usize, k: usize) -> u128 {
    (0..k).fold(1u128, |acc, i| acc * (n - i) as u128)
}

/// Aggregate pass@k over a suite of case results.
pub fn suite_pass_at_k(results: &[CaseResult], k: usize) -> f64 {
    if results.is_empty() {
        return 0.0;
    }
    let sum: f64 = results.iter().map(|r| pass_at_k(r, k)).sum();
    sum / results.len() as f64
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::eval_runner::CaseResult;

    fn make_result(n: usize, pass_count: usize) -> CaseResult {
        CaseResult {
            case_id: "c1".into(),
            rollouts: vec![],
            pass_count,
            n,
        }
    }

    #[test]
    fn pass_at_k_all_fail_returns_zero() {
        let r = make_result(5, 0);
        assert_eq!(pass_at_k(&r, 3), 0.0);
    }

    #[test]
    fn pass_at_k_all_pass_returns_one() {
        let r = make_result(5, 5);
        assert_eq!(pass_at_k(&r, 3), 1.0);
    }

    #[test]
    fn pass_at_k_k_exceeds_n_with_no_passes_returns_zero() {
        let r = make_result(3, 0);
        assert_eq!(pass_at_k(&r, 5), 0.0);
    }

    #[test]
    fn pass_at_k_partial_passes() {
        let r = make_result(4, 2);
        let p = pass_at_k(&r, 2);
        assert!(p > 0.0 && p < 1.0);
    }

    #[test]
    fn pass_power_k_full_pass_rate() {
        let r = make_result(5, 5);
        assert!((pass_power_k(&r, 2) - 1.0).abs() < 1e-9);
    }

    #[test]
    fn pass_power_k_zero_pass_rate() {
        let r = make_result(5, 0);
        assert_eq!(pass_power_k(&r, 2), 0.0);
    }

    #[test]
    fn suite_pass_at_k_empty_returns_zero() {
        assert_eq!(suite_pass_at_k(&[], 1), 0.0);
    }

    #[test]
    fn suite_pass_at_k_full_pass_suite() {
        let results = vec![make_result(5, 5), make_result(3, 3)];
        let s = suite_pass_at_k(&results, 2);
        assert!((s - 1.0).abs() < 1e-9);
    }

    #[test]
    fn pass_at_k_partial_decreases_with_k() {
        let r = make_result(10, 5);
        let p1 = pass_at_k(&r, 1);
        let p2 = pass_at_k(&r, 5);
        assert!(p1 <= p2);
    }
}
