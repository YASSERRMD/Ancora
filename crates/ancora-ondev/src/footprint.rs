//! Memory footprint measurement utilities for ARM targets.
//!
//! Tracks heap allocations and RSS growth during agent operations
//! to validate that the runtime stays within mobile memory budgets.

use crate::perf::MemorySnapshot;
use serde::{Deserialize, Serialize};

/// A footprint report captured before and after an operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FootprintReport {
    /// Snapshot before the operation.
    pub before: SnapRecord,
    /// Snapshot after the operation.
    pub after: SnapRecord,
    /// Delta RSS in bytes (signed; negative means RSS shrank).
    pub delta_rss_bytes: i64,
}

/// A labelled memory snapshot.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapRecord {
    /// Human-readable label.
    pub label: String,
    /// RSS in bytes.
    pub rss_bytes: usize,
}

impl FootprintReport {
    /// Capture a footprint report around the given closure.
    pub fn measure<F: FnOnce()>(label_before: &str, label_after: &str, f: F) -> Self {
        let snap_before = MemorySnapshot::capture();
        f();
        let snap_after = MemorySnapshot::capture();
        let delta = snap_after.rss_bytes as i64 - snap_before.rss_bytes as i64;
        Self {
            before: SnapRecord {
                label: label_before.to_string(),
                rss_bytes: snap_before.rss_bytes,
            },
            after: SnapRecord {
                label: label_after.to_string(),
                rss_bytes: snap_after.rss_bytes,
            },
            delta_rss_bytes: delta,
        }
    }

    /// Return the delta in kibibytes.
    pub fn delta_kib(&self) -> i64 {
        self.delta_rss_bytes / 1024
    }

    /// Return whether the operation grew RSS by less than `limit_mib` MiB.
    pub fn growth_within_mib(&self, limit_mib: u32) -> bool {
        self.delta_rss_bytes <= (limit_mib as i64) * 1024 * 1024
    }
}

/// Measure the footprint of initialising the on-device runtime components.
pub fn measure_runtime_footprint() -> FootprintReport {
    FootprintReport::measure("pre-runtime-init", "post-runtime-init", || {
        let _ = crate::journal::Journal::open();
        let _ = crate::memory::MemoryStore::new();
        let _ = crate::inference::LocalInferenceEngine::new(
            crate::inference::ModelBackend::LocalGguf {
                model_path: "/models/phi3.gguf".to_string(),
            },
            true,
        );
    })
}

#[cfg(test)]
mod unit {
    use super::*;

    #[test]
    fn footprint_report_growth_within_100mib() {
        let report = measure_runtime_footprint();
        assert!(
            report.growth_within_mib(100),
            "runtime init grew RSS by {} bytes",
            report.delta_rss_bytes
        );
    }

    #[test]
    fn footprint_report_delta_kib_consistent() {
        let report = FootprintReport {
            before: SnapRecord {
                label: "a".to_string(),
                rss_bytes: 1024 * 1024,
            },
            after: SnapRecord {
                label: "b".to_string(),
                rss_bytes: 2 * 1024 * 1024,
            },
            delta_rss_bytes: 1024 * 1024,
        };
        assert_eq!(report.delta_kib(), 1024);
        assert!(report.growth_within_mib(2));
        assert!(!report.growth_within_mib(0));
    }
}
