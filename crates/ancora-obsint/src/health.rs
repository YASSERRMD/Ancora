/// Exporter health checks: polls each backend and reports readiness status.

use crate::selection::ExporterBackend;

#[derive(Debug, Clone, PartialEq)]
pub enum HealthStatus {
    Healthy,
    Degraded { reason: String },
    Unhealthy { reason: String },
}

impl HealthStatus {
    pub fn is_healthy(&self) -> bool {
        matches!(self, HealthStatus::Healthy)
    }

    pub fn is_degraded(&self) -> bool {
        matches!(self, HealthStatus::Degraded { .. })
    }

    pub fn is_unhealthy(&self) -> bool {
        matches!(self, HealthStatus::Unhealthy { .. })
    }

    pub fn label(&self) -> &'static str {
        match self {
            HealthStatus::Healthy => "healthy",
            HealthStatus::Degraded { .. } => "degraded",
            HealthStatus::Unhealthy { .. } => "unhealthy",
        }
    }
}

#[derive(Debug, Clone)]
pub struct ExporterHealthReport {
    pub backend: ExporterBackend,
    pub status: HealthStatus,
    pub latency_ms: Option<u64>,
    pub checked_at_ns: u64,
}

impl ExporterHealthReport {
    pub fn healthy(backend: ExporterBackend, latency_ms: u64) -> Self {
        ExporterHealthReport {
            backend,
            status: HealthStatus::Healthy,
            latency_ms: Some(latency_ms),
            checked_at_ns: 0,
        }
    }

    pub fn unhealthy(backend: ExporterBackend, reason: impl Into<String>) -> Self {
        ExporterHealthReport {
            backend,
            status: HealthStatus::Unhealthy {
                reason: reason.into(),
            },
            latency_ms: None,
            checked_at_ns: 0,
        }
    }

    pub fn degraded(backend: ExporterBackend, reason: impl Into<String>, latency_ms: u64) -> Self {
        ExporterHealthReport {
            backend,
            status: HealthStatus::Degraded {
                reason: reason.into(),
            },
            latency_ms: Some(latency_ms),
            checked_at_ns: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct HealthChecker {
    pub backends: Vec<ExporterBackend>,
    /// Simulated latency thresholds (ms): above degraded_threshold_ms -> degraded.
    pub degraded_threshold_ms: u64,
    pub unhealthy_threshold_ms: u64,
}

impl HealthChecker {
    pub fn new(backends: Vec<ExporterBackend>) -> Self {
        HealthChecker {
            backends,
            degraded_threshold_ms: 500,
            unhealthy_threshold_ms: 2000,
        }
    }

    pub fn with_thresholds(mut self, degraded_ms: u64, unhealthy_ms: u64) -> Self {
        self.degraded_threshold_ms = degraded_ms;
        self.unhealthy_threshold_ms = unhealthy_ms;
        self
    }

    /// Evaluates a latency measurement and produces a health report for the given backend.
    pub fn evaluate(&self, backend: ExporterBackend, latency_ms: u64) -> ExporterHealthReport {
        if latency_ms >= self.unhealthy_threshold_ms {
            ExporterHealthReport::unhealthy(
                backend,
                format!("latency {}ms exceeds unhealthy threshold {}ms", latency_ms, self.unhealthy_threshold_ms),
            )
        } else if latency_ms >= self.degraded_threshold_ms {
            ExporterHealthReport::degraded(
                backend,
                format!("latency {}ms exceeds degraded threshold {}ms", latency_ms, self.degraded_threshold_ms),
                latency_ms,
            )
        } else {
            ExporterHealthReport::healthy(backend, latency_ms)
        }
    }
}

/// Summarizes a collection of health reports into an overall system health status.
pub fn summarize_health(reports: &[ExporterHealthReport]) -> HealthStatus {
    if reports.is_empty() {
        return HealthStatus::Unhealthy {
            reason: "no exporters configured".to_string(),
        };
    }
    let unhealthy_count = reports.iter().filter(|r| r.status.is_unhealthy()).count();
    let degraded_count = reports.iter().filter(|r| r.status.is_degraded()).count();

    if unhealthy_count == reports.len() {
        HealthStatus::Unhealthy {
            reason: "all exporters are unhealthy".to_string(),
        }
    } else if unhealthy_count > 0 || degraded_count > 0 {
        HealthStatus::Degraded {
            reason: format!(
                "{} unhealthy, {} degraded out of {} exporters",
                unhealthy_count,
                degraded_count,
                reports.len()
            ),
        }
    } else {
        HealthStatus::Healthy
    }
}
