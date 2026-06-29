// Device registration and identity management for edge fleet

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// Unique device identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct DeviceId(pub String);

impl DeviceId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Device registration status
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum RegistrationStatus {
    Pending,
    Active,
    Revoked,
    Decommissioned,
}

/// Device identity record
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DeviceIdentity {
    pub id: DeviceId,
    pub name: String,
    pub fingerprint: String,
    pub registered_at: u64,
    pub status: RegistrationStatus,
    pub tags: HashMap<String, String>,
}

impl DeviceIdentity {
    pub fn new(id: DeviceId, name: impl Into<String>, fingerprint: impl Into<String>) -> Self {
        let registered_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        Self {
            id,
            name: name.into(),
            fingerprint: fingerprint.into(),
            registered_at,
            status: RegistrationStatus::Pending,
            tags: HashMap::new(),
        }
    }

    pub fn activate(&mut self) {
        self.status = RegistrationStatus::Active;
    }

    pub fn revoke(&mut self) {
        self.status = RegistrationStatus::Revoked;
    }

    pub fn decommission(&mut self) {
        self.status = RegistrationStatus::Decommissioned;
    }

    pub fn is_active(&self) -> bool {
        self.status == RegistrationStatus::Active
    }

    pub fn add_tag(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.tags.insert(key.into(), value.into());
    }
}

/// Registration request from a device
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RegistrationRequest {
    pub device_id: DeviceId,
    pub name: String,
    pub fingerprint: String,
    pub metadata: HashMap<String, String>,
}

/// Registration response from the fleet controller
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RegistrationResponse {
    pub success: bool,
    pub device_id: DeviceId,
    pub token: Option<String>,
    pub message: String,
}

/// Fleet device registry — stores all registered device identities
#[derive(Debug, Default)]
pub struct DeviceRegistry {
    devices: HashMap<DeviceId, DeviceIdentity>,
}

impl DeviceRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a new device; returns error if the device ID is already taken
    pub fn register(&mut self, req: RegistrationRequest) -> RegistrationResponse {
        if self.devices.contains_key(&req.device_id) {
            return RegistrationResponse {
                success: false,
                device_id: req.device_id,
                token: None,
                message: "device already registered".into(),
            };
        }

        let mut identity = DeviceIdentity::new(
            req.device_id.clone(),
            req.name,
            req.fingerprint,
        );
        for (k, v) in req.metadata {
            identity.add_tag(k, v);
        }
        identity.activate();

        let token = format!("tok-{}", &req.device_id.0);
        self.devices.insert(req.device_id.clone(), identity);

        RegistrationResponse {
            success: true,
            device_id: req.device_id,
            token: Some(token),
            message: "device registered successfully".into(),
        }
    }

    pub fn get(&self, id: &DeviceId) -> Option<&DeviceIdentity> {
        self.devices.get(id)
    }

    pub fn get_mut(&mut self, id: &DeviceId) -> Option<&mut DeviceIdentity> {
        self.devices.get_mut(id)
    }

    pub fn count(&self) -> usize {
        self.devices.len()
    }

    pub fn all_ids(&self) -> Vec<&DeviceId> {
        self.devices.keys().collect()
    }

    pub fn active_devices(&self) -> Vec<&DeviceIdentity> {
        self.devices.values().filter(|d| d.is_active()).collect()
    }
}
