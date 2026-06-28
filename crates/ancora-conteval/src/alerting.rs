/// Quality alerting for continuous evaluation.
///
/// Generates alerts when quality metrics cross configured thresholds
/// or when negative trends are detected.

use std::time::SystemTime;

/// Severity of an alert.
#[derive(Debug, Clone, PartialEq)]
pub enum AlertSeverity {
    Info,
    Warning,
    Critical,
}

impl AlertSeverity {
    pub fn as_str(&self) -> &str {
        match self {
            AlertSeverity::Info => "info",
            AlertSeverity::Warning => "warning",
            AlertSeverity::Critical => "critical",
        }
    }
}

/// The reason an alert was raised.
#[derive(Debug, Clone, PartialEq)]
pub enum AlertReason {
    /// Score fell below the configured floor.
    ScoreBelowThreshold { score: f64, threshold: f64 },
    /// A degrading trend was detected.
    DegradingTrend { slope: f64 },
    /// Score dropped sharply between consecutive observations.
    SuddenDrop { previous: f64, current: f64, drop: f64 },
}

/// A quality alert.
#[derive(Debug, Clone)]
pub struct QualityAlert {
    pub id: String,
    pub model: String,
    pub provider: String,
    pub severity: AlertSeverity,
    pub reason: AlertReason,
    pub raised_at: SystemTime,
}

impl QualityAlert {
    pub fn new(
        id: impl Into<String>,
        model: impl Into<String>,
        provider: impl Into<String>,
        severity: AlertSeverity,
        reason: AlertReason,
        raised_at: SystemTime,
    ) -> Self {
        QualityAlert {
            id: id.into(),
            model: model.into(),
            provider: provider.into(),
            severity,
            reason,
            raised_at,
        }
    }
}

/// Configuration thresholds for the alerting engine.
#[derive(Debug, Clone)]
pub struct AlertConfig {
    /// Scores at or below this value trigger a Warning.
    pub warning_threshold: f64,
    /// Scores at or below this value trigger a Critical alert.
    pub critical_threshold: f64,
    /// A drop larger than this fraction of the previous score triggers an alert.
    pub sudden_drop_fraction: f64,
    /// Slope below this negative value triggers a degrading-trend alert.
    pub degrading_slope: f64,
}

impl AlertConfig {
    pub fn new(
        warning_threshold: f64,
        critical_threshold: f64,
        sudden_drop_fraction: f64,
        degrading_slope: f64,
    ) -> Self {
        AlertConfig {
            warning_threshold,
            critical_threshold,
            sudden_drop_fraction,
            degrading_slope,
        }
    }
}

/// Engine that evaluates metrics and emits alerts.
#[derive(Debug, Default)]
pub struct AlertEngine {
    config: Option<AlertConfig>,
    alerts: Vec<QualityAlert>,
    alert_counter: u64,
}

impl AlertEngine {
    pub fn new(config: AlertConfig) -> Self {
        AlertEngine {
            config: Some(config),
            alerts: Vec::new(),
            alert_counter: 0,
        }
    }

    fn next_id(&mut self) -> String {
        self.alert_counter += 1;
        format!("alert-{}", self.alert_counter)
    }

    /// Evaluate a score and raise threshold-based alerts if needed.
    pub fn evaluate_score(
        &mut self,
        model: &str,
        provider: &str,
        score: f64,
        previous: Option<f64>,
        now: SystemTime,
    ) {
        let config = match &self.config {
            Some(c) => c.clone(),
            None => return,
        };

        // Threshold alerts.
        if score <= config.critical_threshold {
            let id = self.next_id();
            self.alerts.push(QualityAlert::new(
                id,
                model,
                provider,
                AlertSeverity::Critical,
                AlertReason::ScoreBelowThreshold {
                    score,
                    threshold: config.critical_threshold,
                },
                now,
            ));
        } else if score <= config.warning_threshold {
            let id = self.next_id();
            self.alerts.push(QualityAlert::new(
                id,
                model,
                provider,
                AlertSeverity::Warning,
                AlertReason::ScoreBelowThreshold {
                    score,
                    threshold: config.warning_threshold,
                },
                now,
            ));
        }

        // Sudden drop alert.
        if let Some(prev) = previous {
            if prev > 0.0 {
                let drop = (prev - score) / prev;
                if drop >= config.sudden_drop_fraction {
                    let id = self.next_id();
                    self.alerts.push(QualityAlert::new(
                        id,
                        model,
                        provider,
                        AlertSeverity::Warning,
                        AlertReason::SuddenDrop {
                            previous: prev,
                            current: score,
                            drop,
                        },
                        now,
                    ));
                }
            }
        }
    }

    /// Raise an alert for a detected degrading trend.
    pub fn raise_trend_alert(
        &mut self,
        model: &str,
        provider: &str,
        slope: f64,
        now: SystemTime,
    ) {
        let config = match &self.config {
            Some(c) => c.clone(),
            None => return,
        };
        if slope < -config.degrading_slope.abs() {
            let id = self.next_id();
            self.alerts.push(QualityAlert::new(
                id,
                model,
                provider,
                AlertSeverity::Warning,
                AlertReason::DegradingTrend { slope },
                now,
            ));
        }
    }

    /// All alerts raised so far.
    pub fn alerts(&self) -> &[QualityAlert] {
        &self.alerts
    }

    /// Number of alerts raised.
    pub fn alert_count(&self) -> usize {
        self.alerts.len()
    }

    /// Drain all alerts, returning them. Used to forward to a sink.
    pub fn drain_alerts(&mut self) -> Vec<QualityAlert> {
        std::mem::take(&mut self.alerts)
    }
}
