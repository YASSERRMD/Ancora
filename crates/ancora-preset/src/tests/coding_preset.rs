use crate::{assemble, coding_agent, Capability};

#[test]
fn coding_preset_assembles() {
    let preset = coding_agent();
    assert_eq!(preset.name, "coding-agent");
    let spec = assemble(&preset).expect("should assemble");
    assert!(spec.tools.contains(&"planning".to_string()));
    assert!(spec.tools.contains(&"tool_synthesis".to_string()));
    assert!(spec.tools.contains(&"skills".to_string()));
    assert!(spec.tools.contains(&"guardrails".to_string()));
}

#[test]
fn coding_preset_includes_cost_control() {
    let preset = coding_agent();
    assert!(preset.capabilities.contains(&Capability::CostControl));
}

#[test]
fn coding_preset_not_locked() {
    let preset = coding_agent();
    assert!(!preset.locked);
}
