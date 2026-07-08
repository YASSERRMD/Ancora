use crate::aggregate::{compute_aggregate, wilson_interval};
use crate::executor::{exact_match, EvalCase, Executor, RunConfig, RunId};
use crate::rollout::RolloutRunner;

#[test]
fn wilson_interval_contains_true_rate() {
    // 70 successes out of 100 trials, 95% CI should contain 0.70.
    let (lo, hi) = wilson_interval(70, 100, 1.96);
    assert!(
        lo <= 0.70 && 0.70 <= hi,
        "CI [{:.4},{:.4}] should contain 0.70",
        lo,
        hi
    );
}

#[test]
fn wilson_interval_extreme_zero() {
    let (lo, hi) = wilson_interval(0, 50, 1.96);
    assert!(
        lo >= 0.0 && hi <= 1.0,
        "CI must be in [0,1]: [{:.4},{:.4}]",
        lo,
        hi
    );
    assert!(lo < hi, "interval should be non-trivial");
}

#[test]
fn wilson_interval_extreme_full() {
    let (lo, hi) = wilson_interval(100, 100, 1.96);
    assert!(
        lo > 0.9,
        "full-pass lower bound should be > 0.9, got {:.4}",
        lo
    );
    assert!(hi <= 1.0, "upper bound must be <= 1.0");
}

#[test]
fn wilson_interval_empty_trials() {
    let (lo, hi) = wilson_interval(0, 0, 1.96);
    assert_eq!(lo, 0.0);
    assert_eq!(hi, 1.0);
}

#[test]
fn aggregate_ci_computed() {
    let cases = vec![
        EvalCase {
            id: "c1".into(),
            input: "x".into(),
            expected: "y".into(),
        },
        EvalCase {
            id: "c2".into(),
            input: "a".into(),
            expected: "b".into(),
        },
    ];
    let infer = |input: &str, _: u64| -> (String, u64, u64) {
        let out = match input {
            "x" => "y",
            _ => "wrong",
        };
        (out.into(), 20, 10)
    };
    let config = RunConfig {
        run_id: RunId("ci-test".into()),
        scorer: exact_match,
        seed: 1,
    };
    let executor = Executor::new(config);
    let runner = RolloutRunner::new(10);
    let rollouts = runner.rollout_suite(&executor, &cases, &infer);
    let metrics = compute_aggregate(&rollouts);

    // 10 pass, 10 fail = 0.5 pass rate.
    assert!(
        (metrics.pass_rate - 0.5).abs() < 0.01,
        "pass_rate should be ~0.5, got {:.3}",
        metrics.pass_rate
    );
    assert!(metrics.ci_lower < 0.5, "lower CI bound should be < 0.5");
    assert!(metrics.ci_upper > 0.5, "upper CI bound should be > 0.5");
    assert!(
        metrics.ci_lower >= 0.0 && metrics.ci_upper <= 1.0,
        "CI must be in [0,1]"
    );
}
