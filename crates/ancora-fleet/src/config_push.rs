// Configuration push to edge devices

use crate::registration::DeviceId;
use std::collections::HashMap;

/// A configuration payload to be pushed to devices
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DeviceConfig {
    pub version: u64,
    pub entries: HashMap<String, String>,
}

impl DeviceConfig {
    pub fn new(version: u64) -> Self {
        Self {
            version,
            entries: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.entries.insert(key.into(), value.into());
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.entries.get(key).map(|s| s.as_str())
    }
}

/// Status of a config push operation
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum PushStatus {
    Pending,
    Applied,
    Failed(String),
}

/// Record of a config push attempt
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ConfigPushRecord {
    pub device_id: DeviceId,
    pub config_version: u64,
    pub status: PushStatus,
}

/// Config push service — tracks which config version each device has applied
#[derive(Debug, Default)]
pub struct ConfigPushService {
    /// Maps device_id -> (applied_config_version, push_status)
    device_configs: HashMap<DeviceId, (u64, PushStatus)>,
}

impl ConfigPushService {
    pub fn new() -> Self {
        Self::default()
    }

    /// Push a config to a specific device; simulates delivery
    pub fn push(&mut self, device_id: &DeviceId, config: &DeviceConfig) -> ConfigPushRecord {
        // In a real system this would send over the wire; here we simulate success
        self.device_configs
            .insert(device_id.clone(), (config.version, PushStatus::Applied));

        ConfigPushRecord {
            device_id: device_id.clone(),
            config_version: config.version,
            status: PushStatus::Applied,
        }
    }

    /// Push a config to all provided devices
    pub fn push_to_fleet(
        &mut self,
        device_ids: &[DeviceId],
        config: &DeviceConfig,
    ) -> Vec<ConfigPushRecord> {
        device_ids.iter().map(|id| self.push(id, config)).collect()
    }

    pub fn applied_version(&self, device_id: &DeviceId) -> Option<u64> {
        self.device_configs.get(device_id).map(|(ver, _)| *ver)
    }

    pub fn status(&self, device_id: &DeviceId) -> Option<&PushStatus> {
        self.device_configs.get(device_id).map(|(_, s)| s)
    }

    /// Devices that have not yet applied the target version
    pub fn pending_devices<'a>(
        &self,
        target_version: u64,
        device_ids: &'a [DeviceId],
    ) -> Vec<&'a DeviceId> {
        device_ids
            .iter()
            .filter(|id| {
                self.device_configs
                    .get(*id)
                    .map(|(v, _)| *v < target_version)
                    .unwrap_or(true)
            })
            .collect()
    }
}
