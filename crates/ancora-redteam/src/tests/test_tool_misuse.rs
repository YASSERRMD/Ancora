use crate::{tool_misuse_scenarios, GuardrailScorer};
use ancora_guard::{AllowDenyGuardrail, GuardrailJournal, GuardrailOutcome, GuardrailPolicy};

fn check_tool(tool: &str) -> bool {
    let mut p = GuardrailPolicy::new();
    p.add_action(AllowDenyGuardrail::allow_only(vec![
        "search_web",
        "summarize",
    ]));
    let mut j = GuardrailJournal::default();
    !matches!(p.check_action(tool, "", &mut j, 1), GuardrailOutcome::Pass)
}

#[test]
fn tool_misuse_run_shell_blocked() {
    assert!(check_tool("run_shell"));
}

#[test]
fn tool_misuse_delete_all_files_blocked() {
    assert!(check_tool("delete_all_files"));
}

#[test]
fn tool_misuse_allowed_tool_passes() {
    assert!(!check_tool("search_web"));
    assert!(!check_tool("summarize"));
}

#[test]
fn tool_misuse_dataset_has_scenarios() {
    let scenarios = tool_misuse_scenarios();
    assert!(!scenarios.is_empty());
}

#[test]
fn tool_misuse_scorer_blocked_count() {
    let denied_tools = ["run_shell", "delete_all_files"];
    let scenarios: Vec<_> = denied_tools
        .iter()
        .map(|t| crate::AdversarialScenario::new("t", crate::AttackCategory::ToolMisuse, t, true))
        .collect();
    let report = GuardrailScorer::score(&scenarios, check_tool);
    assert_eq!(report.false_negatives(), 0);
    assert!((report.effectiveness() - 1.0).abs() < f64::EPSILON);
}
