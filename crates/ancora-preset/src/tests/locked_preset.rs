use crate::{government_compliant, research_assistant};

#[test]
fn government_preset_is_locked() {
    let preset = government_compliant("us-gov-east-1");
    assert!(preset.locked, "government preset must be locked");
}

#[test]
fn research_preset_not_locked_by_default() {
    let preset = research_assistant();
    assert!(
        !preset.locked,
        "research preset should not be locked by default"
    );
}

#[test]
fn locked_flag_encoded_in_system_prompt() {
    use crate::assemble;
    let preset = government_compliant("us-gov-east-1");
    let spec = assemble(&preset).expect("assemble");
    assert!(spec.system_prompt.contains("locked:true"));
}
