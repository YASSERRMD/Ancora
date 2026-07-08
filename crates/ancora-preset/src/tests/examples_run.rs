use crate::{
    assemble, coding_agent, customer_support, data_analysis, government_compliant,
    research_assistant,
};

#[test]
fn research_example_runs() {
    let spec = assemble(&research_assistant()).expect("research");
    assert_eq!(spec.agent_id, "research-assistant");
}

#[test]
fn coding_example_runs() {
    let spec = assemble(&coding_agent()).expect("coding");
    assert_eq!(spec.agent_id, "coding-agent");
}

#[test]
fn support_example_runs() {
    let spec = assemble(&customer_support()).expect("support");
    assert_eq!(spec.agent_id, "customer-support");
}

#[test]
fn analysis_example_runs() {
    let spec = assemble(&data_analysis()).expect("analysis");
    assert_eq!(spec.agent_id, "data-analysis");
}

#[test]
fn government_example_runs() {
    let spec = assemble(&government_compliant("us-gov-east-1")).expect("government");
    assert_eq!(spec.agent_id, "government-compliant");
}
