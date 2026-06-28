use crate::{assemble, validate, AirGapPolicy, Capability, PresetDescriptor, ValidationError};

#[test]
fn airgap_with_routing_fails_validation() {
    let preset = PresetDescriptor::new("conflict", "bad combo")
        .with_capability(Capability::Planning)
        .with_capability(Capability::Routing)
        .with_air_gap(AirGapPolicy::Required);
    let errs = validate(&preset).unwrap_err();
    assert!(errs.contains(&ValidationError::AirGapConflictsWithRouting));
}

#[test]
fn airgap_with_routing_fails_assembly() {
    let preset = PresetDescriptor::new("conflict", "bad combo")
        .with_capability(Capability::Planning)
        .with_capability(Capability::Routing)
        .with_air_gap(AirGapPolicy::Required);
    assert!(assemble(&preset).is_err());
}

#[test]
fn airgap_without_routing_passes() {
    let preset = PresetDescriptor::new("ok", "no routing")
        .with_capability(Capability::Planning)
        .with_capability(Capability::Memory)
        .with_air_gap(AirGapPolicy::Required);
    assert!(validate(&preset).is_ok());
}
