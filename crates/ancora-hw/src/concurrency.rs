//! ancora-hw: Concurrency limit calculation based on hardware capabilities.
//!
//! Computes the maximum number of concurrent inference requests that can be
//! safely handled without starving the OS or triggering thermal throttling.

use crate::model::HardwareProfile;
use crate::thermal::thermal_concurrency_scale;
use serde::{Deserialize, Serialize};

/// Parameters for concurrency limit calculation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConcurrencyConfig {
    /// Memory required per concurrent request in MiB.
    pub mem_per_request_mib: u64,
    /// Fraction of cores to dedicate to inference (0.0 – 1.0).
    pub core_fraction: f64,
    /// Minimum concurrency guaranteed regardless of hardware.
    pub min_concurrency: u32,
    /// Hard cap on concurrency regardless of hardware.
    pub max_concurrency: u32,
}

impl Default for ConcurrencyConfig {
    fn default() -> Self {
        ConcurrencyConfig {
            mem_per_request_mib: 256,
            core_fraction: 0.75,
            min_concurrency: 1,
            max_concurrency: 256,
        }
    }
}

/// The computed concurrency recommendation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConcurrencyLimit {
    /// Maximum safe concurrent requests.
    pub limit: u32,
    /// Number of worker threads recommended.
    pub worker_threads: u32,
    /// Explanation string.
    pub reason: String,
}

/// Compute a concurrency limit for the given hardware and configuration.
///
/// The limit is the minimum of:
/// 1. Memory-based limit (how many requests fit in available RAM).
/// 2. Core-based limit (fraction of logical cores).
/// 3. Thermally-scaled limit.
pub fn compute_concurrency_limit(hw: &HardwareProfile, cfg: &ConcurrencyConfig) -> ConcurrencyLimit {
    // Memory-based limit.
    let mem_limit = if cfg.mem_per_request_mib == 0 {
        cfg.max_concurrency
    } else {
        (hw.total_ram_mib / cfg.mem_per_request_mib).max(1) as u32
    };

    // Core-based limit.
    let core_limit = ((hw.cpu_logical_cores as f64 * cfg.core_fraction) as u32).max(1);

    // Thermal scaling.
    let thermal_scale = thermal_concurrency_scale(hw);
    let thermally_scaled = ((core_limit as f64 * thermal_scale) as u32).max(1);

    let raw_limit = mem_limit.min(thermally_scaled);
    let limit = raw_limit.clamp(cfg.min_concurrency, cfg.max_concurrency);
    let worker_threads = (hw.cpu_logical_cores as f64 * cfg.core_fraction).ceil() as u32;

    ConcurrencyLimit {
        limit,
        worker_threads: worker_threads.min(limit),
        reason: format!(
            "mem_limit={} core_limit={} thermal_scale={:.2}",
            mem_limit, thermally_scaled, thermal_scale
        ),
    }
}

/// Convenience: compute limit with default config.
pub fn default_concurrency_limit(hw: &HardwareProfile) -> ConcurrencyLimit {
    compute_concurrency_limit(hw, &ConcurrencyConfig::default())
}
