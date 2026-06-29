// Air-gapped fleet management via offline bundles

use std::collections::HashMap;
use crate::registration::DeviceId;

/// An offline bundle for air-gapped device fleet updates
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OfflineBundle {
    pub bundle_id: String,
    pub created_at: u64,
    pub description: String,
    /// Bundle contents: filename -> byte content (simulated as Vec<u8>)
    pub files: HashMap<String, Vec<u8>>,
    /// Manifest: artifact_id -> checksum
    pub manifest: HashMap<String, String>,
}

impl OfflineBundle {
    pub fn new(bundle_id: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            bundle_id: bundle_id.into(),
            created_at: 0,
            description: description.into(),
            files: HashMap::new(),
            manifest: HashMap::new(),
        }
    }

    pub fn add_file(&mut self, name: impl Into<String>, content: Vec<u8>) {
        let name = name.into();
        let checksum = simple_checksum(&content);
        self.manifest.insert(name.clone(), checksum);
        self.files.insert(name, content);
    }

    pub fn verify(&self) -> bool {
        self.manifest.iter().all(|(name, expected_checksum)| {
            self.files
                .get(name)
                .map(|content| simple_checksum(content) == *expected_checksum)
                .unwrap_or(false)
        })
    }

    pub fn file_count(&self) -> usize {
        self.files.len()
    }
}

/// Simple deterministic checksum (sum of bytes as hex) — NOT cryptographic, just for tests
fn simple_checksum(data: &[u8]) -> String {
    let sum: u64 = data.iter().map(|b| *b as u64).sum();
    format!("{:016x}", sum)
}

/// Status of bundle application on a device
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum BundleApplyStatus {
    Pending,
    Applied,
    VerificationFailed,
    Error(String),
}

/// Record of an offline bundle application
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BundleApplyRecord {
    pub device_id: DeviceId,
    pub bundle_id: String,
    pub status: BundleApplyStatus,
}

/// Air-gap fleet manager — manages offline bundle creation and application
#[derive(Debug, Default)]
pub struct AirGapFleetManager {
    bundles: HashMap<String, OfflineBundle>,
    /// (device_id, bundle_id) -> BundleApplyRecord
    apply_records: HashMap<(String, String), BundleApplyRecord>,
}

impl AirGapFleetManager {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a bundle in the manager
    pub fn add_bundle(&mut self, bundle: OfflineBundle) {
        self.bundles.insert(bundle.bundle_id.clone(), bundle);
    }

    pub fn get_bundle(&self, bundle_id: &str) -> Option<&OfflineBundle> {
        self.bundles.get(bundle_id)
    }

    /// Apply a bundle to a device; verifies bundle integrity first
    pub fn apply_bundle(
        &mut self,
        device_id: &DeviceId,
        bundle_id: &str,
    ) -> BundleApplyRecord {
        let status = match self.bundles.get(bundle_id) {
            None => BundleApplyStatus::Error(format!("bundle {} not found", bundle_id)),
            Some(bundle) => {
                if bundle.verify() {
                    BundleApplyStatus::Applied
                } else {
                    BundleApplyStatus::VerificationFailed
                }
            }
        };

        let record = BundleApplyRecord {
            device_id: device_id.clone(),
            bundle_id: bundle_id.to_string(),
            status,
        };

        self.apply_records.insert(
            (device_id.0.clone(), bundle_id.to_string()),
            record.clone(),
        );
        record
    }

    /// Apply a bundle to all devices in the fleet
    pub fn apply_to_fleet(
        &mut self,
        device_ids: &[DeviceId],
        bundle_id: &str,
    ) -> Vec<BundleApplyRecord> {
        let ids: Vec<DeviceId> = device_ids.to_vec();
        ids.iter()
            .map(|id| self.apply_bundle(id, bundle_id))
            .collect()
    }

    pub fn apply_status(
        &self,
        device_id: &DeviceId,
        bundle_id: &str,
    ) -> Option<&BundleApplyStatus> {
        let key = (device_id.0.clone(), bundle_id.to_string());
        self.apply_records.get(&key).map(|r| &r.status)
    }

    pub fn bundle_count(&self) -> usize {
        self.bundles.len()
    }
}
