use crate::executor::{EvalCase, Executor, RunConfig, RunId, exact_match, fixture_infer};
use crate::rollout::RolloutRunner;
use crate::aggregate::compute_aggregate;

#[test]
fn eval_run_completes_on_fixture_suite() {
    let cases = vec![
        EvalCase { id: "c1".into(), input: "2+2".into(), expected: "4".into() },
        EvalCase { id: "c2".into(), input: "3+3".into(), expected: "6".into() },
        EvalCase { id: "c3".into(), input: "hello".into(), expected: "world".into() },
    ];

    let mut answers = std::collections::HashMap::new();
    answers.insert("2+2".to_string(), "4".to_string());
    answers.insert("3+3".to_string(), "6".to_string());
    answers.insert("hello".to_string(), "world".to_string());
    let infer = fixture_infer(answers);

    let config = RunConfig {
        run_id: RunId("test-run-1".into()),
        scorer: exact_match,
        seed: 42,
    };
    let executor = Executor::new(config);
    let runner = RolloutRunner::new(3);
    let rollouts = runner.rollout_suite(&executor, &cases, &infer);

    assert_eq!(rollouts.len(), 3);
    for r in &rollouts {
        assert_eq!(r.results.len(), 3);
        assert_eq!(r.pass_count(), 3, "all rollouts should pass for case {}", r.case_id);
    }

    let metrics = compute_aggregate(&rollouts);
    assert!((metrics.pass_rate - 1.0).abs() < 1e-9, "expected 100% pass rate");
    assert!(metrics.ci_lower > 0.6, "ci lower should be high for 9 total rollouts all passing");
}

#[test]
fn eval_run_records_failures() {
    let cases = vec![
        EvalCase { id: "f1".into(), input: "bad".into(), expected: "good".into() },
    ];

    let infer = |_: &str, _: u64| -> (String, u64, u64) { ("wrong".into(), 5, 2) };

    let config = RunConfig {
        run_id: RunId("fail-run".into()),
        scorer: exact_match,
        seed: 0,
    };
    let executor = Executor::new(config);
    let runner = RolloutRunner::new(2);
    let rollouts = runner.rollout_suite(&executor, &cases, &infer);

    assert_eq!(rollouts[0].pass_count(), 0);
    assert_eq!(rollouts[0].fail_count(), 2);
    let metrics = compute_aggregate(&rollouts);
    assert!((metrics.pass_rate).abs() < 1e-9, "expected 0% pass rate");
}
