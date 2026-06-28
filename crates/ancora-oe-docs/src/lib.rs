//! ancora-oe-docs: Observability and evaluation documentation for the Ancora agent framework.
//!
//! This crate provides typed models, platform primitives, and readiness tooling
//! for instrumenting and evaluating Ancora-based agent deployments.

pub mod overview;
pub mod trace_model;
pub mod semantic_conv;
pub mod cost_analytics;
pub mod drift_mon;
pub mod safety_mon;
pub mod telemetry_priv;
pub mod evals_platform;
pub mod datasets_graders;
pub mod regression_gates;
pub mod ab_testing;
pub mod feedback_review;
pub mod cont_eval;
pub mod dev_studio;
pub mod obs_integrations;
pub mod per_lang;
pub mod eval_library;
pub mod troubleshooting;
pub mod examples_index;
pub mod readiness;

#[cfg(test)]
mod tests {
    pub mod test_docs_structure;
    pub mod test_readiness_checklist;
}
