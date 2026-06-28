use crate::{assemble, research_assistant, Capability};

#[test]
fn research_preset_assembles() {
    let preset = research_assistant();
    assert_eq!(preset.name, "research-assistant");
    let spec = assemble(&preset).expect("should assemble");
    assert!(spec.tools.contains(&"memory".to_string()));
    assert!(spec.tools.contains(&"reasoning".to_string()));
    assert!(spec.tools.contains(&"long_horizon".to_string()));
    assert!(spec.tools.contains(&"planning".to_string()));
    assert!(spec.tools.contains(&"reflection".to_string()));
}

#[test]
fn research_preset_has_behavior_eval() {
    let preset = research_assistant();
    assert!(preset.capabilities.contains(&Capability::BehaviorEval));
}

#[test]
fn research_preset_no_air_gap() {
    use crate::AirGapPolicy;
    let preset = research_assistant();
    assert_eq!(preset.air_gap, AirGapPolicy::None);
}
