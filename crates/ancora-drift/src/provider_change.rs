//! Provider behavior change detection.
//!
//! Detects when the mix of providers being used changes, or when a particular
//! provider's latency / error characteristics shift significantly.

use crate::reference::ReferenceDistribution;
use std::collections::HashMap;

/// A single provider's current snapshot metrics.
#[derive(Debug, Clone)]
pub struct ProviderSnapshot {
    pub name: String,
    /// Fraction of requests served by this provider in the current window.
    pub frequency: f64,
    /// Mean latency in milliseconds.
    pub mean_latency_ms: f64,
    /// Error rate (0.0 - 1.0).
    pub error_rate: f64,
}

/// Result for a single provider's change check.
#[derive(Debug, Clone)]
pub struct ProviderChangeEntry {
    pub provider: String,
    pub ref_frequency: f64,
    pub cur_frequency: f64,
    /// True if the provider's share shifted more than the threshold.
    pub frequency_changed: bool,
    pub latency_z_score: Option<f64>,
    pub latency_changed: bool,
}

/// Aggregate result.
#[derive(Debug, Clone)]
pub struct ProviderChangeResult {
    pub entries: Vec<ProviderChangeEntry>,
    pub any_changed: bool,
}

/// Detector for provider-level behavioral changes.
#[derive(Debug, Clone)]
pub struct ProviderChangeDetector {
    /// Maximum tolerated absolute frequency shift (0.0 - 1.0).
    pub freq_threshold: f64,
    /// Z-score threshold for latency shift.
    pub latency_threshold_z: f64,
}

impl Default for ProviderChangeDetector {
    fn default() -> Self {
        Self {
            freq_threshold: 0.10,
            latency_threshold_z: 3.0,
        }
    }
}

impl ProviderChangeDetector {
    pub fn new(freq_threshold: f64, latency_threshold_z: f64) -> Self {
        Self {
            freq_threshold,
            latency_threshold_z,
        }
    }

    /// Compare `snapshots` against the `reference` distribution.
    pub fn check(
        &self,
        reference: &ReferenceDistribution,
        snapshots: &[ProviderSnapshot],
    ) -> ProviderChangeResult {
        let ref_latency_mean = reference.latency_ms.mean;
        let ref_latency_std = reference.latency_ms.std_dev();

        let snapshot_map: HashMap<&str, &ProviderSnapshot> =
            snapshots.iter().map(|s| (s.name.as_str(), s)).collect();

        let all_providers: std::collections::HashSet<String> = reference
            .provider_frequencies
            .keys()
            .cloned()
            .chain(snapshots.iter().map(|s| s.name.clone()))
            .collect();

        let mut entries: Vec<ProviderChangeEntry> = all_providers
            .into_iter()
            .map(|prov| {
                let ref_freq = reference
                    .provider_frequencies
                    .get(prov.as_str())
                    .cloned()
                    .unwrap_or(0.0);
                let (cur_freq, latency_z, latency_changed) =
                    if let Some(snap) = snapshot_map.get(prov.as_str()) {
                        let z = if ref_latency_std > 0.0 {
                            Some((snap.mean_latency_ms - ref_latency_mean) / ref_latency_std)
                        } else {
                            None
                        };
                        let changed = z
                            .map(|z| z.abs() > self.latency_threshold_z)
                            .unwrap_or(false);
                        (snap.frequency, z, changed)
                    } else {
                        (0.0, None, false)
                    };
                let freq_diff = (cur_freq - ref_freq).abs();
                ProviderChangeEntry {
                    provider: prov,
                    ref_frequency: ref_freq,
                    cur_frequency: cur_freq,
                    frequency_changed: freq_diff > self.freq_threshold,
                    latency_z_score: latency_z,
                    latency_changed,
                }
            })
            .collect();

        entries.sort_by(|a, b| a.provider.cmp(&b.provider));
        let any_changed = entries
            .iter()
            .any(|e| e.frequency_changed || e.latency_changed);
        ProviderChangeResult {
            entries,
            any_changed,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reference::ReferenceBuilder;

    fn make_ref() -> ReferenceDistribution {
        let mut b = ReferenceBuilder::new();
        for _ in 0..100 {
            b.add("q", "a", 100, 50, &[], "openai");
        }
        b.build().unwrap()
    }

    #[test]
    fn stable_provider_no_change() {
        let reference = make_ref();
        let snapshots = vec![ProviderSnapshot {
            name: "openai".into(),
            frequency: 1.0,
            mean_latency_ms: 52.0, // slightly different but within threshold
            error_rate: 0.0,
        }];
        let detector = ProviderChangeDetector::default();
        let result = detector.check(&reference, &snapshots);
        assert!(!result.any_changed);
    }

    #[test]
    fn new_provider_appears_is_change() {
        let reference = make_ref();
        let snapshots = vec![ProviderSnapshot {
            name: "anthropic".into(),
            frequency: 0.5,
            mean_latency_ms: 60.0,
            error_rate: 0.0,
        }];
        let detector = ProviderChangeDetector::default();
        let result = detector.check(&reference, &snapshots);
        // "anthropic" was not in reference so its cur_freq will show up as a new provider
        // "openai" disappeared which is a frequency change
        assert!(result.any_changed);
    }
}
