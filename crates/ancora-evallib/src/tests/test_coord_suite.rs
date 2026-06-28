use crate::coordination::{CoordinationSuite, CoordOutcome};

#[test]
fn coordination_default_catalog_all_pass() {
    let suite = CoordinationSuite::default_catalog();
    let (passed, total) = suite.run_all();
    assert_eq!(total, 1, "expected 1 coordination case");
    assert_eq!(passed, total, "all coordination cases should pass");
}

#[test]
fn coordination_assembled_answer_contains_required_keywords() {
    let suite = CoordinationSuite::default_catalog();
    let case = &suite.cases[0];
    let outcome = suite.evaluate(case);
    assert_eq!(outcome, CoordOutcome::Correct);
}
