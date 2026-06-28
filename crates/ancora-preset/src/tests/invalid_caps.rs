use crate::{assemble, validate, PresetDescriptor, ValidationError};

#[test]
fn preset_with_no_capabilities_fails_validation() {
    let preset = PresetDescriptor::new("empty", "no caps");
    let errs = validate(&preset).unwrap_err();
    assert!(errs.contains(&ValidationError::NoCapabilities));
}

#[test]
fn preset_with_no_capabilities_fails_assembly() {
    let preset = PresetDescriptor::new("empty", "no caps");
    assert!(assemble(&preset).is_err());
}
