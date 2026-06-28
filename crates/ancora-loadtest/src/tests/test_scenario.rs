use crate::scenario::Scenario;
use crate::workload::WorkloadSpec;

#[test]
fn passing_scenario_all_fast() {
    let spec = WorkloadSpec::new("fast", 100.0, 1, 10);
    let scenario = Scenario::new(spec, 0.01, 500);
    let result = scenario.run(|_| (10, false));
    assert!(result.passed);
    assert_eq!(result.summary.total_errors, 0);
}

#[test]
fn failing_scenario_high_error_rate() {
    let spec = WorkloadSpec::new("errors", 10.0, 1, 1);
    let scenario = Scenario::new(spec, 0.05, 500);
    let result = scenario.run(|i| if i % 2 == 0 { (10, true) } else { (10, false) });
    assert!(!result.passed);
}

#[test]
fn total_expected_requests_correct() {
    let spec = WorkloadSpec::new("x", 50.0, 2, 1);
    assert_eq!(spec.total_expected_requests(), 100);
}
