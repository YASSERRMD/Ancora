//! ancora-pkg: Packaging templates for SaaS, on-prem, air-gapped, and edge products.
//!
//! This crate provides scaffolding templates and a CLI for generating secure-by-default
//! deployment configurations across all Ancora product delivery modes.

pub mod saas_template;
pub mod onprem_template;
pub mod airgap_template;
pub mod compose_template;
pub mod k8s_template;
pub mod edge_template;
pub mod whitelabel;
pub mod tenant_onboard;
pub mod cli;

#[cfg(test)]
mod tests {
    mod test_saas_renders;
    mod test_onprem_renders;
    mod test_airgap_renders;
    mod test_compose_runs;
    mod test_k8s_renders;
    mod test_edge_builds;
    mod test_whitelabel_applies;
    mod test_security_defaults;
    mod test_cli_scaffolds;
}
