use std::collections::HashMap;
use crate::secret::{Secret, SecretKind, SecretStatus, SecretVersion};
use crate::error::SecretError;
use crate::validator::validate_path;

pub struct SecretStore {
    secrets: HashMap<String, Secret>,
}

impl SecretStore {
    pub fn new() -> Self { Self { secrets: HashMap::new() } }

    fn store_key(tenant_id: &str, path: &str) -> String {
        format!("{}:{}", tenant_id, path)
    }

    pub fn create(
        &mut self,
        tenant_id: impl Into<String>,
        path: impl Into<String>,
        kind: SecretKind,
        value: impl Into<String>,
        tick: u64,
    ) -> Result<(), SecretError> {
        let tenant_id = tenant_id.into();
        let path = path.into();
        validate_path(&path)?;
        let key = Self::store_key(&tenant_id, &path);
        if self.secrets.contains_key(&key) {
            return Err(SecretError::AlreadyExists(path));
        }
        self.secrets.insert(key, Secret::new(path, tenant_id, kind, value, tick));
        Ok(())
    }

    pub fn read(&self, tenant_id: &str, path: &str) -> Result<&Secret, SecretError> {
        let key = Self::store_key(tenant_id, path);
        self.secrets.get(&key).ok_or_else(|| SecretError::NotFound(path.to_string()))
    }

    pub fn read_mut(&mut self, tenant_id: &str, path: &str) -> Result<&mut Secret, SecretError> {
        let key = Self::store_key(tenant_id, path);
        self.secrets.get_mut(&key).ok_or_else(|| SecretError::NotFound(path.to_string()))
    }

    pub fn write_version(
        &mut self,
        tenant_id: &str,
        path: &str,
        new_value: impl Into<String>,
        tick: u64,
    ) -> Result<u32, SecretError> {
        let secret = self.read_mut(tenant_id, path)?;
        let next_version = secret.versions.last().map(|v| v.version + 1).unwrap_or(1);
        for v in &mut secret.versions {
            if v.is_active() { v.status = SecretStatus::Rotated; }
        }
        let new_v = SecretVersion::new(next_version, new_value, tick);
        secret.versions.push(new_v);
        secret.active_version = next_version;
        Ok(next_version)
    }

    pub fn delete(&mut self, tenant_id: &str, path: &str) -> Result<(), SecretError> {
        let key = Self::store_key(tenant_id, path);
        self.secrets.remove(&key).ok_or_else(|| SecretError::NotFound(path.to_string()))?;
        Ok(())
    }

    pub fn list_tenant(&self, tenant_id: &str) -> Vec<&Secret> {
        self.secrets.values()
            .filter(|s| s.tenant_id == tenant_id)
            .collect()
    }

    pub fn count(&self) -> usize { self.secrets.len() }
}

impl Default for SecretStore {
    fn default() -> Self { Self::new() }
}
