use crate::sandbox::SandboxRunner;
use crate::spec::{EffectClass, ToolSpec};
use serde_json::json;

fn spec(name: &str, effect: EffectClass) -> ToolSpec {
    ToolSpec::new(name, "desc", json!({ "type": "object" }), effect)
}

#[test]
fn read_only_runs_in_sandbox() {
    let s = spec("list_files", EffectClass::ReadOnly);
    assert!(SandboxRunner::execute(&s, &json!({})).is_ok());
}

#[test]
fn write_local_runs_in_sandbox() {
    let s = spec("write_note", EffectClass::WriteLocal);
    assert!(SandboxRunner::execute(&s, &json!({})).is_ok());
}

#[test]
fn write_external_blocked_in_sandbox() {
    let s = spec("send_http", EffectClass::WriteExternal);
    assert!(SandboxRunner::execute(&s, &json!({})).is_err());
}

#[test]
fn destructive_blocked_in_sandbox() {
    let s = spec("drop_table", EffectClass::Destructive);
    assert!(SandboxRunner::execute(&s, &json!({})).is_err());
}
