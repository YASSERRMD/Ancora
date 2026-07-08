// Device decommission for edge fleet

use crate::registration::{DeviceId, DeviceRegistry};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// Reason for decommissioning a device
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum DecommissionReason {
    EndOfLife,
    Replaced,
    SecurityBreach,
    HardwareFailure,
    Other(String),
}

/// Record of a device decommission event
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DecommissionRecord {
    pub device_id: DeviceId,
    pub reason: DecommissionReason,
    pub decommissioned_at: u64,
    pub revoked: bool,
}

/// Decommission service — removes devices from active fleet
#[derive(Debug, Default)]
pub struct DecommissionService {
    records: HashMap<DeviceId, DecommissionRecord>,
}

impl DecommissionService {
    pub fn new() -> Self {
        Self::default()
    }

    /// Decommission a device: marks it in the registry and records the event
    pub fn decommission(
        &mut self,
        registry: &mut DeviceRegistry,
        device_id: &DeviceId,
        reason: DecommissionReason,
    ) -> Result<DecommissionRecord, String> {
        let identity = registry
            .get_mut(device_id)
            .ok_or_else(|| format!("device {:?} not found", device_id.as_str()))?;

        identity.decommission();

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let record = DecommissionRecord {
            device_id: device_id.clone(),
            reason,
            decommissioned_at: now,
            revoked: true,
        };

        self.records.insert(device_id.clone(), record.clone());
        Ok(record)
    }

    pub fn is_decommissioned(&self, device_id: &DeviceId) -> bool {
        self.records.contains_key(device_id)
    }

    pub fn record(&self, device_id: &DeviceId) -> Option<&DecommissionRecord> {
        self.records.get(device_id)
    }

    pub fn decommissioned_count(&self) -> usize {
        self.records.len()
    }

    /// List all decommissioned device IDs
    pub fn all_decommissioned(&self) -> Vec<&DeviceId> {
        self.records.keys().collect()
    }
}
