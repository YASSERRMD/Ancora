/// Air-gap end-to-end: offline registry bundle workflow.

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct BundledPlugin {
    pub name: String,
    pub version: String,
    pub payload: Vec<u8>,
    pub checksum: String,
}

impl BundledPlugin {
    pub fn new(name: &str, version: &str, payload: Vec<u8>) -> Self {
        let checksum = format!("{:x}", payload.iter().fold(0u64, |acc, &b| acc.wrapping_add(b as u64)));
        BundledPlugin {
            name: name.to_string(),
            version: version.to_string(),
            payload,
            checksum,
        }
    }

    pub fn verify(&self) -> bool {
        let computed = format!(
            "{:x}",
            self.payload
                .iter()
                .fold(0u64, |acc, &b| acc.wrapping_add(b as u64))
        );
        computed == self.checksum
    }
}

#[derive(Debug, Default)]
pub struct AirgapBundle {
    plugins: HashMap<String, BundledPlugin>,
}

impl AirgapBundle {
    pub fn new() -> Self {
        AirgapBundle {
            plugins: HashMap::new(),
        }
    }

    pub fn add(&mut self, plugin: BundledPlugin) -> Result<(), String> {
        if !plugin.verify() {
            return Err(format!("checksum mismatch for plugin '{}'", plugin.name));
        }
        let key = format!("{}-{}", plugin.name, plugin.version);
        if self.plugins.contains_key(&key) {
            return Err(format!(
                "plugin '{}' v{} already in bundle",
                plugin.name, plugin.version
            ));
        }
        self.plugins.insert(key, plugin);
        Ok(())
    }

    pub fn get(&self, name: &str, version: &str) -> Option<&BundledPlugin> {
        self.plugins.get(&format!("{}-{}", name, version))
    }

    pub fn count(&self) -> usize {
        self.plugins.len()
    }
}

#[derive(Debug, Default)]
pub struct AirgapRegistry {
    bundle: AirgapBundle,
    installed: Vec<String>,
}

impl AirgapRegistry {
    pub fn new(bundle: AirgapBundle) -> Self {
        AirgapRegistry {
            bundle,
            installed: Vec::new(),
        }
    }

    pub fn install(&mut self, name: &str, version: &str) -> Result<(), String> {
        let plugin = self
            .bundle
            .get(name, version)
            .ok_or_else(|| format!("plugin '{}' v{} not in bundle", name, version))?;
        if !plugin.verify() {
            return Err(format!("integrity check failed for '{}'", name));
        }
        let key = format!("{}-{}", name, version);
        if !self.installed.contains(&key) {
            self.installed.push(key);
        }
        Ok(())
    }

    pub fn is_installed(&self, name: &str, version: &str) -> bool {
        self.installed.contains(&format!("{}-{}", name, version))
    }

    pub fn installed_count(&self) -> usize {
        self.installed.len()
    }
}
