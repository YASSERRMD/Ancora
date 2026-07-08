use crate::spec::{spec_from_goal, EffectClass};

#[test]
fn spec_from_goal_creates_tool_spec() {
    let spec = spec_from_goal("List files in directory");
    assert!(spec.name.contains("list"));
    assert_eq!(spec.effect_class, EffectClass::ReadOnly);
}

#[test]
fn spec_has_object_schema() {
    let spec = spec_from_goal("Count tokens");
    assert_eq!(spec.input_schema["type"], "object");
}
