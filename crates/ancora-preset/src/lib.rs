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
    mod airgap_routing_conflict;
    mod airgap_test;
    mod analysis_preset;
    mod assemble_tools;
    mod capability_list;
    mod coding_preset;
    mod coding_support_analysis;
    mod descriptor_fields;
    mod examples_run;
    mod government_preset;
    mod invalid_caps;
    mod invalid_name;
    mod locked_preset;
    mod multi_override;
    mod override_test;
    mod replace_override;
    mod research_preset;
    mod residency_test;
    mod support_preset;
    mod system_prompt;
    mod validation_test;
}
