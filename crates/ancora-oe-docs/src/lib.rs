//! ancora-oe-docs: Observability and evaluation documentation for the Ancora agent framework.
//!
//! This crate provides typed models, platform primitives, and readiness tooling
//! for instrumenting and evaluating Ancora-based agent deployments.

pub mod ab_testing;
pub mod cont_eval;
pub mod cost_analytics;
pub mod datasets_graders;
pub mod dev_studio;
pub mod drift_mon;
pub mod eval_library;
pub mod evals_platform;
pub mod examples_index;
pub mod feedback_review;
pub mod obs_integrations;
pub mod overview;
pub mod per_lang;
pub mod readiness;
pub mod regression_gates;
pub mod safety_mon;
pub mod semantic_conv;
pub mod telemetry_priv;
pub mod trace_model;
pub mod troubleshooting;

#[cfg(test)]
mod tests {
    pub mod test_docs_structure;
    pub mod test_readiness_checklist;
}
