use crate::long_context::{LongContextOutcome, LongContextSuite};

#[test]
fn long_context_default_catalog_all_pass() {
    let suite = LongContextSuite::default_catalog();
    let (passed, total) = suite.run_all();
    assert_eq!(total, 2, "expected 2 long-context cases");
    assert_eq!(passed, total, "all long-context cases should pass");
}

#[test]
fn founder_fact_retrieved_from_long_document() {
    let suite = LongContextSuite::default_catalog();
    let case = &suite.cases[0]; // lc-001: founder
    assert_eq!(suite.evaluate(case), LongContextOutcome::Correct);
}

#[test]
fn license_fact_retrieved_from_long_document() {
    let suite = LongContextSuite::default_catalog();
    let case = &suite.cases[1]; // lc-002: license
    assert_eq!(suite.evaluate(case), LongContextOutcome::Correct);
}
