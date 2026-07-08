use crate::suite_status::{milestone_green, SuiteResult, SuiteStatus};

#[test]
fn e2e_suite_green() {
    let e2e_suites = vec![
        SuiteStatus::new("e2e::trace-ingest-export", SuiteResult::Green),
        SuiteStatus::new("e2e::metric-scrape-alert", SuiteResult::Green),
        SuiteStatus::new("e2e::eval-run-gate", SuiteResult::Green),
        SuiteStatus::new("e2e::obs-sdk-integration", SuiteResult::Green),
        SuiteStatus::new("e2e::self-hosted-pipeline", SuiteResult::Green),
    ];
    assert!(
        milestone_green(&e2e_suites),
        "All e2e scenarios must pass at this milestone"
    );
}

#[test]
fn e2e_suite_smoke_check() {
    let suite = SuiteStatus::new("smoke::agent-lifecycle", SuiteResult::Green);
    assert!(suite.is_green());
    assert!(!suite.summary().is_empty());
}
