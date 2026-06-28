use crate::{assemble, validate, Capability, PresetDescriptor, ValidationError};

#[test]
fn whitespace_only_name_rejected() {
    let preset = PresetDescriptor::new("   ", "desc")
        .with_capability(Capability::Planning);
    let errs = validate(&preset).unwrap_err();
    assert!(errs.contains(&ValidationError::EmptyName));
}

#[test]
fn assemble_rejects_invalid_preset() {
    let preset = PresetDescriptor::new("", "desc")
        .with_capability(Capability::Memory);
    let result = assemble(&preset);
    assert!(result.is_err(), "assembling invalid preset should fail");
}

#[test]
fn assemble_error_message_non_empty() {
    let preset = PresetDescriptor::new("", "");
    let err = assemble(&preset).unwrap_err();
    assert!(!err.0.is_empty());
}
