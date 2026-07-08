//! ancora-hw: Offload policy -- decides which compute unit handles each layer.
//!
//! Supports CPU-only, GPU-only, and mixed CPU+GPU offloading strategies.

use crate::model::{GpuBackend, HardwareProfile};
use serde::{Deserialize, Serialize};

/// The offload destination for a model layer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LayerTarget {
    Cpu,
    Gpu,
    Npu,
}

/// Offload policy describing how layers are split between CPU and GPU.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OffloadPolicy {
    /// Total number of model layers.
    pub total_layers: u32,
    /// Number of layers assigned to the GPU (0 = CPU only).
    pub gpu_layers: u32,
    /// Number of layers assigned to the NPU (0 = not used).
    pub npu_layers: u32,
    /// Number of layers assigned to the CPU.
    pub cpu_layers: u32,
}

impl OffloadPolicy {
    /// Build a CPU-only policy (all layers on CPU).
    pub fn cpu_only(total_layers: u32) -> Self {
        OffloadPolicy {
            total_layers,
            gpu_layers: 0,
            npu_layers: 0,
            cpu_layers: total_layers,
        }
    }

    /// Build a GPU-only policy (all layers on GPU).
    pub fn gpu_only(total_layers: u32) -> Self {
        OffloadPolicy {
            total_layers,
            gpu_layers: total_layers,
            npu_layers: 0,
            cpu_layers: 0,
        }
    }

    /// Return the target for a given layer index.
    pub fn target_for_layer(&self, layer_idx: u32) -> LayerTarget {
        if layer_idx < self.npu_layers {
            LayerTarget::Npu
        } else if layer_idx < self.npu_layers + self.gpu_layers {
            LayerTarget::Gpu
        } else {
            LayerTarget::Cpu
        }
    }
}

/// Compute an offload policy given the hardware and model layer count.
///
/// Strategy:
/// 1. If no GPU is present, all layers go to CPU.
/// 2. If Apple Silicon, use total_ram_mib to estimate how many layers fit
///    in GPU (unified memory), keeping 25 % headroom.
/// 3. Otherwise, use gpu_vram_mib.
pub fn compute_offload_policy(
    hw: &HardwareProfile,
    total_layers: u32,
    bytes_per_layer_mib: u64,
) -> OffloadPolicy {
    if hw.gpu_backend == GpuBackend::None {
        return OffloadPolicy::cpu_only(total_layers);
    }

    let gpu_budget_mib = if hw.is_apple_silicon {
        (hw.total_ram_mib as f64 * 0.75) as u64
    } else {
        hw.gpu_vram_mib
    };

    let gpu_layer_capacity = gpu_budget_mib
        .checked_div(bytes_per_layer_mib)
        .map(|v| v.min(total_layers as u64) as u32)
        .unwrap_or(total_layers);

    let cpu_layers = total_layers - gpu_layer_capacity;

    OffloadPolicy {
        total_layers,
        gpu_layers: gpu_layer_capacity,
        npu_layers: 0,
        cpu_layers,
    }
}

/// Apply policy: return an ordered list of layer targets for all layers.
pub fn layer_assignments(policy: &OffloadPolicy) -> Vec<LayerTarget> {
    (0..policy.total_layers)
        .map(|i| policy.target_for_layer(i))
        .collect()
}
