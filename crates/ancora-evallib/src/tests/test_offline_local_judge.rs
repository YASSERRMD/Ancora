use crate::runner::{run_offline_eval, EvalReport};

#[test]
fn offline_eval_runs_without_network() {
    let report: EvalReport = run_offline_eval();
    assert!(!report.suite_results.is_empty(), "report should contain suite results");
}

#[test]
fn offline_eval_all_suites_pass() {
    let report = run_offline_eval();
    for suite in &report.suite_results {
        assert!(
            suite.all_passed(),
            "suite '{}' failed: {}/{} passed",
            suite.name, suite.passed, suite.total
        );
    }
}

#[test]
fn offline_eval_overall_pass_rate_is_100_percent() {
    let report = run_offline_eval();
    let rate = report.overall_pass_rate();
    assert!(
        (rate - 1.0).abs() < 1e-9,
        "overall pass rate should be 1.0, got {}",
        rate
    );
}

#[test]
fn eval_report_totals_are_consistent() {
    let report = run_offline_eval();
    let sum_passed: usize = report.suite_results.iter().map(|r| r.passed).sum();
    let sum_total: usize = report.suite_results.iter().map(|r| r.total).sum();
    assert_eq!(report.total_passed(), sum_passed);
    assert_eq!(report.total_cases(), sum_total);
    assert!(sum_total > 0, "at least one case must be registered");
}
