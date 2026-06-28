use std::sync::Arc;
use crate::conformance::{Check, CheckResult, ConformanceSuite, grader_suite, provider_suite};
use crate::grader_template::MyGrader;
use crate::provider_template::MyProvider;

#[test]
fn check_result_is_pass_predicate() {
    assert!(CheckResult::Pass.is_pass());
    assert!(!CheckResult::Fail("x".into()).is_pass());
    assert!(!CheckResult::Skip("x".into()).is_pass());
}

#[test]
fn check_result_is_fail_predicate() {
    assert!(CheckResult::Fail("x".into()).is_fail());
    assert!(!CheckResult::Pass.is_fail());
}

#[test]
fn check_result_display() {
    assert_eq!(CheckResult::Pass.to_string(), "PASS");
    assert!(CheckResult::Fail("bad".into()).to_string().contains("bad"));
    assert!(CheckResult::Skip("reason".into()).to_string().contains("reason"));
}

#[test]
fn suite_with_all_passing_checks() {
    let mut suite = ConformanceSuite::new("all-pass");
    suite.add(Check::new("a", "always passes", || CheckResult::Pass));
    suite.add(Check::new("b", "also passes", || CheckResult::Pass));
    let report = suite.run();
    assert_eq!(report.passed(), 2);
    assert_eq!(report.failed(), 0);
    assert!(report.all_passed());
}

#[test]
fn suite_with_failing_check() {
    let mut suite = ConformanceSuite::new("has-failure");
    suite.add(Check::new("a", "passes", || CheckResult::Pass));
    suite.add(Check::new("b", "fails", || CheckResult::Fail("intentional".into())));
    let report = suite.run();
    assert_eq!(report.passed(), 1);
    assert_eq!(report.failed(), 1);
    assert!(!report.all_passed());
}

#[test]
fn suite_with_skipped_check() {
    let mut suite = ConformanceSuite::new("has-skip");
    suite.add(Check::new("a", "skips", || CheckResult::Skip("not applicable".into())));
    let report = suite.run();
    assert_eq!(report.skipped(), 1);
    assert_eq!(report.total(), 1);
}

#[test]
fn provider_conformance_suite_passes_for_my_provider() {
    let provider = Arc::new(MyProvider::new("key"));
    let suite = provider_suite(provider);
    let report = suite.run();
    assert!(
        report.all_passed(),
        "provider conformance failed:\n{report}"
    );
}

#[test]
fn grader_conformance_suite_passes_for_my_grader() {
    let grader = Arc::new(MyGrader);
    let suite = grader_suite(grader);
    let report = suite.run();
    assert!(
        report.all_passed(),
        "grader conformance failed:\n{report}"
    );
}

#[test]
fn suite_report_display_is_nonempty() {
    let mut suite = ConformanceSuite::new("display-test");
    suite.add(Check::new("c1", "passes", || CheckResult::Pass));
    let report = suite.run();
    let s = report.to_string();
    assert!(s.contains("display-test"));
    assert!(s.contains("PASS"));
}

#[test]
fn empty_suite_has_no_failures() {
    let suite = ConformanceSuite::new("empty");
    let report = suite.run();
    assert_eq!(report.total(), 0);
    assert!(report.all_passed());
}
