pub mod config;
pub mod discovery;
pub mod help;
/// ancora-cliplugin: CLI plugin system for the Ancora agent framework.
///
/// This crate provides the interfaces, registration, discovery, help integration,
/// configuration, permission enforcement, update checking, and list rendering
/// that enable CLIs in the Ancora ecosystem to accept third-party plugins.
pub mod interface;
pub mod list;
pub mod permissions;
pub mod registration;
pub mod sample;
pub mod update;

#[cfg(test)]
mod tests {
    mod test_appears_in_help;
    mod test_command_registers;
    mod test_config_honored;
    mod test_conflicting_handled;
    mod test_list_shows;
    mod test_permission_enforced;
    mod test_plugin_runs;
    mod test_sample_works;
    mod test_update_works;
}
