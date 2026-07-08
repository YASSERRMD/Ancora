use crate::eval_e2e::{default_eval_suite, run_eval_suite, EvalCase, ExactMatchJudge};

#[test]
fn eval_run_scores_a_suite() {
    let suite = default_eval_suite();
    let result = run_eval_suite("suite-001", &suite);

    assert!(!result.scores.is_empty(), "result must contain scores");
    assert_eq!(result.pass_rate(), 1.0, "default suite must pass 100%");
}

#[test]
fn eval_result_mean_score_is_correct() {
    let suite = default_eval_suite();
    let result = run_eval_suite("suite-002", &suite);
    assert!((result.mean_score() - 1.0).abs() < f64::EPSILON);
}

#[test]
fn eval_failing_case_reduces_pass_rate() {
    let cases = vec![
        (EvalCase::new("f1", "q1", "ans1"), "ans1".to_string()),
        (EvalCase::new("f2", "q2", "ans2"), "wrong".to_string()),
    ];
    let result = run_eval_suite("suite-fail", &cases);
    assert_eq!(result.scores.len(), 2);
    assert!(!result.all_passed());
    assert!((result.pass_rate() - 0.5).abs() < f64::EPSILON);
}

#[test]
fn exact_match_judge_is_case_sensitive() {
    let judge = ExactMatchJudge;
    let case = EvalCase::new("j1", "q", "Paris");
    let pass_score = judge.score(&case, "Paris");
    let fail_score = judge.score(&case, "paris");
    assert!(pass_score.passed);
    assert!(!fail_score.passed);
}

#[test]
fn exact_match_judge_trims_whitespace() {
    let judge = ExactMatchJudge;
    let case = EvalCase::new("j2", "q", "hello");
    let score = judge.score(&case, "  hello  ");
    assert!(score.passed, "judge must trim whitespace before comparing");
}
