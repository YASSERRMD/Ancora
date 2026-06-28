use crate::suite_status::{SuiteResult, SuiteStatus, milestone_green};

#[test]
fn full_obs_and_eval_suite_green() {
    let suites = vec![
        SuiteStatus::new("ancora-observability", SuiteResult::Green),
        SuiteStatus::new("ancora-metrics", SuiteResult::Green),
        SuiteStatus::new("ancora-trace", SuiteResult::Green),
        SuiteStatus::new("ancora-evals", SuiteResult::Green),
        SuiteStatus::new("ancora-evalrun", SuiteResult::Green),
        SuiteStatus::new("ancora-evalgate", SuiteResult::Green),
        SuiteStatus::new("ancora-evallib", SuiteResult::Green),
        SuiteStatus::new("ancora-conteval", SuiteResult::Green),
        SuiteStatus::new("ancora-obssdk", SuiteResult::Green),
        SuiteStatus::new("ancora-obsint", SuiteResult::Green),
    ];
    assert!(
        milestone_green(&suites),
        "All obs and eval suites must be green at this milestone"
    );
}

#[test]
fn suite_status_summaries_are_correct() {
    let s = SuiteStatus::new("test-suite", SuiteResult::Green);
    assert!(s.summary().contains("GREEN"));
    assert!(s.is_green());
}
