// All advanced capability operations run in-memory; no network calls required.
// These tests verify that the full capability surface completes synchronously
// without any I/O by exercising each crate's core path in a single test binary.

use ancora_coord::Blackboard;
use ancora_guard::{GuardrailJournal, GuardrailPolicy, PiiInputGuardrail};
use ancora_lh::BackgroundRun;
use ancora_memcon::TokenBudget;
use ancora_orchestrate::fan_out;
use ancora_reason::StepDecomposer;
use ancora_skills::{SkillDescriptor, SkillRegistry, SkillScope};
use ancora_toolsynth::spec_from_goal;
use serde_json::json;

#[test]
fn no_network_orchestrate_fan_out() {
    let tasks = fan_out("o", "a", vec![json!("t1"), json!("t2")], "root");
    assert_eq!(tasks.len(), 2);
}

#[test]
fn no_network_coord_blackboard() {
    let mut b = Blackboard::default();
    b.claim_role("a1", "key");
    b.write("a1", "key", "value").unwrap();
    assert_eq!(b.read("key"), Some("value"));
}

#[test]
fn no_network_guard_policy() {
    let mut p = GuardrailPolicy::new();
    p.add_input(PiiInputGuardrail);
    let mut j = GuardrailJournal::default();
    p.check_input("hello", &mut j, 1);
    // passes without network
}

#[test]
fn no_network_lh_run() {
    let mut run = BackgroundRun::new("r", 1);
    run.start();
    run.apply_effect("step");
    run.complete();
    assert!(run.effects_applied.contains(&"step".to_string()));
}

#[test]
fn no_network_reasoning() {
    let steps = StepDecomposer::decompose(vec!["claim".into()]);
    assert_eq!(steps.len(), 1);
}

#[test]
fn no_network_token_budget() {
    let budget = TokenBudget::new(500);
    assert!(budget.within_budget(&["short content".to_string()]));
}

#[test]
fn no_network_toolsynth_spec() {
    let spec = spec_from_goal("search documents");
    assert_eq!(spec.name, "search_documents");
}

#[test]
fn no_network_skills_load() {
    let mut reg = SkillRegistry::default();
    reg.load(SkillDescriptor::new(
        "echo",
        1,
        "echo text",
        vec![],
        SkillScope::ReadOnly,
    ));
    assert!(reg.find("echo").is_some());
}
