use crate::tool_use::{ToolUseSuite, ToolUseOutcome};

#[test]
fn tool_use_default_catalog_all_pass() {
    let suite = ToolUseSuite::default_catalog();
    let (passed, total) = suite.run_all();
    assert_eq!(total, 3, "expected 3 tool-use cases");
    assert_eq!(passed, total, "all tool-use cases should pass");
}

#[test]
fn tool_use_correct_outcome_for_first_case() {
    let suite = ToolUseSuite::default_catalog();
    let case = &suite.cases[0];
    let outcome = suite.evaluate(case);
    assert_eq!(outcome, ToolUseOutcome::Correct);
}

#[test]
fn tool_use_no_tool_selected_when_no_match() {
    use crate::tool_use::{Tool, ToolUseCase};
    let suite = ToolUseSuite { cases: vec![] };
    let case = ToolUseCase::new(
        "t-x",
        "Completely unrelated request with no keyword match.",
        vec![Tool::new("search", "search the web")],
        "search",
    );
    let outcome = suite.evaluate(&case);
    // The agent cannot find a match, so it returns NoToolSelected.
    assert_eq!(outcome, ToolUseOutcome::NoToolSelected);
}
