// Device health telemetry for edge fleet

use crate::registration::DeviceId;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// Health metric sample from a device
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HealthSample {
    pub device_id: DeviceId,
    pub timestamp: u64,
    pub cpu_percent: f32,
    pub memory_percent: f32,
    pub disk_percent: f32,
    pub temperature_celsius: Option<f32>,
    pub uptime_secs: u64,
    pub error_count: u32,
}

impl HealthSample {
    pub fn new(device_id: DeviceId) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        Self {
            device_id,
            timestamp: now,
            cpu_percent: 0.0,
            memory_percent: 0.0,
            disk_percent: 0.0,
            temperature_celsius: None,
            uptime_secs: 0,
            error_count: 0,
        }
    }

    pub fn with_metrics(
        mut self,
        cpu_percent: f32,
        memory_percent: f32,
        disk_percent: f32,
        uptime_secs: u64,
    ) -> Self {
        self.cpu_percent = cpu_percent;
        self.memory_percent = memory_percent;
        self.disk_percent = disk_percent;
        self.uptime_secs = uptime_secs;
        self
    }

    pub fn is_healthy(&self) -> bool {
        self.cpu_percent < 90.0
            && self.memory_percent < 90.0
            && self.disk_percent < 90.0
            && self.error_count == 0
    }
}

/// Alert level for health status
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum AlertLevel {
    Ok,
    Warning,
    Critical,
}

/// Fleet telemetry store — retains latest health sample per device
#[derive(Debug, Default)]
pub struct FleetTelemetry {
    latest: HashMap<DeviceId, HealthSample>,
    history: HashMap<DeviceId, Vec<HealthSample>>,
}

impl FleetTelemetry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Ingest a health sample from a device
    pub fn ingest(&mut self, sample: HealthSample) {
        self.history
            .entry(sample.device_id.clone())
            .or_default()
            .push(sample.clone());
        self.latest.insert(sample.device_id.clone(), sample);
    }

    pub fn latest_for(&self, device_id: &DeviceId) -> Option<&HealthSample> {
        self.latest.get(device_id)
    }

    pub fn history_for(&self, device_id: &DeviceId) -> Vec<&HealthSample> {
        self.history
            .get(device_id)
            .map(|v| v.iter().collect())
            .unwrap_or_default()
    }

    /// Devices currently reporting unhealthy metrics
    pub fn unhealthy_devices(&self) -> Vec<&DeviceId> {
        self.latest
            .iter()
            .filter(|(_, s)| !s.is_healthy())
            .map(|(id, _)| id)
            .collect()
    }

    /// Alert level for a device based on its latest sample
    pub fn alert_level(&self, device_id: &DeviceId) -> AlertLevel {
        match self.latest.get(device_id) {
            None => AlertLevel::Critical,
            Some(s) if s.cpu_percent > 95.0 || s.memory_percent > 95.0 => AlertLevel::Critical,
            Some(s) if s.cpu_percent > 80.0 || s.memory_percent > 80.0 => AlertLevel::Warning,
            Some(s) if s.error_count > 0 => AlertLevel::Warning,
            _ => AlertLevel::Ok,
        }
    }

    pub fn device_count(&self) -> usize {
        self.latest.len()
    }
}
