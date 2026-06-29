// Model distribution to edge devices

use std::collections::HashMap;
use crate::registration::DeviceId;

/// Describes a model artifact to distribute
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ModelArtifact {
    pub model_id: String,
    pub version: String,
    pub size_bytes: u64,
    pub checksum: String, // SHA-256 hex
}

impl ModelArtifact {
    pub fn new(
        model_id: impl Into<String>,
        version: impl Into<String>,
        size_bytes: u64,
        checksum: impl Into<String>,
    ) -> Self {
        Self {
            model_id: model_id.into(),
            version: version.into(),
            size_bytes,
            checksum: checksum.into(),
        }
    }
}

/// Status of a model distribution to a specific device
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum DistributionStatus {
    Queued,
    Transferred,
    Verified,
    Failed(String),
}

/// Record of a model distribution attempt
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DistributionRecord {
    pub device_id: DeviceId,
    pub model_id: String,
    pub model_version: String,
    pub status: DistributionStatus,
}

/// Model distribution service
#[derive(Debug, Default)]
pub struct ModelDistributionService {
    /// (device_id, model_id) -> DistributionRecord
    records: HashMap<(String, String), DistributionRecord>,
}

impl ModelDistributionService {
    pub fn new() -> Self {
        Self::default()
    }

    /// Distribute a model artifact to a single device; simulates transfer + verify
    pub fn distribute(
        &mut self,
        device_id: &DeviceId,
        artifact: &ModelArtifact,
    ) -> DistributionRecord {
        let key = (device_id.0.clone(), artifact.model_id.clone());

        // Simulate: transfer then verify (checksum non-empty => verified)
        let status = if artifact.checksum.is_empty() {
            DistributionStatus::Failed("checksum missing".into())
        } else {
            DistributionStatus::Verified
        };

        let record = DistributionRecord {
            device_id: device_id.clone(),
            model_id: artifact.model_id.clone(),
            model_version: artifact.version.clone(),
            status,
        };

        self.records.insert(key, record.clone());
        record
    }

    /// Distribute a model to all provided devices
    pub fn distribute_to_fleet(
        &mut self,
        device_ids: &[DeviceId],
        artifact: &ModelArtifact,
    ) -> Vec<DistributionRecord> {
        device_ids
            .iter()
            .map(|id| self.distribute(id, artifact))
            .collect()
    }

    pub fn status(&self, device_id: &DeviceId, model_id: &str) -> Option<&DistributionStatus> {
        let key = (device_id.0.clone(), model_id.to_string());
        self.records.get(&key).map(|r| &r.status)
    }

    pub fn is_verified(&self, device_id: &DeviceId, model_id: &str) -> bool {
        self.status(device_id, model_id)
            .map(|s| *s == DistributionStatus::Verified)
            .unwrap_or(false)
    }

    /// Devices that have verified the model
    pub fn verified_devices(&self, model_id: &str) -> Vec<&DeviceId> {
        self.records
            .values()
            .filter(|r| r.model_id == model_id && r.status == DistributionStatus::Verified)
            .map(|r| &r.device_id)
            .collect()
    }
}
