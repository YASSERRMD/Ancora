use crate::rag_faithfulness::{RagFaithfulnessSuite, RagOutcome};

#[test]
fn rag_default_catalog_all_pass() {
    let suite = RagFaithfulnessSuite::default_catalog();
    let (passed, total) = suite.run_all();
    assert_eq!(total, 3, "expected 3 RAG faithfulness cases");
    assert_eq!(passed, total, "all RAG cases should pass");
}

#[test]
fn rag_faithful_answer_returns_correct() {
    let suite = RagFaithfulnessSuite::default_catalog();
    // case rf-001 is faithful and expected_faithful = true
    let case = &suite.cases[0];
    assert_eq!(suite.evaluate(case), RagOutcome::Correct);
}

#[test]
fn rag_unfaithful_answer_returns_correct_outcome() {
    let suite = RagFaithfulnessSuite::default_catalog();
    // case rf-002 is unfaithful and expected_faithful = false
    let case = &suite.cases[1];
    assert_eq!(suite.evaluate(case), RagOutcome::Correct);
}
