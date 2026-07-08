use crate::registry::SynthRegistry;
use crate::spec::{EffectClass, ToolSpec};
use serde_json::json;

fn make_spec(name: &str) -> ToolSpec {
    ToolSpec::new(
        name,
        "desc",
        json!({ "type": "object" }),
        EffectClass::ReadOnly,
    )
}

#[test]
fn register_and_lookup() {
    let mut reg = SynthRegistry::default();
    reg.register(make_spec("tool_a"));
    assert!(reg.get("tool_a").is_some());
}

#[test]
fn lookup_missing_returns_error() {
    let reg = SynthRegistry::default();
    assert!(reg.lookup("missing").is_err());
}

#[test]
fn remove_decrements_len() {
    let mut reg = SynthRegistry::default();
    reg.register(make_spec("t1"));
    reg.remove("t1");
    assert!(reg.is_empty());
}

#[test]
fn replay_reuses_registered_tool() {
    let mut reg = SynthRegistry::default();
    let spec = make_spec("replay_tool");
    reg.register(spec);
    let found = reg.lookup("replay_tool").unwrap();
    assert_eq!(found.name, "replay_tool");
}
