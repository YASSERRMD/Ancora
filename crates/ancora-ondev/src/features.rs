//! Reduced-footprint feature flag registry for on-device builds.
//!
//! Allows selective enabling/disabling of runtime capabilities to keep
//! binary size within mobile platform budgets.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A named capability that can be toggled at compile or runtime.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Feature {
    /// Embedded SQLite journal for agent state persistence.
    SqliteJournal,
    /// In-process LanceDB vector memory store.
    LanceDbMemory,
    /// Local-only inference enforcement (blocks remote calls).
    LocalOnlyInference,
    /// Cold-start performance instrumentation.
    ColdStartPerfMonitor,
    /// JNI bridge for Android.
    AndroidJni,
    /// iOS C-ABI exports.
    IosCabi,
    /// Structured logging to stderr.
    StructuredLogging,
    /// ARM NEON SIMD acceleration.
    NeonSimd,
}

impl std::fmt::Display for Feature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::SqliteJournal => "sqlite-journal",
            Self::LanceDbMemory => "lancedb-memory",
            Self::LocalOnlyInference => "local-only-inference",
            Self::ColdStartPerfMonitor => "cold-start-perf-monitor",
            Self::AndroidJni => "android-jni",
            Self::IosCabi => "ios-cabi",
            Self::StructuredLogging => "structured-logging",
            Self::NeonSimd => "neon-simd",
        };
        write!(f, "{}", s)
    }
}

/// Priority class for a feature (determines inclusion order when trimming).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum FeaturePriority {
    /// Must always be present.
    Required,
    /// Highly recommended; omit only under extreme size pressure.
    High,
    /// Optional; omit first when trimming.
    Low,
}

/// Descriptor for a single feature entry in the registry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureEntry {
    /// The feature.
    pub feature: Feature,
    /// Whether it is currently enabled.
    pub enabled: bool,
    /// Priority class.
    pub priority: FeaturePriority,
    /// Estimated binary size contribution in bytes.
    pub size_contribution_bytes: usize,
}

/// The feature registry for an on-device build.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureRegistry {
    entries: HashMap<String, FeatureEntry>,
}

impl FeatureRegistry {
    /// Create the minimal registry (only required features enabled).
    pub fn minimal() -> Self {
        let mut r = Self { entries: HashMap::new() };
        r.register(Feature::LocalOnlyInference, true, FeaturePriority::Required, 0);
        r.register(Feature::SqliteJournal, false, FeaturePriority::High, 512 * 1024);
        r.register(Feature::LanceDbMemory, false, FeaturePriority::High, 1024 * 1024);
        r.register(Feature::ColdStartPerfMonitor, false, FeaturePriority::Low, 64 * 1024);
        r.register(Feature::AndroidJni, false, FeaturePriority::High, 32 * 1024);
        r.register(Feature::IosCabi, false, FeaturePriority::High, 32 * 1024);
        r.register(Feature::StructuredLogging, false, FeaturePriority::Low, 128 * 1024);
        r.register(Feature::NeonSimd, false, FeaturePriority::Low, 16 * 1024);
        r
    }

    /// Create a full registry with everything enabled.
    pub fn full() -> Self {
        let mut r = Self::minimal();
        for entry in r.entries.values_mut() {
            entry.enabled = true;
        }
        r
    }

    fn register(
        &mut self,
        f: Feature,
        enabled: bool,
        priority: FeaturePriority,
        size_bytes: usize,
    ) {
        let key = f.to_string();
        self.entries.insert(
            key,
            FeatureEntry { feature: f, enabled, priority, size_contribution_bytes: size_bytes },
        );
    }

    /// Enable a feature.
    pub fn enable(&mut self, feature: &Feature) -> bool {
        if let Some(e) = self.entries.get_mut(&feature.to_string()) {
            e.enabled = true;
            true
        } else {
            false
        }
    }

    /// Disable a feature (required features cannot be disabled).
    pub fn disable(&mut self, feature: &Feature) -> Result<(), &'static str> {
        if let Some(e) = self.entries.get_mut(&feature.to_string()) {
            if e.priority == FeaturePriority::Required {
                return Err("cannot disable a required feature");
            }
            e.enabled = false;
            Ok(())
        } else {
            Err("feature not found")
        }
    }

    /// Return whether a feature is enabled.
    pub fn is_enabled(&self, feature: &Feature) -> bool {
        self.entries
            .get(&feature.to_string())
            .map(|e| e.enabled)
            .unwrap_or(false)
    }

    /// Estimated total binary contribution of enabled features in bytes.
    pub fn total_size_bytes(&self) -> usize {
        self.entries.values().filter(|e| e.enabled).map(|e| e.size_contribution_bytes).sum()
    }

    /// Trim low-priority features until the total size is within `budget_bytes`.
    pub fn trim_to_budget(&mut self, budget_bytes: usize) {
        while self.total_size_bytes() > budget_bytes {
            // Find the first enabled low-priority feature.
            let key = self
                .entries
                .iter()
                .filter(|(_, e)| e.enabled && e.priority == FeaturePriority::Low)
                .map(|(k, _)| k.clone())
                .next();
            match key {
                Some(k) => {
                    self.entries.get_mut(&k).unwrap().enabled = false;
                }
                None => break, // nothing left to trim
            }
        }
    }
}

#[cfg(test)]
mod unit {
    use super::*;

    #[test]
    fn minimal_only_required_enabled() {
        let r = FeatureRegistry::minimal();
        assert!(r.is_enabled(&Feature::LocalOnlyInference));
        assert!(!r.is_enabled(&Feature::SqliteJournal));
        assert!(!r.is_enabled(&Feature::LanceDbMemory));
    }

    #[test]
    fn enable_disable_roundtrip() {
        let mut r = FeatureRegistry::minimal();
        r.enable(&Feature::SqliteJournal);
        assert!(r.is_enabled(&Feature::SqliteJournal));
        r.disable(&Feature::SqliteJournal).unwrap();
        assert!(!r.is_enabled(&Feature::SqliteJournal));
    }

    #[test]
    fn cannot_disable_required() {
        let mut r = FeatureRegistry::minimal();
        let err = r.disable(&Feature::LocalOnlyInference).unwrap_err();
        assert_eq!(err, "cannot disable a required feature");
    }

    #[test]
    fn trim_to_budget_disables_low_priority() {
        let mut r = FeatureRegistry::full();
        // Trim to a very small budget.
        r.trim_to_budget(100);
        assert!(!r.is_enabled(&Feature::ColdStartPerfMonitor));
    }
}
