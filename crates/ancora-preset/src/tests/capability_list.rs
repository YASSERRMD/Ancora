use crate::{
    coding_agent, customer_support, data_analysis, government_compliant, research_assistant,
};

#[test]
fn research_preset_capability_count() {
    let preset = research_assistant();
    assert!(
        preset.capabilities.len() >= 5,
        "research preset should have at least 5 capabilities"
    );
}

#[test]
fn coding_preset_capability_count() {
    let preset = coding_agent();
    assert!(
        preset.capabilities.len() >= 5,
        "coding preset should have at least 5 capabilities"
    );
}

#[test]
fn support_preset_capability_count() {
    let preset = customer_support();
    assert!(
        preset.capabilities.len() >= 4,
        "support preset should have at least 4 capabilities"
    );
}

#[test]
fn analysis_preset_capability_count() {
    let preset = data_analysis();
    assert!(
        preset.capabilities.len() >= 5,
        "data-analysis preset should have at least 5 capabilities"
    );
}

#[test]
fn government_preset_capability_count() {
    let preset = government_compliant("us-gov-east-1");
    assert!(
        preset.capabilities.len() >= 5,
        "government preset should have at least 5 capabilities"
    );
}

#[test]
fn no_duplicate_capabilities() {
    use crate::Capability;
    use std::collections::HashSet;
    let presets = vec![
        research_assistant(),
        coding_agent(),
        customer_support(),
        data_analysis(),
        government_compliant("z"),
    ];
    for p in &presets {
        let mut seen: HashSet<String> = HashSet::new();
        for cap in &p.capabilities {
            let key = format!("{cap:?}");
            assert!(
                seen.insert(key.clone()),
                "duplicate capability {key} in preset {}",
                p.name
            );
        }
    }
}
