use crate::PresetDescriptor;
use crate::Capability;

#[test]
fn descriptor_name_round_trips() {
    let p = PresetDescriptor::new("my-preset", "desc")
        .with_capability(Capability::Planning);
    assert_eq!(p.name, "my-preset");
}

#[test]
fn descriptor_description_round_trips() {
    let p = PresetDescriptor::new("n", "my description")
        .with_capability(Capability::Planning);
    assert_eq!(p.description, "my description");
}

#[test]
fn descriptor_builder_chain_works() {
    use crate::{AirGapPolicy, ResidencyConstraint};
    let p = PresetDescriptor::new("chain", "desc")
        .with_capability(Capability::Memory)
        .with_capability(Capability::Guardrails)
        .with_air_gap(AirGapPolicy::Required)
        .with_residency(ResidencyConstraint::Zone("eu-west-1".to_string()))
        .with_locked(true)
        .with_override("timeout", "30s");
    assert_eq!(p.capabilities.len(), 2);
    assert_eq!(p.air_gap, AirGapPolicy::Required);
    assert!(p.locked);
    assert_eq!(p.overrides.len(), 1);
}
