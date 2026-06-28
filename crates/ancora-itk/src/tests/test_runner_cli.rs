use crate::runner::Runner;

#[test]
fn runner_accumulates_results() {
    let mut runner = Runner::new();
    runner.record_kit(
        "provider_kit",
        vec![
            ("name_check".into(), true, "ok".into()),
            ("models_check".into(), true, "ok".into()),
        ],
    );
    runner.record_kit(
        "tool_kit",
        vec![
            ("name_check".into(), true, "ok".into()),
            ("call_check".into(), false, "failed".into()),
        ],
    );

    assert_eq!(runner.total(), 4);
    assert_eq!(runner.passed(), 3);
    assert_eq!(runner.failed(), 1);
}

#[test]
fn runner_builds_report_correctly() {
    let mut runner = Runner::new();
    runner.record_kit(
        "grader_kit",
        vec![
            ("name_check".into(), true, "ok".into()),
            ("score_bounds".into(), true, "ok".into()),
        ],
    );
    let report = runner.into_report("Grader Conformance Run");
    assert!(report.is_pass());
    assert_eq!(report.title, "Grader Conformance Run");
    assert_eq!(report.total, 2);
    assert_eq!(report.passed, 2);
    assert!(report.failing_lines().is_empty());
}

#[test]
fn runner_empty_produces_empty_report() {
    let runner = Runner::new();
    let report = runner.into_report("Empty Run");
    assert_eq!(report.total, 0);
    assert_eq!(report.passed, 0);
}
