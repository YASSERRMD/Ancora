use ancora_toolsynth::{spec_from_goal, ApprovalGate, EffectClass, SandboxRunner, SynthCache, ToolSpec};
use serde_json::json;

#[test]
fn toolsynth_parity_spec_from_goal() {
    let spec = spec_from_goal("search the web");
    assert_eq!(spec.name, "search_the_web");
    assert_eq!(spec.description, "search the web");
    assert_eq!(spec.effect_class, EffectClass::ReadOnly);
}

#[test]
fn toolsynth_parity_cache_hit() {
    let mut cache = SynthCache::default();
    let spec = spec_from_goal("summarize text");
    cache.insert("summarize text", spec);
    assert!(cache.get("summarize text").is_some());
    assert!(cache.get("other").is_none());
}

#[test]
fn toolsynth_parity_approval_gate() {
    let mut gate = ApprovalGate::default();
    assert!(!gate.is_approved("search_web"));
    gate.approve("search_web");
    assert!(gate.is_approved("search_web"));
}

#[test]
fn toolsynth_parity_sandbox_execute_readonly() {
    let spec = ToolSpec::new(
        "echo",
        "echo input",
        json!({"type": "object"}),
        EffectClass::ReadOnly,
    );
    let result = SandboxRunner::execute(&spec, &json!({"text": "hello"}));
    assert!(result.is_ok());
}

#[test]
fn toolsynth_parity_sandbox_destructive_blocked() {
    let spec = ToolSpec::new(
        "nuke",
        "delete everything",
        json!({}),
        EffectClass::Destructive,
    );
    assert!(SandboxRunner::execute(&spec, &json!({})).is_err());
}
