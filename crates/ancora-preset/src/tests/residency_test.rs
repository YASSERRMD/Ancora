use crate::{government_compliant, research_assistant, ResidencyConstraint};

#[test]
fn research_preset_no_residency() {
    let preset = research_assistant();
    assert_eq!(preset.residency, ResidencyConstraint::None);
}

#[test]
fn government_preset_zone_set() {
    let preset = government_compliant("ap-gov-south-1");
    assert_eq!(
        preset.residency,
        ResidencyConstraint::Zone("ap-gov-south-1".to_string())
    );
}

#[test]
fn zone_round_trips_through_assembly() {
    use crate::assemble;
    let preset = government_compliant("eu-gov-north-1");
    let spec = assemble(&preset).expect("assemble");
    assert!(spec.system_prompt.contains("residency_zone:eu-gov-north-1"));
}
