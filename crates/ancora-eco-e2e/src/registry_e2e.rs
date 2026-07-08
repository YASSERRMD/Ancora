/// Registry end-to-end: publish, install, and list plugins from a local registry.
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct RegistryEntry {
    pub name: String,
    pub version: String,
    pub checksum: String,
    pub publisher: String,
}

impl RegistryEntry {
    pub fn new(name: &str, version: &str, publisher: &str) -> Self {
        let checksum = format!("{}-{}-{}", name, version, publisher.len());
        RegistryEntry {
            name: name.to_string(),
            version: version.to_string(),
            checksum,
            publisher: publisher.to_string(),
        }
    }
}

#[derive(Debug, Default)]
pub struct LocalRegistry {
    entries: HashMap<String, Vec<RegistryEntry>>,
}

impl LocalRegistry {
    pub fn new() -> Self {
        LocalRegistry {
            entries: HashMap::new(),
        }
    }

    pub fn publish(&mut self, entry: RegistryEntry) -> Result<(), String> {
        if entry.name.is_empty() {
            return Err("entry name must not be empty".to_string());
        }
        let versions = self.entries.entry(entry.name.clone()).or_default();
        let already = versions.iter().any(|e| e.version == entry.version);
        if already {
            return Err(format!(
                "version {} already published for {}",
                entry.version, entry.name
            ));
        }
        versions.push(entry);
        Ok(())
    }

    pub fn latest(&self, name: &str) -> Option<&RegistryEntry> {
        self.entries.get(name)?.last()
    }

    pub fn all_versions(&self, name: &str) -> Vec<&RegistryEntry> {
        self.entries
            .get(name)
            .map(|v| v.iter().collect())
            .unwrap_or_default()
    }

    pub fn list_all(&self) -> Vec<&RegistryEntry> {
        self.entries.values().flat_map(|v| v.iter()).collect()
    }

    pub fn remove(&mut self, name: &str, version: &str) -> bool {
        if let Some(versions) = self.entries.get_mut(name) {
            let before = versions.len();
            versions.retain(|e| e.version != version);
            return versions.len() < before;
        }
        false
    }
}
