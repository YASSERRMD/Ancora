use crate::aggregate::compute_aggregate;
use crate::breakdown::compute_breakdown;
use crate::executor::{exact_match, fixture_infer, EvalCase, Executor, RunConfig, RunId};
use crate::report::EvalReport;
use crate::rollout::RolloutRunner;

fn build_report() -> EvalReport {
    let cases = vec![
        EvalCase {
            id: "r1".into(),
            input: "ping".into(),
            expected: "pong".into(),
        },
        EvalCase {
            id: "r2".into(),
            input: "a".into(),
            expected: "b".into(),
        },
    ];
    let mut answers = std::collections::HashMap::new();
    answers.insert("ping".to_string(), "pong".to_string());
    answers.insert("a".to_string(), "b".to_string());
    let infer = fixture_infer(answers);

    let config = RunConfig {
        run_id: RunId("report-run".into()),
        scorer: exact_match,
        seed: 99,
    };
    let executor = Executor::new(config);
    let runner = RolloutRunner::new(2);
    let rollouts = runner.rollout_suite(&executor, &cases, &infer);
    let metrics = compute_aggregate(&rollouts);
    let breakdowns = compute_breakdown(&rollouts);

    EvalReport::new(
        RunId("report-run".into()),
        "fixture-suite".into(),
        1_000_000,
        metrics,
        breakdowns,
    )
}

#[test]
fn report_json_generated() {
    let report = build_report();
    let json = report.to_json();
    assert!(json.contains("report-run"), "JSON should contain run_id");
    assert!(
        json.contains("fixture-suite"),
        "JSON should contain suite_name"
    );
    assert!(
        json.contains("pass_rate"),
        "JSON should contain pass_rate field"
    );
    assert!(json.contains("cases"), "JSON should contain cases array");
}

#[test]
fn report_html_generated() {
    let report = build_report();
    let html = report.to_html();
    assert!(
        html.contains("<!DOCTYPE html>"),
        "HTML should start with doctype"
    );
    assert!(
        html.contains("fixture-suite"),
        "HTML should contain suite name"
    );
    assert!(html.contains("report-run"), "HTML should contain run ID");
    assert!(html.contains("<table"), "HTML should contain a table");
}

#[test]
fn report_metrics_are_correct() {
    let report = build_report();
    assert!(
        (report.metrics.pass_rate - 1.0).abs() < 1e-9,
        "all cases pass in fixture"
    );
    assert_eq!(report.breakdowns.len(), 2, "two cases in suite");
}

#[test]
fn report_html_contains_case_rows() {
    let report = build_report();
    let html = report.to_html();
    assert!(html.contains("r1"), "HTML should contain case r1");
    assert!(html.contains("r2"), "HTML should contain case r2");
}
