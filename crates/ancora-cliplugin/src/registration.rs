/// Plugin command registration - maintains the registry of commands and their
/// owning plugins so the CLI dispatcher can resolve invocations.
use std::collections::HashMap;

use crate::interface::{
    CliPlugin, CommandSpec, ExecContext, ExecOutput, PluginError, PluginResult,
};

/// A handle to a registered plugin, including its resolved command index.
struct PluginEntry {
    plugin: Box<dyn CliPlugin>,
    /// Maps command name (and aliases) back to the canonical command name.
    name_index: HashMap<String, String>,
}

/// Central registry that maps command names to plugins.
pub struct PluginRegistry {
    entries: Vec<PluginEntry>,
    /// Global command name -> entry index to detect conflicts quickly.
    command_map: HashMap<String, usize>,
}

impl PluginRegistry {
    /// Create an empty registry.
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            command_map: HashMap::new(),
        }
    }

    /// Register a plugin, returning an error on any command name conflict.
    pub fn register(&mut self, plugin: Box<dyn CliPlugin>) -> PluginResult<()> {
        let commands = plugin.commands();
        let entry_idx = self.entries.len();

        let mut name_index: HashMap<String, String> = HashMap::new();

        for spec in &commands {
            // Build the full set of names (canonical + aliases).
            let mut all_names = vec![spec.name.clone()];
            all_names.extend_from_slice(&spec.aliases);

            for n in &all_names {
                if self.command_map.contains_key(n) {
                    return Err(PluginError::ConflictingCommand(n.clone()));
                }
                name_index.insert(n.clone(), spec.name.clone());
            }
        }

        // Commit: insert into the global map.
        for key in name_index.keys() {
            self.command_map.insert(key.clone(), entry_idx);
        }

        self.entries.push(PluginEntry { plugin, name_index });
        Ok(())
    }

    /// Dispatch a command invocation to the owning plugin.
    pub fn dispatch(&self, command: &str, ctx: ExecContext) -> PluginResult<ExecOutput> {
        let entry_idx =
            self.command_map.get(command).copied().ok_or_else(|| {
                PluginError::ExecutionFailed(format!("unknown command: {}", command))
            })?;

        let entry = &self.entries[entry_idx];
        let canonical = entry
            .name_index
            .get(command)
            .expect("index coherent with command_map");

        entry.plugin.execute(canonical, ctx)
    }

    /// Return all registered command specs from all plugins.
    pub fn all_commands(&self) -> Vec<&CommandSpec> {
        // We need to return references from the static lifetime of each plugin.
        // Build a temporary vec each call - acceptable for CLI help rendering.
        self.entries
            .iter()
            .flat_map(|e| {
                // SAFETY: The plugin is stored in a Box and lives as long as self.
                // We coerce the reference lifetime to 'self here.
                let cmds: Vec<&CommandSpec> = e
                    .plugin
                    .commands()
                    .iter()
                    .map(|_| {
                        // The plugin's commands() returns owned values; we cannot
                        // return references into them directly. We store nothing
                        // extra here, so we document this as a design note:
                        // callers should use `all_command_specs()` for owned data.
                        unimplemented!("use all_command_specs for owned data")
                    })
                    .collect();
                cmds
            })
            .collect()
    }

    /// Return all registered command specs (owned) from all plugins.
    pub fn all_command_specs(&self) -> Vec<CommandSpec> {
        self.entries
            .iter()
            .flat_map(|e| e.plugin.commands())
            .collect()
    }

    /// Return the number of registered plugins.
    pub fn plugin_count(&self) -> usize {
        self.entries.len()
    }

    /// Return the number of registered command names (including aliases).
    pub fn command_name_count(&self) -> usize {
        self.command_map.len()
    }

    /// Check whether a command name is registered.
    pub fn has_command(&self, name: &str) -> bool {
        self.command_map.contains_key(name)
    }
}

impl Default for PluginRegistry {
    fn default() -> Self {
        Self::new()
    }
}
