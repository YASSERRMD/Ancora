use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SecretStatus {
    Active,
    Rotated,
    Deleted,
    Expired,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SecretKind {
    Opaque,
    DatabaseCredential,
    ApiKey,
    TlsCertificate,
    SshKey,
}

#[derive(Debug, Clone)]
pub struct SecretVersion {
    pub version: u32,
    pub value: String,
    pub created_tick: u64,
    pub status: SecretStatus,
    pub metadata: HashMap<String, String>,
}

impl SecretVersion {
    pub fn new(version: u32, value: impl Into<String>, created_tick: u64) -> Self {
        Self {
            version,
            value: value.into(),
            created_tick,
            status: SecretStatus::Active,
            metadata: HashMap::new(),
        }
    }

    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    pub fn is_active(&self) -> bool { self.status == SecretStatus::Active }
}

#[derive(Debug, Clone)]
pub struct Secret {
    pub path: String,
    pub tenant_id: String,
    pub kind: SecretKind,
    pub versions: Vec<SecretVersion>,
    pub active_version: u32,
    pub ttl_ticks: Option<u64>,
    pub created_tick: u64,
}

impl Secret {
    pub fn new(path: impl Into<String>, tenant_id: impl Into<String>, kind: SecretKind, initial_value: impl Into<String>, tick: u64) -> Self {
        let v = SecretVersion::new(1, initial_value, tick);
        Self {
            path: path.into(),
            tenant_id: tenant_id.into(),
            kind,
            active_version: 1,
            versions: vec![v],
            ttl_ticks: None,
            created_tick: tick,
        }
    }

    pub fn with_ttl(mut self, ttl_ticks: u64) -> Self {
        self.ttl_ticks = Some(ttl_ticks);
        self
    }

    pub fn active_value(&self) -> Option<&str> {
        self.versions.iter()
            .find(|v| v.version == self.active_version && v.is_active())
            .map(|v| v.value.as_str())
    }

    pub fn version_count(&self) -> usize { self.versions.len() }

    pub fn is_expired(&self, current_tick: u64) -> bool {
        if let Some(ttl) = self.ttl_ticks {
            current_tick > self.created_tick + ttl
        } else {
            false
        }
    }
}
