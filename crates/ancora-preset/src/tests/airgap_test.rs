use crate::{assemble, coding_agent, government_compliant, AirGapPolicy};

#[test]
fn coding_preset_allows_network() {
    let preset = coding_agent();
    assert_eq!(preset.air_gap, AirGapPolicy::None);
}

#[test]
fn government_preset_requires_airgap() {
    let preset = government_compliant("us-gov-east-1");
    assert_eq!(preset.air_gap, AirGapPolicy::Required);
}

#[test]
fn government_spec_prompt_has_airgap_flag() {
    let preset = government_compliant("us-gov-east-1");
    let spec = assemble(&preset).expect("assemble");
    assert!(spec.system_prompt.contains("air_gap:required"));
}

#[test]
fn all_presets_run_in_process() {
    use crate::{customer_support, data_analysis, research_assistant};
    // Assemble all 5 presets -- each must succeed with no network call
    let presets = vec![
        research_assistant(),
        coding_agent(),
        customer_support(),
        data_analysis(),
        government_compliant("local"),
    ];
    for p in &presets {
        let spec = assemble(p).expect("all presets should assemble in-process");
        assert!(!spec.agent_id.is_empty());
    }
}
