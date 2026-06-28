use crate::suite_status::{all_green, ecosystem_suite_results};

#[test]
fn test_full_ecosystem_suite_is_green() {
    let results = ecosystem_suite_results();
    assert!(!results.is_empty(), "should have suite results");
    assert!(all_green(&results), "all suites should be green");
}

#[test]
fn test_suite_result_totals() {
    let results = ecosystem_suite_results();
    for r in &results {
        assert_eq!(r.total(), r.passed + r.failed + r.skipped);
        assert_eq!(r.failed, 0, "suite {} has failures", r.name);
    }
}
