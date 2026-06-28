//! Drift detection parity - validates that drift signals are consistent across SDKs.

use std::collections::VecDeque;

/// A windowed metric buffer for drift detection.
#[derive(Debug, Clone)]
pub struct MetricWindow {
    pub name: String,
    values: VecDeque<f64>,
    capacity: usize,
}

impl MetricWindow {
    pub fn new(name: impl Into<String>, capacity: usize) -> Self {
        Self {
            name: name.into(),
            values: VecDeque::with_capacity(capacity),
            capacity,
        }
    }

    pub fn push(&mut self, value: f64) {
        if self.values.len() == self.capacity {
            self.values.pop_front();
        }
        self.values.push_back(value);
    }

    pub fn mean(&self) -> Option<f64> {
        if self.values.is_empty() {
            None
        } else {
            Some(self.values.iter().sum::<f64>() / self.values.len() as f64)
        }
    }

    pub fn std_dev(&self) -> Option<f64> {
        let mean = self.mean()?;
        let variance = self.values.iter().map(|v| (v - mean).powi(2)).sum::<f64>()
            / self.values.len() as f64;
        Some(variance.sqrt())
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

/// Drift severity level.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DriftSeverity {
    None,
    Warning,
    Critical,
}

/// Result of a drift check.
#[derive(Debug, Clone)]
pub struct DriftResult {
    pub metric_name: String,
    pub language: String,
    pub severity: DriftSeverity,
    pub z_score: f64,
    pub current_value: f64,
    pub baseline_mean: f64,
}

/// Detect drift by comparing current value against baseline statistics.
pub fn detect_drift(
    metric_name: impl Into<String>,
    language: impl Into<String>,
    current: f64,
    baseline_mean: f64,
    baseline_std: f64,
) -> DriftResult {
    let z_score = if baseline_std == 0.0 {
        if (current - baseline_mean).abs() < 1e-10 {
            0.0
        } else {
            f64::INFINITY
        }
    } else {
        (current - baseline_mean).abs() / baseline_std
    };

    let severity = if z_score < 2.0 {
        DriftSeverity::None
    } else if z_score < 3.0 {
        DriftSeverity::Warning
    } else {
        DriftSeverity::Critical
    };

    DriftResult {
        metric_name: metric_name.into(),
        language: language.into(),
        severity,
        z_score,
        current_value: current,
        baseline_mean,
    }
}

/// Drift report across multiple languages.
#[derive(Debug, Clone)]
pub struct DriftReport {
    pub results: Vec<DriftResult>,
}

impl DriftReport {
    pub fn new(results: Vec<DriftResult>) -> Self {
        Self { results }
    }

    pub fn has_critical(&self) -> bool {
        self.results.iter().any(|r| r.severity == DriftSeverity::Critical)
    }

    pub fn critical_count(&self) -> usize {
        self.results.iter().filter(|r| r.severity == DriftSeverity::Critical).count()
    }
}

/// Check drift parity across language reports.
pub fn check_drift_parity(reports: &[(&str, DriftReport)]) -> Vec<String> {
    let mut issues = Vec::new();

    if let Some((first_lang, first_report)) = reports.first() {
        for (other_lang, other_report) in reports.iter().skip(1) {
            if first_report.has_critical() != other_report.has_critical() {
                issues.push(format!(
                    "drift critical mismatch: {:?} has_critical={} vs {:?} has_critical={}",
                    first_lang,
                    first_report.has_critical(),
                    other_lang,
                    other_report.has_critical()
                ));
            }
            if first_report.critical_count() != other_report.critical_count() {
                issues.push(format!(
                    "drift critical count: {:?}={} vs {:?}={}",
                    first_lang,
                    first_report.critical_count(),
                    other_lang,
                    other_report.critical_count()
                ));
            }
        }
    }

    issues
}

/// Build a stable drift report (no drift) for parity testing.
pub fn stable_drift_report(language: impl Into<String>) -> DriftReport {
    let lang = language.into();
    let results = vec![
        detect_drift("mean_score", &lang, 0.90, 0.88, 0.05),
        detect_drift("p50_latency_ms", &lang, 195.0, 200.0, 30.0),
        detect_drift("cost_per_call", &lang, 0.0031, 0.003, 0.001),
    ];
    DriftReport::new(results)
}
