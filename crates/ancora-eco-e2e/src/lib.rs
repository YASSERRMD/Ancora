pub mod adapter_e2e;
pub mod airgap_e2e;
pub mod builder_e2e;
pub mod catalog_e2e;
pub mod cliplugin_e2e;
pub mod parity_e2e;
pub mod perf;
pub mod plan;
pub mod plugin_e2e;
pub mod recipe_e2e;
pub mod registry_e2e;
pub mod trust_e2e;

#[cfg(test)]
mod tests {
    mod test_adapter_tool;
    mod test_airgap;
    mod test_all_offline;
    mod test_builder_graph;
    mod test_catalog_install;
    mod test_cli_plugin;
    mod test_crash_isolated;
    mod test_crash_isolation_detail;
    mod test_determinism_replay;
    mod test_install_from_registry;
    mod test_parity;
    mod test_plugin_from_template;
    mod test_plugin_interop_kit;
    mod test_publish_to_registry;
    mod test_recipe_runs;
    mod test_sandboxed;
    mod test_trust_blocks;
}
