pub mod conformance;
pub mod exporter_template;
pub mod grader_template;
pub mod guardrail_template;
pub mod plugin_template;
/// ancora-contrib: community contribution templates
///
/// Scaffolding templates that produce testable, conformance-ready contributions
/// for every extension point in the ancora agent framework.
pub mod provider_template;
pub mod scaffolding;
pub mod tool_template;
pub mod vectorstore_template;

#[cfg(test)]
mod tests;
