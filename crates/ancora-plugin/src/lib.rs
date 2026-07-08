pub mod compatibility;
pub mod discovery;
pub mod exporter_ext;
pub mod extension_points;
pub mod grader_ext;
pub mod guardrail_ext;
/// Ancora Plugin SDK - stable extension points with versioning and scoping.
pub mod manifest;
pub mod memory_ext;
pub mod permission;
pub mod provider_ext;
pub mod tool_ext;
pub mod vectorstore_ext;

#[cfg(test)]
mod tests {
    mod test_grader_plugin_loads;
    mod test_incompatible_rejected;
    mod test_manifest_validates;
    mod test_provider_plugin_loads;
    mod test_scope_enforced;
    mod test_tool_plugin_loads;
}
