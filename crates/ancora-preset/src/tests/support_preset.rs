use crate::{assemble, customer_support, Capability};

#[test]
fn support_preset_assembles() {
    let preset = customer_support();
    assert_eq!(preset.name, "customer-support");
    let spec = assemble(&preset).expect("should assemble");
    assert!(spec.tools.contains(&"routing".to_string()));
    assert!(spec.tools.contains(&"guardrails".to_string()));
    assert!(spec.tools.contains(&"memory".to_string()));
    assert!(spec.tools.contains(&"coordination".to_string()));
}

#[test]
fn support_preset_has_cost_control() {
    let preset = customer_support();
    assert!(preset.capabilities.contains(&Capability::CostControl));
}

#[test]
fn support_preset_system_prompt_contains_name() {
    let preset = customer_support();
    let spec = assemble(&preset).expect("should assemble");
    assert!(spec.system_prompt.contains("customer-support"));
}
