pub mod code_review;
pub mod customer_support;
pub mod data_extraction;
pub mod debate;
pub mod doc_processing;
/// Parameterized workflow recipes for common agent patterns.
/// All recipes run offline and can be installed into projects.
pub mod format;
pub mod install;
pub mod params;
pub mod rag_citations;
pub mod research_report;

#[cfg(test)]
mod tests {
    mod test_code_review_runs;
    mod test_debate_runs;
    mod test_extraction_runs;
    mod test_install_cmd;
    mod test_offline;
    mod test_params_apply;
    mod test_rag_runs;
    mod test_research_runs;
    mod test_support_runs;
}
