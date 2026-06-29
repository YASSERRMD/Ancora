//! Memory-footprint metric for edge model evaluation.
//!
//! Re-exports and extends `MemoryFootprint` from the runtime module with
//! higher-level helpers for footprint budgeting across multiple models.

pub use crate::runtime::MemoryFootprint;

/// A budget constraint for edge memory.
#[derive(Debug, Clone)]
pub struct MemoryBudget {
    pub device_name: String,
    /// Total available RAM in MiB.
    pub total_mib: f64,
    /// Fraction reserved for the OS and other processes (0.0-1.0).
    pub os_reserve_fraction: f64,
}

impl MemoryBudget {
    pub fn new(device_name: impl Into<String>, total_mib: f64, os_reserve_fraction: f64) -> Self {
        Self {
            device_name: device_name.into(),
            total_mib,
            os_reserve_fraction: os_reserve_fraction.clamp(0.0, 0.99),
        }
    }

    /// Available MiB after reserving OS overhead.
    pub fn available_mib(&self) -> f64 {
        self.total_mib * (1.0 - self.os_reserve_fraction)
    }

    /// Check whether a footprint fits within budget.
    pub fn fits(&self, footprint: &MemoryFootprint) -> bool {
        footprint.total_mib() <= self.available_mib()
    }

    /// Headroom in MiB (negative means over budget).
    pub fn headroom_mib(&self, footprint: &MemoryFootprint) -> f64 {
        self.available_mib() - footprint.total_mib()
    }
}

/// Compare multiple models' footprints and return the smallest that meets a quality bar.
pub fn smallest_fitting<'a>(
    footprints: &'a [(String, MemoryFootprint)],
    budget: &MemoryBudget,
) -> Option<&'a str> {
    footprints
        .iter()
        .filter(|(_, fp)| budget.fits(fp))
        .min_by(|(_, a), (_, b)| a.total_bytes().cmp(&b.total_bytes()))
        .map(|(name, _)| name.as_str())
}
