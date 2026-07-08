use crate::allowdeny::AllowDenyGuardrail;
use crate::guardrail::{ActionGuardrail, GuardrailOutcome};

#[test]
fn denied_action_blocked() {
    let g = AllowDenyGuardrail::deny(vec!["delete_file", "drop_table"]);
    assert!(matches!(
        g.check_action("delete_file", "{}"),
        GuardrailOutcome::Block(_)
    ));
}

#[test]
fn allowed_action_passes_with_allowlist() {
    let g = AllowDenyGuardrail::allow_only(vec!["read_file", "list_files"]);
    assert_eq!(g.check_action("read_file", "{}"), GuardrailOutcome::Pass);
}

#[test]
fn non_allowlist_action_blocked() {
    let g = AllowDenyGuardrail::allow_only(vec!["read_file"]);
    assert!(matches!(
        g.check_action("write_file", "{}"),
        GuardrailOutcome::Block(_)
    ));
}

#[test]
fn not_on_denylist_passes() {
    let g = AllowDenyGuardrail::deny(vec!["bad_tool"]);
    assert_eq!(g.check_action("good_tool", "{}"), GuardrailOutcome::Pass);
}
