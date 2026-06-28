pub mod assembly;
pub mod descriptor;
pub mod overrides;
pub mod presets;
pub mod validation;

pub use assembly::{assemble, AssemblyError};
pub use descriptor::{AirGapPolicy, Capability, PresetDescriptor, ResidencyConstraint};
pub use overrides::{apply_overrides, get_override};
pub use presets::{
    coding_agent, customer_support, data_analysis, government_compliant, research_assistant,
};
pub use validation::{validate, ValidationError};

#[cfg(test)]
mod tests {
    mod research_preset;
    mod coding_preset;
    mod support_preset;
    mod analysis_preset;
    mod government_preset;
    mod override_test;
    mod validation_test;
    mod airgap_test;
    mod examples_run;
    mod locked_preset;
    mod capability_list;
    mod descriptor_fields;
    mod assemble_tools;
    mod system_prompt;
    mod residency_test;
    mod multi_override;
    mod replace_override;
    mod invalid_name;
    mod invalid_caps;
    mod airgap_routing_conflict;
    mod coding_support_analysis;
}
