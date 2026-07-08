use crate::report::{LoadTestReport, ScenarioReport};
use crate::soak::SoakSummary;

fn summary(errors: u64, p99: u64) -> SoakSummary {
    SoakSummary {
        label: "s".into(),
        total_requests: 100,
        total_errors: errors,
        error_rate: errors as f64 / 100.0,
        peak_rps: 10.0,
        p50_ms: 10,
        p95_ms: 20,
        p99_ms: p99,
    }
}

#[test]
fn all_passed_when_all_pass() {
    let reports = vec![
        ScenarioReport::from_summary("a", true, &summary(0, 50)),
        ScenarioReport::from_summary("b", true, &summary(0, 50)),
    ];
    let report = LoadTestReport::new(reports);
    assert!(report.all_passed);
    assert_eq!(report.failed_count(), 0);
}

#[test]
fn failed_count_correct() {
    let reports = vec![
        ScenarioReport::from_summary("a", true, &summary(0, 50)),
        ScenarioReport::from_summary("b", false, &summary(10, 800)),
    ];
    let report = LoadTestReport::new(reports);
    assert!(!report.all_passed);
    assert_eq!(report.failed_count(), 1);
}

#[test]
fn to_json_contains_scenario_name() {
    let reports = vec![ScenarioReport::from_summary(
        "my-scenario",
        true,
        &summary(0, 50),
    )];
    let report = LoadTestReport::new(reports);
    let json = report.to_json();
    assert!(json.contains("my-scenario"));
}
