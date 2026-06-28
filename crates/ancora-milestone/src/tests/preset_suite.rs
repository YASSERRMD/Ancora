use ancora_preset::{assemble, government_compliant, research_assistant, validate};

#[test]
fn preset_suite_research_valid() {
    assert!(validate(&research_assistant()).is_ok());
}

#[test]
fn preset_suite_government_valid() {
    assert!(validate(&government_compliant("test-zone")).is_ok());
}

#[test]
fn preset_suite_all_assemble() {
    use ancora_preset::{coding_agent, customer_support, data_analysis};
    for preset in [
        research_assistant(),
        coding_agent(),
        customer_support(),
        data_analysis(),
        government_compliant("zone"),
    ] {
        assert!(assemble(&preset).is_ok(), "preset '{}' failed", preset.name);
    }
}
