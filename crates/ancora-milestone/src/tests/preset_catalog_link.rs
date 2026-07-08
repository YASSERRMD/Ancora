use ancora_preset::{
    assemble, coding_agent, customer_support, data_analysis, government_compliant,
    research_assistant,
};

#[test]
fn all_five_presets_listed_in_catalog() {
    let presets = [
        research_assistant(),
        coding_agent(),
        customer_support(),
        data_analysis(),
        government_compliant("us-gov-east-1"),
    ];
    let names: Vec<&str> = presets.iter().map(|p| p.name.as_str()).collect();
    assert!(names.contains(&"research-assistant"));
    assert!(names.contains(&"coding-agent"));
    assert!(names.contains(&"customer-support"));
    assert!(names.contains(&"data-analysis"));
    assert!(names.contains(&"government-compliant"));
}

#[test]
fn all_five_presets_assemble() {
    let presets = [
        research_assistant(),
        coding_agent(),
        customer_support(),
        data_analysis(),
        government_compliant("us-gov-east-1"),
    ];
    for p in &presets {
        assert!(
            assemble(p).is_ok(),
            "preset '{}' failed to assemble",
            p.name
        );
    }
}
