//! ancora-hw: Apple Silicon detection and tuning.
//!
//! Provides Apple Silicon-specific optimisations: unified memory accounting,
//! ANE (Apple Neural Engine) scheduling hints, and Metal GPU tuning.

use crate::model::{CpuArch, GpuBackend, HardwareProfile, NpuPlatform};
use crate::probe::{detect_cpu_arch, detect_cpu_cores, detect_total_ram_mib};
use serde::{Deserialize, Serialize};

/// Apple Silicon chip tier.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AppleSiliconTier {
    /// M-series (M1/M2/M3/M4 base).
    MBase,
    /// M-series Pro.
    MPro,
    /// M-series Max.
    MMax,
    /// M-series Ultra.
    MUltra,
    /// Not Apple Silicon.
    None,
}

/// Apple Silicon tuning recommendations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppleSiliconTuning {
    pub tier: AppleSiliconTier,
    /// Recommended fraction of unified memory to allocate to Metal/ANE.
    pub gpu_memory_fraction: f64,
    /// Whether to prefer ANE over Metal for small models.
    pub prefer_ane: bool,
    /// Recommended number of GPU command queues.
    pub gpu_command_queues: u32,
}

impl AppleSiliconTuning {
    /// Default safe tuning for any Apple Silicon device.
    pub fn default_apple_silicon() -> Self {
        AppleSiliconTuning {
            tier: AppleSiliconTier::MBase,
            gpu_memory_fraction: 0.70,
            prefer_ane: true,
            gpu_command_queues: 2,
        }
    }
}

/// Detect the Apple Silicon tier based on core count and RAM heuristics.
pub fn detect_apple_silicon_tier(hw: &HardwareProfile) -> AppleSiliconTier {
    if !hw.is_apple_silicon {
        return AppleSiliconTier::None;
    }
    match (hw.cpu_logical_cores, hw.total_ram_mib) {
        (c, _) if c >= 24 => AppleSiliconTier::MUltra,
        (c, _) if c >= 12 => AppleSiliconTier::MMax,
        (c, _) if c >= 10 => AppleSiliconTier::MPro,
        _ => AppleSiliconTier::MBase,
    }
}

/// Generate tuning recommendations for an Apple Silicon device.
///
/// Returns `None` when the device is not Apple Silicon.
pub fn apple_silicon_tuning(hw: &HardwareProfile) -> Option<AppleSiliconTuning> {
    if !hw.is_apple_silicon {
        return None;
    }
    let tier = detect_apple_silicon_tier(hw);
    let (gpu_frac, queues) = match &tier {
        AppleSiliconTier::MUltra => (0.80, 8),
        AppleSiliconTier::MMax => (0.75, 4),
        AppleSiliconTier::MPro => (0.72, 3),
        _ => (0.70, 2),
    };
    Some(AppleSiliconTuning {
        tier,
        gpu_memory_fraction: gpu_frac,
        prefer_ane: true,
        gpu_command_queues: queues,
    })
}

/// Build an Apple Silicon specific `HardwareProfile` for testing or overrides.
pub fn build_apple_silicon_profile(
    cpu_cores: u32,
    total_ram_mib: u64,
) -> HardwareProfile {
    HardwareProfile {
        cpu_arch: CpuArch::Aarch64,
        cpu_logical_cores: cpu_cores,
        cpu_physical_cores: cpu_cores,
        cpu_freq_mhz: 3200,
        total_ram_mib,
        gpu_backend: GpuBackend::Metal,
        gpu_vram_mib: 0, // unified memory
        npu_platform: NpuPlatform::AppleAne,
        is_apple_silicon: true,
        has_arm_npu: true,
        thermal_pressure: 0,
        power_budget_watts: 30,
    }
}
