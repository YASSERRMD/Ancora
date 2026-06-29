//! ancora-hw: Model-to-hardware fit check.
//!
//! Determines whether a model's requirements can be satisfied by the
//! current hardware profile, and which compute unit should host it.

use crate::model::{ComputeUnit, GpuBackend, HardwareProfile, ModelRequirements, NpuPlatform};
use serde::{Deserialize, Serialize};

/// Result of a fit check between model requirements and hardware.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FitResult {
    /// The model fits and should run on this compute unit.
    Fits(ComputeUnit),
    /// The model does not fit; contains the reason.
    DoesNotFit(FitRejectionReason),
}

/// Reasons why a model cannot run on the available hardware.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FitRejectionReason {
    InsufficientRam,
    InsufficientVram,
    NoCompatibleBackend,
}

/// Check whether `req` fits on `hw` and return the best compute unit.
///
/// Priority order: NPU > GPU > CPU.
pub fn check_fit(hw: &HardwareProfile, req: &ModelRequirements) -> FitResult {
    // Always check minimum RAM first.
    if hw.total_ram_mib < req.min_ram_mib {
        return FitResult::DoesNotFit(FitRejectionReason::InsufficientRam);
    }

    // Try NPU first (lowest power, highest throughput for capable models).
    if req.npu_capable && hw.npu_platform != NpuPlatform::None {
        return FitResult::Fits(ComputeUnit::Npu);
    }

    // Try GPU next.
    if hw.gpu_backend != GpuBackend::None && req.min_vram_mib > 0 {
        // For Apple Silicon the GPU shares system memory; use total_ram_mib.
        let effective_vram = if hw.is_apple_silicon {
            hw.total_ram_mib
        } else {
            hw.gpu_vram_mib
        };
        if effective_vram >= req.min_vram_mib {
            return FitResult::Fits(ComputeUnit::Gpu);
        } else {
            return FitResult::DoesNotFit(FitRejectionReason::InsufficientVram);
        }
    }

    // CPU fallback: already checked RAM above.
    FitResult::Fits(ComputeUnit::Cpu)
}

/// Returns true when the model can run on the given hardware.
pub fn can_run(hw: &HardwareProfile, req: &ModelRequirements) -> bool {
    !matches!(check_fit(hw, req), FitResult::DoesNotFit(_))
}
