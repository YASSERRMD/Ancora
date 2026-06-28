use crate::metric::MetricScore;
use crate::report::EvalReport;

#[test]
fn eval_report_generated_with_scores() {
    let mut report = EvalReport::new("test-run", 42);
    report.add_score(MetricScore::new("planning", 0.8));
    report.add_score(MetricScore::new("reasoning", 0.9));
    assert_eq!(report.scores.len(), 2);
    let mean = report.mean_score();
    assert!((mean - 0.85).abs() < 1e-10);
}

#[test]
fn report_detects_regressions() {
    let mut report = EvalReport::new("test-run", 1);
    report.add_regression("planning");
    assert!(report.has_regressions());
    assert_eq!(report.regressions.len(), 1);
}

#[test]
fn report_summary_contains_name() {
    let report = EvalReport::new("my-eval", 5);
    let summary = report.summary();
    assert!(summary.contains("my-eval"));
}

#[test]
fn report_empty_mean_is_zero() {
    let report = EvalReport::new("empty", 0);
    assert_eq!(report.mean_score(), 0.0);
}
