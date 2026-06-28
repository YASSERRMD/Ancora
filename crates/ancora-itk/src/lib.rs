/// ancora-itk - Interop Test Kit
///
/// Certifies extensions against conformance with pass-fail reports and badges.

pub mod badge;
pub mod exporter_kit;
pub mod grader_kit;
pub mod guardrail_kit;
pub mod plugin_kit;
pub mod provider_kit;
pub mod report;
pub mod runner;
pub mod tool_kit;
pub mod vectorstore_kit;

#[cfg(test)]
mod tests {
    mod test_badge_issued;
    mod test_grader_kit;
    mod test_kit_fails_broken;
    mod test_provider_kit;
    mod test_report_generated;
    mod test_runner_cli;
    mod test_tool_kit;
    mod test_vector_kit;
}
