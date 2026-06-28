use crate::{assemble, coding_agent, government_compliant, research_assistant};

#[test]
fn system_prompt_contains_preset_name() {
    let spec = assemble(&research_assistant()).expect("assemble");
    assert!(spec.system_prompt.contains("preset:research-assistant"));
}

#[test]
fn system_prompt_contains_description() {
    let spec = assemble(&coding_agent()).expect("assemble");
    assert!(spec.system_prompt.contains("description:"));
}

#[test]
fn system_prompt_contains_all_capabilities() {
    let preset = research_assistant();
    let spec = assemble(&preset).expect("assemble");
    for tool in &spec.tools {
        assert!(
            spec.system_prompt.contains(&format!("capability:{tool}")),
            "system_prompt missing capability:{tool}"
        );
    }
}

#[test]
fn government_system_prompt_residency() {
    let spec = assemble(&government_compliant("us-gov-east-1")).expect("assemble");
    assert!(spec.system_prompt.contains("residency_zone:us-gov-east-1"));
}

#[test]
fn override_appears_in_system_prompt() {
    use crate::apply_overrides;
    let preset = research_assistant();
    let modified = apply_overrides(preset, vec![("depth".to_string(), "5".to_string())]);
    let spec = assemble(&modified).expect("assemble");
    assert!(spec.system_prompt.contains("override:depth=5"));
}
