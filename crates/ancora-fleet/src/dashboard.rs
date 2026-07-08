// Fleet dashboard JSON output

use crate::inventory::FleetInventory;
use crate::registration::DeviceRegistry;
use crate::telemetry::FleetTelemetry;

/// Fleet-level aggregated dashboard data
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FleetDashboard {
    pub total_devices: usize,
    pub active_devices: usize,
    pub healthy_devices: usize,
    pub unhealthy_devices: usize,
    pub total_cpu_cores: u32,
    pub total_memory_mb: u64,
    pub total_disk_gb: u64,
    pub device_summaries: Vec<DeviceSummary>,
}

/// Per-device summary row in the dashboard
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DeviceSummary {
    pub device_id: String,
    pub name: String,
    pub is_active: bool,
    pub cpu_percent: f32,
    pub memory_percent: f32,
    pub disk_percent: f32,
    pub uptime_secs: u64,
    pub installed_models: Vec<String>,
}

/// Build the fleet dashboard JSON from the various service stores
pub fn build_dashboard(
    registry: &DeviceRegistry,
    inventory: &FleetInventory,
    telemetry: &FleetTelemetry,
) -> FleetDashboard {
    let total_devices = registry.count();
    let active_devices = registry.active_devices().len();

    let inv_summary = inventory.summary();

    let mut device_summaries: Vec<DeviceSummary> = Vec::new();
    let mut healthy_devices = 0usize;
    let mut unhealthy_devices = 0usize;

    for identity in registry.active_devices() {
        let telemetry_sample = telemetry.latest_for(&identity.id);
        let (cpu, mem, disk, uptime) = match telemetry_sample {
            Some(s) => (
                s.cpu_percent,
                s.memory_percent,
                s.disk_percent,
                s.uptime_secs,
            ),
            None => (0.0, 0.0, 0.0, 0),
        };

        let is_healthy = telemetry_sample.map(|s| s.is_healthy()).unwrap_or(true);
        if is_healthy {
            healthy_devices += 1;
        } else {
            unhealthy_devices += 1;
        }

        let installed_models = inventory
            .get(&identity.id)
            .map(|inv| inv.installed_models.clone())
            .unwrap_or_default();

        device_summaries.push(DeviceSummary {
            device_id: identity.id.0.clone(),
            name: identity.name.clone(),
            is_active: identity.is_active(),
            cpu_percent: cpu,
            memory_percent: mem,
            disk_percent: disk,
            uptime_secs: uptime,
            installed_models,
        });
    }

    FleetDashboard {
        total_devices,
        active_devices,
        healthy_devices,
        unhealthy_devices,
        total_cpu_cores: inv_summary.total_cpu_cores,
        total_memory_mb: inv_summary.total_memory_mb,
        total_disk_gb: inv_summary.total_disk_gb,
        device_summaries,
    }
}

/// Serialize the dashboard to a JSON string
pub fn dashboard_to_json(dashboard: &FleetDashboard) -> Result<String, String> {
    serde_json::to_string_pretty(dashboard).map_err(|e| e.to_string())
}
