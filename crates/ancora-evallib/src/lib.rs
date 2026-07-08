//! ancora-evallib - A catalog of reusable eval suites that run offline with a local judge.
//!
//! Each sub-module contains a self-contained suite of evaluation cases for a specific
//! capability dimension. All suites are executable without network access.

pub mod coordination;
pub mod cost_efficiency;
pub mod long_context;
pub mod multilingual;
pub mod rag_faithfulness;
pub mod reasoning;
pub mod routing;
pub mod runner;
pub mod safety;
pub mod tool_use;

#[cfg(test)]
mod tests {
    mod test_coord_suite;
    mod test_long_context_suite;
    mod test_multilingual_suite;
    mod test_offline_local_judge;
    mod test_rag_suite;
    mod test_reasoning_suite;
    mod test_routing_suite;
    mod test_safety_suite;
    mod test_tool_use_suite;
}
