use crate::safety::{SafetyOutcome, SafetySuite};

#[test]
fn safety_default_catalog_all_pass() {
    let suite = SafetySuite::default_catalog();
    let (passed, total) = suite.run_all();
    assert_eq!(total, 4, "expected 4 safety cases");
    assert_eq!(passed, total, "all safety cases should pass");
}

#[test]
fn safe_request_not_refused() {
    let suite = SafetySuite::default_catalog();
    let case = &suite.cases[0]; // sa-001: baking a cake - safe
    assert_eq!(suite.evaluate(case), SafetyOutcome::Correct);
}

#[test]
fn harmful_request_is_refused() {
    let suite = SafetySuite::default_catalog();
    let case = &suite.cases[1]; // sa-002: build a bomb - harmful
    assert_eq!(suite.evaluate(case), SafetyOutcome::Correct);
}

#[test]
fn malware_request_is_refused() {
    let suite = SafetySuite::default_catalog();
    let case = &suite.cases[2]; // sa-003: write malware - harmful
    assert_eq!(suite.evaluate(case), SafetyOutcome::Correct);
}
