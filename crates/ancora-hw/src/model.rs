//! ancora-hw: Core data models for hardware-aware scheduling.
//!
//! Defines capability enumerations, hardware profiles, and model requirements
//! used throughout the crate.

use serde::{Deserialize, Serialize};

/// Enumeration of compute unit types available on a device.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComputeUnit {
    /// Standard x86/ARM CPU cores.
    Cpu,
    /// Discrete or integrated GPU (CUDA, ROCm, Metal).
    Gpu,
    /// Neural Processing Unit (Apple ANE, Qualcomm HTP, etc.).
    Npu,
}

/// CPU architecture classification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CpuArch {
    X86_64,
    Aarch64,
    Unknown,
}

/// GPU backend classification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GpuBackend {
    Metal,
    Cuda,
    Rocm,
    Vulkan,
    None,
}

/// NPU vendor / platform classification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NpuPlatform {
    /// Apple Neural Engine (M-series, A-series).
    AppleAne,
    /// Qualcomm Hexagon Tensor Processor (HTP).
    QualcommHtp,
    /// Generic NNAPI-exposed NPU.
    Nnapi,
    None,
}

/// Describes the full hardware capabilities of the current device.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareProfile {
    /// CPU architecture.
    pub cpu_arch: CpuArch,
    /// Logical (hyperthreaded) CPU core count.
    pub cpu_logical_cores: u32,
    /// Physical CPU core count (may equal logical on some platforms).
    pub cpu_physical_cores: u32,
    /// CPU base clock frequency in MHz (0 if unknown).
    pub cpu_freq_mhz: u32,
    /// Total system RAM in MiB.
    pub total_ram_mib: u64,
    /// GPU backend available on this device.
    pub gpu_backend: GpuBackend,
    /// Approximate GPU VRAM in MiB (0 if no discrete GPU).
    pub gpu_vram_mib: u64,
    /// NPU platform available on this device.
    pub npu_platform: NpuPlatform,
    /// Whether this device is identified as Apple Silicon (M-series).
    pub is_apple_silicon: bool,
    /// Whether an ARM NPU is detected.
    pub has_arm_npu: bool,
    /// Thermal pressure level (0 = nominal, 1 = fair, 2 = serious, 3 = critical).
    pub thermal_pressure: u8,
    /// Estimated power budget in watts (0 if unknown).
    pub power_budget_watts: u32,
}

impl HardwareProfile {
    /// Build a profile with all defaults set to sensible "no-hardware" values.
    pub fn default_safe() -> Self {
        HardwareProfile {
            cpu_arch: CpuArch::Unknown,
            cpu_logical_cores: 1,
            cpu_physical_cores: 1,
            cpu_freq_mhz: 0,
            total_ram_mib: 512,
            gpu_backend: GpuBackend::None,
            gpu_vram_mib: 0,
            npu_platform: NpuPlatform::None,
            is_apple_silicon: false,
            has_arm_npu: false,
            thermal_pressure: 0,
            power_budget_watts: 0,
        }
    }

    /// Returns true when the profile indicates at least one accelerator
    /// (GPU or NPU) is available.
    pub fn has_accelerator(&self) -> bool {
        self.gpu_backend != GpuBackend::None || self.npu_platform != NpuPlatform::None
    }

    /// Returns the set of available compute units.
    pub fn available_units(&self) -> Vec<ComputeUnit> {
        let mut units = vec![ComputeUnit::Cpu];
        if self.gpu_backend != GpuBackend::None {
            units.push(ComputeUnit::Gpu);
        }
        if self.npu_platform != NpuPlatform::None {
            units.push(ComputeUnit::Npu);
        }
        units
    }
}

/// Requirements that a model or workload places on hardware.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelRequirements {
    /// Model identifier / name.
    pub name: String,
    /// Minimum RAM needed to load the model, in MiB.
    pub min_ram_mib: u64,
    /// Minimum VRAM needed if GPU execution is desired, in MiB (0 = CPU only).
    pub min_vram_mib: u64,
    /// Whether the model can benefit from NPU offloading.
    pub npu_capable: bool,
    /// Parameter count in millions (used for heuristic sizing).
    pub params_millions: u32,
}

impl ModelRequirements {
    /// Create requirements for a CPU-only model.
    pub fn cpu_only(name: &str, min_ram_mib: u64, params_millions: u32) -> Self {
        ModelRequirements {
            name: name.to_owned(),
            min_ram_mib,
            min_vram_mib: 0,
            npu_capable: false,
            params_millions,
        }
    }

    /// Create requirements for a GPU-preferred model.
    pub fn gpu_preferred(
        name: &str,
        min_ram_mib: u64,
        min_vram_mib: u64,
        params_millions: u32,
    ) -> Self {
        ModelRequirements {
            name: name.to_owned(),
            min_ram_mib,
            min_vram_mib,
            npu_capable: false,
            params_millions,
        }
    }
}
