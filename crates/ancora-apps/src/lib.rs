pub mod coding_assistant;
/// ancora-apps: A gallery of full sample applications.
///
/// All apps run offline. The compliance-review app supports
/// fully air-gapped government environments.
pub mod compliance_review;
pub mod customer_support;
pub mod data_analysis;
pub mod document_qa;
pub mod index;
pub mod local_models;
pub mod research_assistant;
pub mod safety;
pub mod traces;

#[cfg(test)]
mod tests {
    mod test_apps_emit_traces;
    mod test_apps_pass_guardrails;
    mod test_coding_offline;
    mod test_compliance_airgapped;
    mod test_dataanalysis_offline;
    mod test_docqa_offline;
    mod test_research_offline;
    mod test_support_offline;
}
