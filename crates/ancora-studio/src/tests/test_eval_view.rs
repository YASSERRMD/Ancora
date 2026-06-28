use crate::eval_view::{EvalView, EvalResult, EvalOutcome};

fn make_result(name: &str, run_id: &str, outcome: EvalOutcome) -> EvalResult {
    EvalResult {
        eval_name: name.into(),
        run_id: run_id.into(),
        outcome,
        reason: None,
        latency_ms: 10,
    }
}

#[test]
fn test_eval_view_renders_results() {
    let view = EvalView::new(vec![
        make_result("precision", "r1", EvalOutcome::Pass),
        make_result("recall", "r1", EvalOutcome::Pass),
        make_result("f1", "r1", EvalOutcome::Fail),
    ]);
    assert_eq!(view.results().len(), 3);
}

#[test]
fn test_eval_pass_rate_two_thirds() {
    let view = EvalView::new(vec![
        make_result("a", "r1", EvalOutcome::Pass),
        make_result("b", "r1", EvalOutcome::Pass),
        make_result("c", "r1", EvalOutcome::Fail),
    ]);
    let rate = view.pass_rate();
    assert!((rate - 2.0 / 3.0).abs() < 1e-9);
}

#[test]
fn test_eval_skip_excluded_from_pass_rate() {
    let view = EvalView::new(vec![
        make_result("a", "r1", EvalOutcome::Pass),
        make_result("b", "r1", EvalOutcome::Skip),
    ]);
    assert!((view.pass_rate() - 1.0).abs() < 1e-9);
}

#[test]
fn test_eval_partial_score() {
    let view = EvalView::new(vec![
        make_result("a", "r1", EvalOutcome::Partial { score: 0.75 }),
    ]);
    let avg = view.average_score().unwrap();
    assert!((avg - 0.75).abs() < 1e-9);
}
