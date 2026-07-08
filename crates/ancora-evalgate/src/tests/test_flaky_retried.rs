use crate::flaky::{evaluate_with_retry, FlakyOutcome, FlakyPolicy};
use crate::regression::RegressionResult;

#[test]
fn flaky_eval_majority_pass_is_not_blocked() {
    // 3 runs: only 1 regresses - minority, so gate passes
    let policy = FlakyPolicy::new(2, 0.5);
    let mut call_count = 0usize;
    let outcome = evaluate_with_retry(&policy, |_| {
        let r = if call_count == 0 {
            RegressionResult::Regression {
                delta: 0.15,
                threshold: 0.02,
            }
        } else {
            RegressionResult::Improvement { delta: -0.01 }
        };
        call_count += 1;
        r
    });
    assert!(
        matches!(outcome, FlakyOutcome::Passed { .. }),
        "1/3 blocking runs should not fail the gate"
    );
}

#[test]
fn flaky_eval_majority_block_fails() {
    // 3 runs: 2 regress - majority, so gate fails
    let policy = FlakyPolicy::new(2, 0.5);
    let mut call_count = 0usize;
    let outcome = evaluate_with_retry(&policy, |_| {
        let r = if call_count < 2 {
            RegressionResult::Regression {
                delta: 0.15,
                threshold: 0.02,
            }
        } else {
            RegressionResult::Improvement { delta: -0.01 }
        };
        call_count += 1;
        r
    });
    assert!(matches!(outcome, FlakyOutcome::Failed { .. }));
}
