use crate::{validate, Capability, PresetDescriptor, ValidationError};

#[test]
fn valid_preset_passes() {
    let preset = PresetDescriptor::new("test", "a valid preset")
        .with_capability(Capability::Planning);
    assert!(validate(&preset).is_ok());
}

#[test]
fn empty_name_rejected() {
    let preset = PresetDescriptor::new("", "desc")
        .with_capability(Capability::Planning);
    let errs = validate(&preset).unwrap_err();
    assert!(errs.contains(&ValidationError::EmptyName));
}

#[test]
fn empty_description_rejected() {
    let preset = PresetDescriptor::new("name", "")
        .with_capability(Capability::Planning);
    let errs = validate(&preset).unwrap_err();
    assert!(errs.contains(&ValidationError::EmptyDescription));
}

#[test]
fn no_capabilities_rejected() {
    let preset = PresetDescriptor::new("name", "desc");
    let errs = validate(&preset).unwrap_err();
    assert!(errs.contains(&ValidationError::NoCapabilities));
}

#[test]
fn multiple_errors_accumulated() {
    let preset = PresetDescriptor::new("", "");
    let errs = validate(&preset).unwrap_err();
    assert!(errs.len() >= 2);
}
