//! ancora-hw: ARM NPU detection and scheduling hints.
//!
//! Supports Qualcomm Hexagon HTP, NNAPI-exposed NPUs, and generic ARM NPUs.
//! The detection is heuristic and offline -- no network calls.

use crate::model::{HardwareProfile, NpuPlatform};
use serde::{Deserialize, Serialize};

/// ARM NPU capability descriptor.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArmNpuCapability {
    pub platform: NpuPlatform,
    /// Estimated TOPS (Tera-Operations per Second); 0 if unknown.
    pub estimated_tops: u32,
    /// Whether mixed-precision (fp16+int8) is supported.
    pub mixed_precision: bool,
    /// Whether the NPU can handle the entire model or only some layers.
    pub whole_model: bool,
}

impl ArmNpuCapability {
    /// Placeholder capability when the NPU type is unknown.
    pub fn unknown() -> Self {
        ArmNpuCapability {
            platform: NpuPlatform::None,
            estimated_tops: 0,
            mixed_precision: false,
            whole_model: false,
        }
    }
}

/// Detect ARM NPU capability from the hardware profile.
///
/// Returns `None` when no ARM NPU is present.
pub fn detect_arm_npu_capability(hw: &HardwareProfile) -> Option<ArmNpuCapability> {
    if !hw.has_arm_npu {
        return None;
    }
    let cap = match &hw.npu_platform {
        NpuPlatform::AppleAne => ArmNpuCapability {
            platform: NpuPlatform::AppleAne,
            estimated_tops: 38, // M3 ANE ~38 TOPS
            mixed_precision: true,
            whole_model: false, // ANE handles supported ops only
        },
        NpuPlatform::QualcommHtp => ArmNpuCapability {
            platform: NpuPlatform::QualcommHtp,
            estimated_tops: 26, // Snapdragon 8 Gen 2
            mixed_precision: true,
            whole_model: true,
        },
        NpuPlatform::Nnapi => ArmNpuCapability {
            platform: NpuPlatform::Nnapi,
            estimated_tops: 10,
            mixed_precision: false,
            whole_model: false,
        },
        NpuPlatform::None => return None,
    };
    Some(cap)
}

/// Returns scheduling hints for the detected ARM NPU.
///
/// The hints are strings that a downstream scheduler can interpret.
pub fn arm_npu_scheduling_hints(cap: &ArmNpuCapability) -> Vec<String> {
    let mut hints = Vec::new();
    if cap.whole_model {
        hints.push("prefer-npu-whole-model".to_owned());
    } else {
        hints.push("prefer-npu-ops-only".to_owned());
    }
    if cap.mixed_precision {
        hints.push("allow-fp16-int8-mix".to_owned());
    }
    hints.push(format!("estimated-tops={}", cap.estimated_tops));
    hints
}
