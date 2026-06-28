//! `ancora-ecodoc` - Ecosystem and extensibility documentation for Ancora.
//!
//! This crate contains the documentation modules, link checks, and readiness
//! checklist for the Ancora plugin ecosystem.

pub mod catalog_format;
pub mod cli_plugins;
pub mod contrib_templates;
pub mod examples_index;
pub mod extensibility_overview;
pub mod fw_adapters;
pub mod governance;
pub mod graph_builder;
pub mod interop_kit;
pub mod market_trust;
pub mod packaging;
pub mod plugin_safety;
pub mod plugin_sdk;
pub mod quickstart;
pub mod readiness;
pub mod recipes;
pub mod registry;
pub mod sdk_extensions;
pub mod security;
pub mod troubleshooting;

#[cfg(test)]
mod tests {
    mod test_docs_structure;
    mod test_link_check;
    mod test_readiness_checklist;
}
