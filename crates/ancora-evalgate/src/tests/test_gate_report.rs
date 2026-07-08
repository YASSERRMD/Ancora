use crate::gate::GateDecision;
use crate::gate::MetricVerdict;
use crate::regression::RegressionResult;
use crate::report::GateReport;

#[test]
fn gate_report_contains_dataset_and_status() {
    let verdicts = vec![MetricVerdict {
        metric: "accuracy".to_string(),
        regression: RegressionResult::Improvement { delta: -0.01 },
        significant: false,
        blocks: false,
    }];
    let report = GateReport::new("mmlu", GateDecision::Pass, verdicts);
    let md = report.to_markdown();
    assert!(md.contains("mmlu"), "report should mention dataset name");
    assert!(md.contains("PASS"), "report should show PASS status");
    assert!(md.contains("accuracy"), "report should list the metric");
}

#[test]
fn gate_report_fail_contains_blocking_note() {
    let verdicts = vec![MetricVerdict {
        metric: "accuracy".to_string(),
        regression: RegressionResult::Regression {
            delta: 0.10,
            threshold: 0.02,
        },
        significant: true,
        blocks: true,
    }];
    let report = GateReport::new("mmlu", GateDecision::Fail, verdicts);
    let md = report.to_markdown();
    assert!(md.contains("FAIL"));
    assert!(md.contains("regressed"));
}
