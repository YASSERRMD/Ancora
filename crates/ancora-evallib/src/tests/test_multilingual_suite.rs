use crate::multilingual::{MultilingualSuite, MultilingualOutcome};

#[test]
fn multilingual_default_catalog_all_pass() {
    let suite = MultilingualSuite::default_catalog();
    let (passed, total) = suite.run_all();
    assert_eq!(total, 4, "expected 4 multilingual cases");
    assert_eq!(passed, total, "all multilingual cases should pass");
}

#[test]
fn english_hello_correct() {
    let suite = MultilingualSuite::default_catalog();
    let case = &suite.cases[0]; // ml-001: English
    assert_eq!(suite.evaluate(case), MultilingualOutcome::Correct);
}

#[test]
fn spanish_hola_correct() {
    let suite = MultilingualSuite::default_catalog();
    let case = &suite.cases[1]; // ml-002: Spanish
    assert_eq!(suite.evaluate(case), MultilingualOutcome::Correct);
}

#[test]
fn french_bonjour_correct() {
    let suite = MultilingualSuite::default_catalog();
    let case = &suite.cases[2]; // ml-003: French
    assert_eq!(suite.evaluate(case), MultilingualOutcome::Correct);
}
