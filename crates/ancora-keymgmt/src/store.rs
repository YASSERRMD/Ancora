use crate::key::{CryptoKey, KeyStatus};
use std::collections::HashMap;

#[derive(Debug)]
pub enum KeyStoreError {
    KeyNotFound(String),
    KeyAlreadyExists(String),
    KeyNotActive(String),
    KeyDestroyed(String),
}

impl std::fmt::Display for KeyStoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KeyStoreError::KeyNotFound(id) => write!(f, "key not found: {id}"),
            KeyStoreError::KeyAlreadyExists(id) => write!(f, "key already exists: {id}"),
            KeyStoreError::KeyNotActive(id) => write!(f, "key is not active: {id}"),
            KeyStoreError::KeyDestroyed(id) => write!(f, "key is destroyed: {id}"),
        }
    }
}

pub struct KeyStore {
    keys: HashMap<String, Vec<CryptoKey>>,
}

impl KeyStore {
    pub fn new() -> Self {
        Self {
            keys: HashMap::new(),
        }
    }

    fn store_key(&self, key: &CryptoKey) -> String {
        format!("{}:{}", key.tenant_id, key.id)
    }

    pub fn create(&mut self, key: CryptoKey) -> Result<(), KeyStoreError> {
        let k = self.store_key(&key);
        if self.keys.contains_key(&k) {
            return Err(KeyStoreError::KeyAlreadyExists(key.id.clone()));
        }
        self.keys.insert(k, vec![key]);
        Ok(())
    }

    pub fn get_active(&self, tenant_id: &str, id: &str) -> Result<&CryptoKey, KeyStoreError> {
        let k = format!("{tenant_id}:{id}");
        let versions = self
            .keys
            .get(&k)
            .ok_or_else(|| KeyStoreError::KeyNotFound(id.to_string()))?;
        versions
            .iter()
            .rev()
            .find(|kv| kv.status == KeyStatus::Active)
            .ok_or_else(|| KeyStoreError::KeyNotActive(id.to_string()))
    }

    pub fn get_version(
        &self,
        tenant_id: &str,
        id: &str,
        version: u32,
    ) -> Result<&CryptoKey, KeyStoreError> {
        let k = format!("{tenant_id}:{id}");
        let versions = self
            .keys
            .get(&k)
            .ok_or_else(|| KeyStoreError::KeyNotFound(id.to_string()))?;
        versions
            .iter()
            .find(|kv| kv.version == version)
            .ok_or_else(|| KeyStoreError::KeyNotFound(format!("{id}@v{version}")))
    }

    pub fn get_latest_mut(
        &mut self,
        tenant_id: &str,
        id: &str,
    ) -> Result<&mut CryptoKey, KeyStoreError> {
        let k = format!("{tenant_id}:{id}");
        let versions = self
            .keys
            .get_mut(&k)
            .ok_or_else(|| KeyStoreError::KeyNotFound(id.to_string()))?;
        versions
            .last_mut()
            .ok_or_else(|| KeyStoreError::KeyNotFound(id.to_string()))
    }

    pub fn add_version(
        &mut self,
        tenant_id: &str,
        id: &str,
        new_key: CryptoKey,
    ) -> Result<(), KeyStoreError> {
        let k = format!("{tenant_id}:{id}");
        let versions = self
            .keys
            .get_mut(&k)
            .ok_or_else(|| KeyStoreError::KeyNotFound(id.to_string()))?;
        versions.push(new_key);
        Ok(())
    }

    pub fn version_count(&self, tenant_id: &str, id: &str) -> usize {
        let k = format!("{tenant_id}:{id}");
        self.keys.get(&k).map_or(0, |v| v.len())
    }

    pub fn list_tenant_active(&self, tenant_id: &str) -> Vec<&CryptoKey> {
        self.keys
            .values()
            .flat_map(|v| v.iter())
            .filter(|k| k.tenant_id == tenant_id && k.is_active())
            .collect()
    }

    pub fn total_key_ids(&self) -> usize {
        self.keys.len()
    }
}

impl Default for KeyStore {
    fn default() -> Self {
        Self::new()
    }
}
