/// Exporter selection: determines which backend(s) to use based on configuration.
use crate::selfhosted::ResidencyPolicy;

#[derive(Debug, Clone, PartialEq)]
pub enum ExporterBackend {
    Otlp,
    Langfuse,
    Phoenix,
    GrafanaTempo,
    GrafanaLoki,
    Datadog,
    Prometheus,
    SelfHosted,
}

impl ExporterBackend {
    pub fn as_str(&self) -> &'static str {
        match self {
            ExporterBackend::Otlp => "otlp",
            ExporterBackend::Langfuse => "langfuse",
            ExporterBackend::Phoenix => "phoenix",
            ExporterBackend::GrafanaTempo => "grafana_tempo",
            ExporterBackend::GrafanaLoki => "grafana_loki",
            ExporterBackend::Datadog => "datadog",
            ExporterBackend::Prometheus => "prometheus",
            ExporterBackend::SelfHosted => "self_hosted",
        }
    }

    /// Returns true if this backend sends data to an external (cloud) service.
    pub fn is_external(&self) -> bool {
        matches!(
            self,
            ExporterBackend::Langfuse | ExporterBackend::Phoenix | ExporterBackend::Datadog
        )
    }
}

#[derive(Debug, Clone)]
pub struct ExporterSelection {
    pub backends: Vec<ExporterBackend>,
    pub residency_policy: ResidencyPolicy,
}

impl ExporterSelection {
    pub fn new(policy: ResidencyPolicy) -> Self {
        ExporterSelection {
            backends: Vec::new(),
            residency_policy: policy,
        }
    }

    /// Adds a backend if it does not violate the residency policy.
    pub fn add_backend(&mut self, backend: ExporterBackend) -> Result<(), SelectionError> {
        if self.residency_policy.is_self_hosted() && backend.is_external() {
            return Err(SelectionError::ExternalBackendForbidden {
                backend: backend.as_str().to_string(),
            });
        }
        if !self.backends.contains(&backend) {
            self.backends.push(backend);
        }
        Ok(())
    }

    pub fn has_backend(&self, backend: &ExporterBackend) -> bool {
        self.backends.contains(backend)
    }

    pub fn active_count(&self) -> usize {
        self.backends.len()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SelectionError {
    ExternalBackendForbidden { backend: String },
    InvalidConfig { reason: String },
}

impl std::fmt::Display for SelectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SelectionError::ExternalBackendForbidden { backend } => {
                write!(
                    f,
                    "backend '{}' is external and forbidden under self-hosted-only policy",
                    backend
                )
            }
            SelectionError::InvalidConfig { reason } => {
                write!(f, "invalid exporter config: {}", reason)
            }
        }
    }
}

/// Parses a comma-separated list of backend names into ExporterBackend values.
pub fn parse_backends(s: &str) -> Result<Vec<ExporterBackend>, SelectionError> {
    let mut result = Vec::new();
    for part in s.split(',') {
        let part = part.trim();
        let backend = match part {
            "otlp" => ExporterBackend::Otlp,
            "langfuse" => ExporterBackend::Langfuse,
            "phoenix" => ExporterBackend::Phoenix,
            "grafana_tempo" => ExporterBackend::GrafanaTempo,
            "grafana_loki" => ExporterBackend::GrafanaLoki,
            "datadog" => ExporterBackend::Datadog,
            "prometheus" => ExporterBackend::Prometheus,
            "self_hosted" => ExporterBackend::SelfHosted,
            other => {
                return Err(SelectionError::InvalidConfig {
                    reason: format!("unknown backend '{}'", other),
                })
            }
        };
        result.push(backend);
    }
    Ok(result)
}
