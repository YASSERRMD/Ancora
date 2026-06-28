use crate::{assemble, government_compliant, AirGapPolicy, Capability, ResidencyConstraint};

#[test]
fn government_preset_air_gap_required() {
    let preset = government_compliant("us-gov-east-1");
    assert_eq!(preset.air_gap, AirGapPolicy::Required);
}

#[test]
fn government_preset_residency_zone() {
    let preset = government_compliant("us-gov-east-1");
    assert_eq!(
        preset.residency,
        ResidencyConstraint::Zone("us-gov-east-1".to_string())
    );
}

#[test]
fn government_preset_is_locked() {
    let preset = government_compliant("us-gov-east-1");
    assert!(preset.locked);
}

#[test]
fn government_preset_no_routing() {
    let preset = government_compliant("us-gov-east-1");
    assert!(
        !preset.capabilities.contains(&Capability::Routing),
        "air-gapped preset must not include Routing"
    );
}

#[test]
fn government_preset_assembles() {
    let preset = government_compliant("eu-gov-west-1");
    let spec = assemble(&preset).expect("should assemble");
    assert!(spec.system_prompt.contains("air_gap:required"));
    assert!(spec.system_prompt.contains("residency_zone:eu-gov-west-1"));
    assert!(spec.system_prompt.contains("locked:true"));
}

#[test]
fn government_preset_has_guardrails() {
    let preset = government_compliant("us-gov-east-1");
    assert!(preset.capabilities.contains(&Capability::Guardrails));
}
