//! In-memory plugin registry used during documentation builds.
//!
//! The registry maps plugin names to their catalog entries
//! and provides lookup helpers used by the doc toolchain.

use crate::catalog_format::CatalogEntry;
use std::collections::HashMap;

/// An in-memory registry of plugin catalog entries.
#[derive(Debug, Default)]
pub struct Registry {
    entries: HashMap<String, CatalogEntry>,
}

impl Registry {
    /// Create an empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a plugin. Returns an error if a plugin with the same name
    /// and version already exists.
    pub fn register(&mut self, entry: CatalogEntry) -> Result<(), String> {
        entry.validate()?;
        let key = format!("{}@{}", entry.name, entry.version);
        if self.entries.contains_key(&key) {
            return Err(format!("plugin {key} is already registered"));
        }
        self.entries.insert(key, entry);
        Ok(())
    }

    /// Look up all registered entries for a given plugin name.
    pub fn lookup(&self, name: &str) -> Vec<&CatalogEntry> {
        self.entries
            .values()
            .filter(|e| e.name == name)
            .collect()
    }

    /// Returns the total number of registered plugins.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns `true` if no plugins are registered.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog_format::CatalogEntry;

    fn entry(name: &str, ver: &str) -> CatalogEntry {
        CatalogEntry::new(name, ver, "desc", "author", "MIT", vec![])
    }

    #[test]
    fn register_and_lookup() {
        let mut reg = Registry::new();
        reg.register(entry("foo", "0.1.0")).unwrap();
        assert_eq!(reg.lookup("foo").len(), 1);
    }

    #[test]
    fn duplicate_registration_fails() {
        let mut reg = Registry::new();
        reg.register(entry("bar", "1.0.0")).unwrap();
        assert!(reg.register(entry("bar", "1.0.0")).is_err());
    }

    #[test]
    fn lookup_unknown_returns_empty() {
        let reg = Registry::new();
        assert!(reg.lookup("unknown").is_empty());
    }
}
