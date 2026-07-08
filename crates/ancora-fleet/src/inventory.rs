// Device inventory management for edge fleet

use crate::registration::DeviceId;
use std::collections::HashMap;

/// Device hardware and software inventory snapshot
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DeviceInventory {
    pub device_id: DeviceId,
    pub hostname: String,
    pub os: String,
    pub arch: String,
    pub cpu_cores: u32,
    pub memory_mb: u64,
    pub disk_gb: u64,
    pub firmware_version: String,
    pub agent_version: String,
    pub installed_models: Vec<String>,
    pub extra: HashMap<String, String>,
}

impl DeviceInventory {
    pub fn new(device_id: DeviceId, hostname: impl Into<String>) -> Self {
        Self {
            device_id,
            hostname: hostname.into(),
            os: "unknown".into(),
            arch: "unknown".into(),
            cpu_cores: 0,
            memory_mb: 0,
            disk_gb: 0,
            firmware_version: "0.0.0".into(),
            agent_version: "0.1.0".into(),
            installed_models: Vec::new(),
            extra: HashMap::new(),
        }
    }

    pub fn with_os(mut self, os: impl Into<String>, arch: impl Into<String>) -> Self {
        self.os = os.into();
        self.arch = arch.into();
        self
    }

    pub fn with_resources(mut self, cpu_cores: u32, memory_mb: u64, disk_gb: u64) -> Self {
        self.cpu_cores = cpu_cores;
        self.memory_mb = memory_mb;
        self.disk_gb = disk_gb;
        self
    }

    pub fn add_model(&mut self, model: impl Into<String>) {
        self.installed_models.push(model.into());
    }

    pub fn has_model(&self, model: &str) -> bool {
        self.installed_models.iter().any(|m| m == model)
    }
}

/// Fleet inventory store — tracks inventory for all devices
#[derive(Debug, Default)]
pub struct FleetInventory {
    records: HashMap<DeviceId, DeviceInventory>,
}

impl FleetInventory {
    pub fn new() -> Self {
        Self::default()
    }

    /// Upsert inventory for a device
    pub fn update(&mut self, inventory: DeviceInventory) {
        self.records.insert(inventory.device_id.clone(), inventory);
    }

    pub fn get(&self, id: &DeviceId) -> Option<&DeviceInventory> {
        self.records.get(id)
    }

    pub fn count(&self) -> usize {
        self.records.len()
    }

    /// Find devices that have a specific model installed
    pub fn devices_with_model(&self, model: &str) -> Vec<&DeviceInventory> {
        self.records
            .values()
            .filter(|inv| inv.has_model(model))
            .collect()
    }

    /// Find devices by OS
    pub fn devices_by_os(&self, os: &str) -> Vec<&DeviceInventory> {
        self.records.values().filter(|inv| inv.os == os).collect()
    }

    /// Summary: total devices, total CPU cores, total memory
    pub fn summary(&self) -> InventorySummary {
        let total_devices = self.records.len();
        let total_cpu_cores: u32 = self.records.values().map(|d| d.cpu_cores).sum();
        let total_memory_mb: u64 = self.records.values().map(|d| d.memory_mb).sum();
        let total_disk_gb: u64 = self.records.values().map(|d| d.disk_gb).sum();
        InventorySummary {
            total_devices,
            total_cpu_cores,
            total_memory_mb,
            total_disk_gb,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct InventorySummary {
    pub total_devices: usize,
    pub total_cpu_cores: u32,
    pub total_memory_mb: u64,
    pub total_disk_gb: u64,
}
