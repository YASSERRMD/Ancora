//! ancora-hw: Hardware profile configuration.
//!
//! Allows users to override detected hardware values via a JSON configuration
//! file (e.g., for edge deployments where runtime detection is not possible).

use crate::model::HardwareProfile;
use serde::{Deserialize, Serialize};

/// User-supplied overrides for a hardware profile.
///
/// Any field that is `None` keeps the probed value.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HardwareProfileOverride {
    pub cpu_logical_cores: Option<u32>,
    pub cpu_physical_cores: Option<u32>,
    pub total_ram_mib: Option<u64>,
    pub gpu_vram_mib: Option<u64>,
    pub thermal_pressure: Option<u8>,
    pub power_budget_watts: Option<u32>,
    pub is_apple_silicon: Option<bool>,
    pub has_arm_npu: Option<bool>,
}

impl HardwareProfileOverride {
    /// Apply overrides to a base profile, returning the merged result.
    pub fn apply(&self, mut base: HardwareProfile) -> HardwareProfile {
        if let Some(v) = self.cpu_logical_cores {
            base.cpu_logical_cores = v;
        }
        if let Some(v) = self.cpu_physical_cores {
            base.cpu_physical_cores = v;
        }
        if let Some(v) = self.total_ram_mib {
            base.total_ram_mib = v;
        }
        if let Some(v) = self.gpu_vram_mib {
            base.gpu_vram_mib = v;
        }
        if let Some(v) = self.thermal_pressure {
            base.thermal_pressure = v;
        }
        if let Some(v) = self.power_budget_watts {
            base.power_budget_watts = v;
        }
        if let Some(v) = self.is_apple_silicon {
            base.is_apple_silicon = v;
        }
        if let Some(v) = self.has_arm_npu {
            base.has_arm_npu = v;
        }
        base
    }
}

/// Parse a `HardwareProfileOverride` from a JSON string.
pub fn parse_override(json: &str) -> Result<HardwareProfileOverride, serde_json::Error> {
    serde_json::from_str(json)
}

/// Serialise a `HardwareProfile` to a JSON string.
pub fn serialize_profile(profile: &HardwareProfile) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(profile)
}
