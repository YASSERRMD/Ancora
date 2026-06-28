//! Drift alerting.
//!
//! Aggregates signals from all drift detectors and produces structured alerts
//! that downstream systems can act on (page on-call, open an incident, etc.).

use std::fmt;

/// Severity level for a drift alert.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    Info,
    Warning,
    Critical,
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Severity::Info => write!(f, "INFO"),
            Severity::Warning => write!(f, "WARNING"),
            Severity::Critical => write!(f, "CRITICAL"),
        }
    }
}

/// A single drift alert.
#[derive(Debug, Clone)]
pub struct Alert {
    pub severity: Severity,
    /// Short machine-readable key, e.g. "input_drift", "cost_drift".
    pub kind: String,
    /// Human-readable description.
    pub message: String,
    /// Optional numeric metric associated with the alert.
    pub metric_value: Option<f64>,
}

impl Alert {
    pub fn new(severity: Severity, kind: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            severity,
            kind: kind.into(),
            message: message.into(),
            metric_value: None,
        }
    }

    pub fn with_metric(mut self, value: f64) -> Self {
        self.metric_value = Some(value);
        self
    }
}

impl fmt::Display for Alert {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}: {}", self.severity, self.kind, self.message)?;
        if let Some(v) = self.metric_value {
            write!(f, " (metric={v:.4})")?;
        }
        Ok(())
    }
}

/// Routing policy that decides which alerts to suppress or escalate.
#[derive(Debug, Clone)]
pub struct AlertPolicy {
    /// Minimum severity to emit (alerts below this are suppressed).
    pub min_severity: Severity,
}

impl Default for AlertPolicy {
    fn default() -> Self {
        Self { min_severity: Severity::Warning }
    }
}

impl AlertPolicy {
    pub fn new(min_severity: Severity) -> Self {
        Self { min_severity }
    }

    /// Filter `alerts` to only those meeting the policy's minimum severity.
    pub fn filter<'a>(&self, alerts: &'a [Alert]) -> Vec<&'a Alert> {
        alerts
            .iter()
            .filter(|a| a.severity >= self.min_severity)
            .collect()
    }
}

/// Aggregator that collects drift signals and emits alerts.
#[derive(Debug, Default)]
pub struct AlertAggregator {
    pending: Vec<Alert>,
}

impl AlertAggregator {
    pub fn new() -> Self {
        Self::default()
    }

    /// Record a drift alert.
    pub fn push(&mut self, alert: Alert) {
        self.pending.push(alert);
    }

    /// Push an alert only if the condition is true.
    pub fn push_if(&mut self, condition: bool, alert: Alert) {
        if condition {
            self.pending.push(alert);
        }
    }

    /// Return all pending alerts and clear the buffer.
    pub fn flush(&mut self) -> Vec<Alert> {
        std::mem::take(&mut self.pending)
    }

    /// Number of pending alerts.
    pub fn len(&self) -> usize {
        self.pending.len()
    }

    pub fn is_empty(&self) -> bool {
        self.pending.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alert_policy_filters_info() {
        let policy = AlertPolicy::new(Severity::Warning);
        let alerts = vec![
            Alert::new(Severity::Info, "x", "msg"),
            Alert::new(Severity::Warning, "y", "msg"),
            Alert::new(Severity::Critical, "z", "msg"),
        ];
        let filtered = policy.filter(&alerts);
        assert_eq!(filtered.len(), 2);
        assert!(filtered.iter().all(|a| a.severity >= Severity::Warning));
    }

    #[test]
    fn aggregator_flush() {
        let mut agg = AlertAggregator::new();
        agg.push(Alert::new(Severity::Critical, "cost_drift", "cost spiked"));
        assert_eq!(agg.len(), 1);
        let flushed = agg.flush();
        assert_eq!(flushed.len(), 1);
        assert!(agg.is_empty());
    }

    #[test]
    fn alert_display() {
        let a = Alert::new(Severity::Warning, "input_drift", "mean shifted").with_metric(4.5);
        let s = a.to_string();
        assert!(s.contains("WARNING"));
        assert!(s.contains("input_drift"));
        assert!(s.contains("4.5"));
    }
}
