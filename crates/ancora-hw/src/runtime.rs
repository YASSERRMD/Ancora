//! ancora-hw: Runtime scheduling engine.
//!
//! Combines probe, fit, batch, offload, thermal, and concurrency modules into
//! a single deterministic scheduling decision given a hardware profile.

use crate::batch::{tune_batch_size, BatchConfig, BatchRecommendation};
use crate::concurrency::{compute_concurrency_limit, ConcurrencyConfig, ConcurrencyLimit};
use crate::fit::{check_fit, FitResult};
use crate::model::{HardwareProfile, ModelRequirements};
use crate::offload::{compute_offload_policy, OffloadPolicy};
use crate::thermal::thermal_concurrency_scale;
use serde::{Deserialize, Serialize};

/// A complete scheduling decision for a model on the current hardware.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchedulingDecision {
    pub fit: FitResult,
    pub batch: BatchRecommendation,
    pub offload: OffloadPolicy,
    pub concurrency: ConcurrencyLimit,
    pub thermal_scale: f64,
}

/// Produce a deterministic scheduling decision from a profile + requirements.
///
/// Given the same `hw` and `req` inputs, this function always returns the same
/// output (no randomness, no I/O, no system calls).
pub fn schedule(
    hw: &HardwareProfile,
    req: &ModelRequirements,
    total_layers: u32,
    bytes_per_layer_mib: u64,
) -> SchedulingDecision {
    let fit = check_fit(hw, req);

    let batch_cfg = BatchConfig {
        model_footprint_mib: req.min_ram_mib,
        ..BatchConfig::default()
    };
    let batch = tune_batch_size(hw, &batch_cfg);

    let offload = compute_offload_policy(hw, total_layers, bytes_per_layer_mib);

    let concurrency = compute_concurrency_limit(hw, &ConcurrencyConfig::default());

    let thermal_scale = thermal_concurrency_scale(hw);

    SchedulingDecision {
        fit,
        batch,
        offload,
        concurrency,
        thermal_scale,
    }
}

/// Overhead measurement: returns the number of nanoseconds taken to run
/// `schedule` once on the given inputs.
pub fn measure_schedule_overhead_ns(
    hw: &HardwareProfile,
    req: &ModelRequirements,
    total_layers: u32,
    bytes_per_layer_mib: u64,
) -> u128 {
    use std::time::Instant;
    let start = Instant::now();
    let _ = schedule(hw, req, total_layers, bytes_per_layer_mib);
    start.elapsed().as_nanos()
}
