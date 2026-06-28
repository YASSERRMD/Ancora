use crate::reasoning::{ReasoningSuite, ReasoningOutcome};

#[test]
fn reasoning_default_catalog_all_pass() {
    let suite = ReasoningSuite::default_catalog();
    let (passed, total) = suite.run_all();
    assert_eq!(total, 4, "expected 4 reasoning cases");
    assert_eq!(passed, total, "all reasoning cases should pass");
}

#[test]
fn arithmetic_addition_correct() {
    let suite = ReasoningSuite::default_catalog();
    let case = &suite.cases[0]; // re-001: 42 + 58 = 100
    assert_eq!(suite.evaluate(case), ReasoningOutcome::Correct);
}

#[test]
fn arithmetic_subtraction_correct() {
    let suite = ReasoningSuite::default_catalog();
    let case = &suite.cases[1]; // re-002: 100 - 37 = 63
    assert_eq!(suite.evaluate(case), ReasoningOutcome::Correct);
}

#[test]
fn logical_syllogism_correct() {
    let suite = ReasoningSuite::default_catalog();
    let case = &suite.cases[2]; // re-003
    assert_eq!(suite.evaluate(case), ReasoningOutcome::Correct);
}
