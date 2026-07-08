use crate::aggregate::{compute_aggregate, AggregateMetrics};
use crate::compare::{ci_overlap, compare_runs, CompareVerdict};
use crate::executor::{exact_match, EvalCase, Executor, RunConfig, RunId};
use crate::rollout::RolloutRunner;

fn make_metrics(pass_rate: f64, total_rollouts: usize) -> AggregateMetrics {
    let successes = (pass_rate * total_rollouts as f64).round() as usize;
    let (ci_lower, ci_upper) = crate::aggregate::wilson_interval(successes, total_rollouts, 1.96);
    AggregateMetrics {
        total_cases: 1,
        total_rollouts,
        pass_rate,
        ci_lower,
        ci_upper,
        mean_latency_ms: 10.0,
        p50_latency_ms: 10.0,
        p95_latency_ms: 15.0,
        total_cost_tokens: 100,
        mean_cost_tokens: 10.0,
    }
}

#[test]
fn run_comparison_highlights_improvement() {
    let m_a = make_metrics(0.60, 100);
    let m_b = make_metrics(0.80, 100);
    let delta = compare_runs(RunId("run-a".into()), &m_a, RunId("run-b".into()), &m_b);
    assert!(delta.pass_rate_delta > 0.0, "b is better than a");
    assert_eq!(delta.verdict(), CompareVerdict::Improved);
    assert!(delta.significant, "20pp improvement should be significant");
}

#[test]
fn run_comparison_highlights_regression() {
    let m_a = make_metrics(0.80, 100);
    let m_b = make_metrics(0.55, 100);
    let delta = compare_runs(RunId("run-a".into()), &m_a, RunId("run-b".into()), &m_b);
    assert!(delta.pass_rate_delta < 0.0, "b is worse than a");
    assert_eq!(delta.verdict(), CompareVerdict::Regressed);
}

#[test]
fn run_comparison_neutral_small_delta() {
    let m_a = make_metrics(0.700, 100);
    let m_b = make_metrics(0.705, 100);
    let delta = compare_runs(RunId("run-a".into()), &m_a, RunId("run-b".into()), &m_b);
    // 0.5pp delta < 1pp threshold - should be neutral.
    assert_eq!(delta.verdict(), CompareVerdict::Neutral);
}

#[test]
fn ci_overlap_detection() {
    // Overlapping intervals.
    assert!(ci_overlap(0.4, 0.6, 0.5, 0.7), "intervals should overlap");
    // Non-overlapping intervals.
    assert!(
        !ci_overlap(0.1, 0.3, 0.4, 0.6),
        "intervals should not overlap"
    );
}

#[test]
fn compare_summary_contains_run_ids() {
    let m_a = make_metrics(0.70, 50);
    let m_b = make_metrics(0.75, 50);
    let delta = compare_runs(RunId("run-001".into()), &m_a, RunId("run-002".into()), &m_b);
    let s = delta.summary();
    assert!(s.contains("run-001"), "summary should mention run-001");
    assert!(s.contains("run-002"), "summary should mention run-002");
}
