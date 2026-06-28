use crate::report::{KitReport, KitStatus};

fn make_all_pass_report() -> KitReport {
    KitReport {
        title: "Test Suite".into(),
        status: KitStatus::AllPassed,
        lines: vec![
            "[PASS] kit::check_name - ok".into(),
            "[PASS] kit::check_call - ok".into(),
        ],
        total: 2,
        passed: 2,
    }
}

fn make_partial_report() -> KitReport {
    KitReport {
        title: "Partial Suite".into(),
        status: KitStatus::SomeFailed { failed: 1, total: 3 },
        lines: vec![
            "[PASS] kit::check_name - ok".into(),
            "[PASS] kit::check_call - ok".into(),
            "[FAIL] kit::check_schema - missing field".into(),
        ],
        total: 3,
        passed: 2,
    }
}

#[test]
fn report_is_pass_when_all_checks_pass() {
    let report = make_all_pass_report();
    assert!(report.is_pass());
}

#[test]
fn report_is_not_pass_when_some_fail() {
    let report = make_partial_report();
    assert!(!report.is_pass());
}

#[test]
fn report_failing_lines_returns_only_failures() {
    let report = make_partial_report();
    let fails = report.failing_lines();
    assert_eq!(fails.len(), 1);
    assert!(fails[0].contains("check_schema"));
}

#[test]
fn report_render_contains_title_and_status() {
    let report = make_all_pass_report();
    let rendered = report.render();
    assert!(rendered.contains("Test Suite"));
    assert!(rendered.contains("ALL PASSED"));
    assert!(rendered.contains("2/2"));
}

#[test]
fn report_status_display() {
    let all_pass = KitStatus::AllPassed;
    assert_eq!(format!("{all_pass}"), "ALL PASSED");

    let some_fail = KitStatus::SomeFailed { failed: 2, total: 5 };
    assert_eq!(format!("{some_fail}"), "2/5 FAILED");
}
