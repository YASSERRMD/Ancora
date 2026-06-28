use crate::{assemble, coding_agent, customer_support, data_analysis};

#[test]
fn coding_agent_description_set() {
    let preset = coding_agent();
    assert!(!preset.description.is_empty());
}

#[test]
fn customer_support_description_set() {
    let preset = customer_support();
    assert!(!preset.description.is_empty());
}

#[test]
fn data_analysis_description_set() {
    let preset = data_analysis();
    assert!(!preset.description.is_empty());
}

#[test]
fn all_three_assemble_with_correct_ids() {
    let specs = [
        assemble(&coding_agent()).expect("coding"),
        assemble(&customer_support()).expect("support"),
        assemble(&data_analysis()).expect("analysis"),
    ];
    assert_eq!(specs[0].agent_id, "coding-agent");
    assert_eq!(specs[1].agent_id, "customer-support");
    assert_eq!(specs[2].agent_id, "data-analysis");
}
