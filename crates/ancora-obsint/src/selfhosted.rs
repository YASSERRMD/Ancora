/// Self-hosted observability mode: enforces data residency by blocking external exports.
/// When self-hosted mode is active, only configured internal endpoints are permitted.

#[derive(Debug, Clone, PartialEq)]
pub enum ResidencyPolicy {
    /// Data may be sent to any endpoint.
    Unrestricted,
    /// Data must stay within the specified network prefixes.
    SelfHostedOnly { allowed_prefixes: Vec<String> },
}

impl ResidencyPolicy {
    pub fn self_hosted(allowed_prefixes: Vec<String>) -> Self {
        ResidencyPolicy::SelfHostedOnly { allowed_prefixes }
    }

    pub fn is_self_hosted(&self) -> bool {
        matches!(self, ResidencyPolicy::SelfHostedOnly { .. })
    }

    /// Returns Ok if the endpoint is allowed under this policy.
    pub fn check_endpoint(&self, endpoint: &str) -> Result<(), ResidencyError> {
        match self {
            ResidencyPolicy::Unrestricted => Ok(()),
            ResidencyPolicy::SelfHostedOnly { allowed_prefixes } => {
                for prefix in allowed_prefixes {
                    if endpoint.starts_with(prefix.as_str()) {
                        return Ok(());
                    }
                }
                Err(ResidencyError::ExternalEndpointBlocked {
                    endpoint: endpoint.to_string(),
                })
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ResidencyError {
    ExternalEndpointBlocked { endpoint: String },
    PolicyViolation { reason: String },
}

impl std::fmt::Display for ResidencyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResidencyError::ExternalEndpointBlocked { endpoint } => {
                write!(
                    f,
                    "residency policy blocks external endpoint: {}",
                    endpoint
                )
            }
            ResidencyError::PolicyViolation { reason } => {
                write!(f, "residency policy violation: {}", reason)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct SelfHostedConfig {
    pub policy: ResidencyPolicy,
    pub internal_tempo_endpoint: Option<String>,
    pub internal_loki_endpoint: Option<String>,
    pub internal_prometheus_endpoint: Option<String>,
    pub internal_otlp_endpoint: Option<String>,
}

impl SelfHostedConfig {
    pub fn new(policy: ResidencyPolicy) -> Self {
        SelfHostedConfig {
            policy,
            internal_tempo_endpoint: None,
            internal_loki_endpoint: None,
            internal_prometheus_endpoint: None,
            internal_otlp_endpoint: None,
        }
    }

    pub fn with_tempo(mut self, endpoint: impl Into<String>) -> Self {
        self.internal_tempo_endpoint = Some(endpoint.into());
        self
    }

    pub fn with_loki(mut self, endpoint: impl Into<String>) -> Self {
        self.internal_loki_endpoint = Some(endpoint.into());
        self
    }

    pub fn with_prometheus(mut self, endpoint: impl Into<String>) -> Self {
        self.internal_prometheus_endpoint = Some(endpoint.into());
        self
    }

    pub fn with_otlp(mut self, endpoint: impl Into<String>) -> Self {
        self.internal_otlp_endpoint = Some(endpoint.into());
        self
    }

    /// Validates that all configured endpoints comply with the residency policy.
    pub fn validate(&self) -> Result<(), ResidencyError> {
        let endpoints: Vec<&str> = [
            self.internal_tempo_endpoint.as_deref(),
            self.internal_loki_endpoint.as_deref(),
            self.internal_prometheus_endpoint.as_deref(),
            self.internal_otlp_endpoint.as_deref(),
        ]
        .iter()
        .filter_map(|e| *e)
        .collect();

        for endpoint in endpoints {
            self.policy.check_endpoint(endpoint)?;
        }
        Ok(())
    }
}

/// Checks whether a given export destination is permitted under the active residency policy.
pub fn is_export_permitted(policy: &ResidencyPolicy, destination: &str) -> bool {
    policy.check_endpoint(destination).is_ok()
}
