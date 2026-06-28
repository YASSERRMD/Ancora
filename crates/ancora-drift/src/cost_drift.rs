//! Cost drift detection.
//!
//! Monitors per-request cost trends and flags when cost has drifted beyond
//! acceptable bounds relative to the reference window.

use crate::reference::{ReferenceDistribution, Stats};

/// Result of a cost drift check.
#[derive(Debug, Clone, PartialEq)]
pub struct CostDriftResult {
    pub drifted: bool,
    /// Z-score of the mean cost shift.
    pub mean_z_score: f64,
    /// Absolute mean shift in micro-dollars.
    pub mean_shift_micros: f64,
    pub threshold_z: f64,
    /// Percentage increase relative to reference mean (positive = more expensive).
    pub pct_change: f64,
}

/// Detector for changes in per-request cost.
#[derive(Debug, Clone)]
pub struct CostDriftDetector {
    pub threshold_z: f64,
}

impl Default for CostDriftDetector {
    fn default() -> Self {
        Self { threshold_z: 3.0 }
    }
}

impl CostDriftDetector {
    pub fn new(threshold_z: f64) -> Self {
        Self { threshold_z }
    }

    /// Compare `current` cost stats against the `reference` distribution.
    pub fn check(
        &self,
        reference: &ReferenceDistribution,
        current: &Stats,
    ) -> Result<CostDriftResult, String> {
        let ref_std = reference.cost_micros.std_dev();
        if ref_std == 0.0 {
            return Err("Reference cost std_dev is zero".into());
        }
        let ref_mean = reference.cost_micros.mean;
        let mean_shift_micros = current.mean - ref_mean;
        let mean_z_score = mean_shift_micros / ref_std;
        let pct_change = if ref_mean == 0.0 {
            0.0
        } else {
            (mean_shift_micros / ref_mean) * 100.0
        };
        let drifted = mean_z_score.abs() > self.threshold_z;
        Ok(CostDriftResult {
            drifted,
            mean_z_score,
            mean_shift_micros,
            threshold_z: self.threshold_z,
            pct_change,
        })
    }
}

/// Accumulates cost observations and computes running statistics.
#[derive(Debug, Default)]
pub struct CostAccumulator {
    values: Vec<f64>,
}

impl CostAccumulator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, cost_micros: u64) {
        self.values.push(cost_micros as f64);
    }

    pub fn stats(&self) -> Option<Stats> {
        Stats::from_slice(&self.values)
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reference::ReferenceBuilder;

    fn make_ref() -> ReferenceDistribution {
        let mut b = ReferenceBuilder::new();
        b.add("q", "a", 100, 50, &[], "openai");
        b.add("q", "a", 200, 50, &[], "openai");
        b.build().unwrap()
    }

    #[test]
    fn stable_cost_no_drift() {
        let reference = make_ref();
        // Current mean matches reference mean
        let current = Stats::from_slice(&[150.0]).unwrap();
        let detector = CostDriftDetector::new(3.0);
        let result = detector.check(&reference, &current).unwrap();
        assert!(!result.drifted);
    }

    #[test]
    fn large_cost_increase_detected() {
        let reference = make_ref();
        // Hugely more expensive
        let current = Stats::from_slice(&[10_000.0]).unwrap();
        let detector = CostDriftDetector::new(3.0);
        let result = detector.check(&reference, &current).unwrap();
        assert!(result.drifted);
        assert!(result.pct_change > 0.0);
    }

    #[test]
    fn accumulator_stats() {
        let mut acc = CostAccumulator::new();
        acc.add(100);
        acc.add(200);
        let s = acc.stats().unwrap();
        assert_eq!(s.mean, 150.0);
    }
}
