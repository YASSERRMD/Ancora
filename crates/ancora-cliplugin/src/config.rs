/// Plugin configuration - stores per-plugin key/value settings and provides
/// helpers for loading, merging, and validating configuration.

use std::collections::HashMap;

/// A single plugin's configuration block.
#[derive(Debug, Clone, Default)]
pub struct PluginConfig {
    /// The plugin id this config belongs to.
    pub plugin_id: String,
    /// Key/value settings.
    values: HashMap<String, String>,
}

impl PluginConfig {
    /// Create a new empty config for the given plugin id.
    pub fn new(plugin_id: impl Into<String>) -> Self {
        Self {
            plugin_id: plugin_id.into(),
            values: HashMap::new(),
        }
    }

    /// Set a configuration value.
    pub fn set(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.values.insert(key.into(), value.into());
    }

    /// Get a configuration value.
    pub fn get(&self, key: &str) -> Option<&str> {
        self.values.get(key).map(|s| s.as_str())
    }

    /// Return a value or a fallback default.
    pub fn get_or_default<'a>(&'a self, key: &str, default: &'a str) -> &'a str {
        self.values.get(key).map(|s| s.as_str()).unwrap_or(default)
    }

    /// Return all key/value pairs.
    pub fn entries(&self) -> impl Iterator<Item = (&str, &str)> {
        self.values.iter().map(|(k, v)| (k.as_str(), v.as_str()))
    }

    /// Merge another config into this one; incoming values overwrite existing keys.
    pub fn merge(&mut self, other: &PluginConfig) {
        for (k, v) in &other.values {
            self.values.insert(k.clone(), v.clone());
        }
    }

    /// Remove a key.
    pub fn remove(&mut self, key: &str) -> Option<String> {
        self.values.remove(key)
    }

    /// Number of configured keys.
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Whether the config has no keys.
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

/// The global configuration store, holding configs for all plugins.
#[derive(Debug, Clone, Default)]
pub struct ConfigStore {
    configs: HashMap<String, PluginConfig>,
}

impl ConfigStore {
    /// Create an empty store.
    pub fn new() -> Self {
        Self {
            configs: HashMap::new(),
        }
    }

    /// Insert or replace the config for a plugin.
    pub fn insert(&mut self, config: PluginConfig) {
        self.configs.insert(config.plugin_id.clone(), config);
    }

    /// Get an immutable reference to a plugin's config.
    pub fn get(&self, plugin_id: &str) -> Option<&PluginConfig> {
        self.configs.get(plugin_id)
    }

    /// Get a mutable reference to a plugin's config, creating it if absent.
    pub fn get_or_insert_mut(&mut self, plugin_id: &str) -> &mut PluginConfig {
        self.configs
            .entry(plugin_id.to_string())
            .or_insert_with(|| PluginConfig::new(plugin_id))
    }

    /// Merge a config into the store (creates the entry if it does not exist).
    pub fn merge(&mut self, incoming: PluginConfig) {
        let entry = self.get_or_insert_mut(&incoming.plugin_id.clone());
        entry.merge(&incoming);
    }

    /// Remove a plugin's config and return it.
    pub fn remove(&mut self, plugin_id: &str) -> Option<PluginConfig> {
        self.configs.remove(plugin_id)
    }

    /// Iterate over all plugin configs.
    pub fn iter(&self) -> impl Iterator<Item = (&str, &PluginConfig)> {
        self.configs.iter().map(|(k, v)| (k.as_str(), v))
    }

    /// Number of plugins that have config entries.
    pub fn len(&self) -> usize {
        self.configs.len()
    }

    /// Whether the store is empty.
    pub fn is_empty(&self) -> bool {
        self.configs.is_empty()
    }
}

/// A simple text-based config parser (key = value per line, # comments).
pub fn parse_config_text(plugin_id: &str, text: &str) -> PluginConfig {
    let mut cfg = PluginConfig::new(plugin_id);
    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('#') || trimmed.is_empty() {
            continue;
        }
        if let Some((key, val)) = trimmed.split_once('=') {
            cfg.set(key.trim(), val.trim().trim_matches('"'));
        }
    }
    cfg
}
