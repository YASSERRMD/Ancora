use crate::{assemble, research_assistant};

#[test]
fn assembled_spec_tools_match_capabilities() {
    let preset = research_assistant();
    let cap_count = preset.capabilities.len();
    let spec = assemble(&preset).expect("assemble");
    assert_eq!(
        spec.tools.len(),
        cap_count,
        "tools count must equal capabilities count"
    );
}

#[test]
fn assembled_spec_has_correct_role() {
    use ancora_orchestrate::AgentRole;
    let preset = research_assistant();
    let spec = assemble(&preset).expect("assemble");
    assert_eq!(spec.role, AgentRole::Orchestrator);
}

#[test]
fn assembled_spec_model_set() {
    let preset = research_assistant();
    let spec = assemble(&preset).expect("assemble");
    assert!(!spec.model.is_empty());
}
