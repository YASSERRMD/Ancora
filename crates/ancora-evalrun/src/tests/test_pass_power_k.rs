use crate::executor::{exact_match, EvalCase, Executor, RunConfig, RunId};
use crate::passatk::{pass_at_k, pass_power_k, suite_pass_at_k, suite_pass_power_k};
use crate::rollout::RolloutRunner;

fn make_rollout_all_pass() -> crate::rollout::RolloutResult {
    let case = EvalCase {
        id: "c1".into(),
        input: "q".into(),
        expected: "a".into(),
    };
    let infer = |_: &str, _: u64| -> (String, u64, u64) { ("a".into(), 1, 1) };
    let config = RunConfig {
        run_id: RunId("r".into()),
        scorer: exact_match,
        seed: 0,
    };
    let executor = Executor::new(config);
    let runner = RolloutRunner::new(5);
    let mut rollouts = runner.rollout_suite(&executor, &[case], &infer);
    rollouts.remove(0)
}

fn make_rollout_half_pass() -> crate::rollout::RolloutResult {
    // Returns "a" for even seeds, "wrong" for odd seeds.
    let case = EvalCase {
        id: "c2".into(),
        input: "q".into(),
        expected: "a".into(),
    };
    let infer = |_: &str, seed: u64| -> (String, u64, u64) {
        let ans = if seed % 2 == 0 { "a" } else { "wrong" };
        (ans.into(), 1, 1)
    };
    let config = RunConfig {
        run_id: RunId("r".into()),
        scorer: exact_match,
        seed: 0,
    };
    let executor = Executor::new(config);
    let runner = RolloutRunner::new(10);
    let mut rollouts = runner.rollout_suite(&executor, &[case], &infer);
    rollouts.remove(0)
}

#[test]
fn pass_power_k_all_pass() {
    let r = make_rollout_all_pass();
    let ppk = pass_power_k(&r, 3);
    assert!(
        (ppk - 1.0).abs() < 1e-9,
        "all-pass -> pass^3 should be 1.0, got {}",
        ppk
    );
}

#[test]
fn pass_power_k_half_pass() {
    let r = make_rollout_half_pass();
    let ppk = pass_power_k(&r, 2);
    // Expected: (0.5)^2 = 0.25
    assert!(
        (ppk - 0.25).abs() < 0.01,
        "half-pass -> pass^2 ~ 0.25, got {}",
        ppk
    );
}

#[test]
fn pass_at_k_all_pass() {
    let r = make_rollout_all_pass();
    let pak = pass_at_k(&r, 3).expect("k <= n");
    assert!(
        (pak - 1.0).abs() < 1e-9,
        "pass@3 should be 1.0 for all-pass, got {}",
        pak
    );
}

#[test]
fn pass_at_k_none_when_k_exceeds_n() {
    let r = make_rollout_all_pass();
    assert!(pass_at_k(&r, 100).is_none(), "k > n should return None");
}

#[test]
fn suite_pass_power_k_computed() {
    let r1 = make_rollout_all_pass();
    let r2 = make_rollout_half_pass();
    let suite = vec![r1, r2];
    let ppk = suite_pass_power_k(&suite, 2);
    // Average of 1.0 and 0.25 = 0.625
    assert!(
        (ppk - 0.625).abs() < 0.05,
        "suite pass^2 ~ 0.625, got {}",
        ppk
    );
}

#[test]
fn suite_pass_at_k_computed() {
    let r = make_rollout_all_pass();
    let suite = vec![r];
    let pak = suite_pass_at_k(&suite, 2).expect("should compute");
    assert!(
        (pak - 1.0).abs() < 1e-9,
        "all-pass suite pass@2 = 1.0, got {}",
        pak
    );
}
