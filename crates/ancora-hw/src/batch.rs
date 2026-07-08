//! ancora-hw: Batch size tuning based on hardware capabilities.
//!
//! Computes a safe maximum batch size that fits within available memory,
//! accounting for model size, context length, and hardware headroom.

use crate::model::HardwareProfile;
use serde::{Deserialize, Serialize};

/// Parameters used to compute a recommended batch size.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchConfig {
    /// Bytes consumed per token per batch item (activations + KV cache).
    pub bytes_per_token: u64,
    /// Maximum context length in tokens.
    pub context_len: u32,
    /// Static model footprint in MiB (weights loaded once).
    pub model_footprint_mib: u64,
    /// Headroom factor (0.0–1.0); fraction of free memory reserved for OS/other.
    pub headroom: f64,
}

impl Default for BatchConfig {
    fn default() -> Self {
        BatchConfig {
            bytes_per_token: 2 * 1024, // 2 KiB per token (fp16)
            context_len: 2048,
            model_footprint_mib: 0,
            headroom: 0.85,
        }
    }
}

/// Recommended batch configuration for a given hardware profile.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchRecommendation {
    /// Maximum safe batch size.
    pub max_batch_size: u32,
    /// Suggested batch size (conservative: 75 % of max).
    pub suggested_batch_size: u32,
    /// Available memory after model load in MiB.
    pub available_mib: u64,
}

/// Compute a batch recommendation from hardware and model config.
///
/// The function uses `total_ram_mib` on CPU/NPU paths and attempts to use
/// GPU VRAM on GPU paths.  Apple Silicon unified-memory devices always use
/// system RAM.
pub fn tune_batch_size(hw: &HardwareProfile, cfg: &BatchConfig) -> BatchRecommendation {
    let total_mib = if hw.is_apple_silicon || hw.gpu_vram_mib == 0 {
        hw.total_ram_mib
    } else {
        hw.gpu_vram_mib
    };

    let available_mib = if total_mib > cfg.model_footprint_mib {
        ((total_mib - cfg.model_footprint_mib) as f64 * cfg.headroom) as u64
    } else {
        0
    };

    // Each batch item requires: bytes_per_token * context_len bytes.
    let bytes_per_item = cfg.bytes_per_token * cfg.context_len as u64;
    let available_bytes = available_mib * 1024 * 1024;

    let max_batch_size = available_bytes
        .checked_div(bytes_per_item)
        .map(|v| v.max(1) as u32)
        .unwrap_or(1);

    let suggested_batch_size = ((max_batch_size as f64 * 0.75) as u32).max(1);

    BatchRecommendation {
        max_batch_size,
        suggested_batch_size,
        available_mib,
    }
}

/// Returns a simple batch size scaled by logical core count for CPU workloads.
pub fn cpu_batch_hint(hw: &HardwareProfile) -> u32 {
    hw.cpu_logical_cores.clamp(1, 64)
}
