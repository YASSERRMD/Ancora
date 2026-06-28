//! Observability and evaluation overview for the Ancora framework.

/// Describes the top-level observability posture for an agent deployment.
#[derive(Debug, Clone, PartialEq)]
pub struct ObservabilityOverview {
    /// Human-readable label for this deployment.
    pub deployment_name: String,
    /// Whether distributed tracing is enabled.
    pub tracing_enabled: bool,
    /// Whether metrics collection is active.
    pub metrics_enabled: bool,
    /// Whether evaluation pipelines are attached.
    pub eval_enabled: bool,
}

impl ObservabilityOverview {
    /// Create a new overview with all features enabled by default.
    pub fn new(deployment_name: impl Into<String>) -> Self {
        Self {
            deployment_name: deployment_name.into(),
            tracing_enabled: true,
            metrics_enabled: true,
            eval_enabled: true,
        }
    }

    /// Returns true if the deployment is fully observable.
    pub fn is_fully_observable(&self) -> bool {
        self.tracing_enabled && self.metrics_enabled && self.eval_enabled
    }
}

/// Summarises which observability pillars are active.
#[derive(Debug, Clone)]
pub struct ObservabilityPillars {
    pub tracing: bool,
    pub metrics: bool,
    pub logging: bool,
    pub evaluation: bool,
}

impl Default for ObservabilityPillars {
    fn default() -> Self {
        Self {
            tracing: true,
            metrics: true,
            logging: true,
            evaluation: false,
        }
    }
}

impl ObservabilityPillars {
    /// Count how many pillars are active.
    pub fn active_count(&self) -> usize {
        [self.tracing, self.metrics, self.logging, self.evaluation]
            .iter()
            .filter(|&&b| b)
            .count()
    }
}
