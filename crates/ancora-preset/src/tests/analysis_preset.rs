use crate::{assemble, data_analysis, Capability};

#[test]
fn analysis_preset_assembles() {
    let preset = data_analysis();
    assert_eq!(preset.name, "data-analysis");
    let spec = assemble(&preset).expect("should assemble");
    assert!(spec.tools.contains(&"planning".to_string()));
    assert!(spec.tools.contains(&"reasoning".to_string()));
    assert!(spec.tools.contains(&"tool_synthesis".to_string()));
    assert!(spec.tools.contains(&"behavior_eval".to_string()));
}

#[test]
fn analysis_preset_has_memory() {
    let preset = data_analysis();
    assert!(preset.capabilities.contains(&Capability::Memory));
}

#[test]
fn analysis_preset_spec_agent_id() {
    let preset = data_analysis();
    let spec = assemble(&preset).expect("should assemble");
    assert_eq!(spec.agent_id, "data-analysis");
}
