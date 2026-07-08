/// Deployment topology for self-hosted observability.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Topology {
    SingleNode,
    HighAvailability,
    FederatedMultiRegion,
}

impl std::fmt::Display for Topology {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Topology::SingleNode => "single-node",
            Topology::HighAvailability => "high-availability",
            Topology::FederatedMultiRegion => "federated-multi-region",
        };
        write!(f, "{}", s)
    }
}

/// Summary of a self-hosted observability deployment.
#[derive(Debug, Clone)]
pub struct SelfHostedSummary {
    pub topology: Topology,
    pub backend: String,
    pub metrics_retention_days: u32,
    pub traces_retention_days: u32,
    pub logs_retention_days: u32,
    pub tls_enabled: bool,
    pub auth_provider: Option<String>,
}

impl SelfHostedSummary {
    pub fn new(topology: Topology, backend: impl Into<String>) -> Self {
        Self {
            topology,
            backend: backend.into(),
            metrics_retention_days: 30,
            traces_retention_days: 7,
            logs_retention_days: 14,
            tls_enabled: true,
            auth_provider: None,
        }
    }

    pub fn with_auth(mut self, provider: impl Into<String>) -> Self {
        self.auth_provider = Some(provider.into());
        self
    }

    pub fn with_retention(mut self, metrics: u32, traces: u32, logs: u32) -> Self {
        self.metrics_retention_days = metrics;
        self.traces_retention_days = traces;
        self.logs_retention_days = logs;
        self
    }

    pub fn is_production_grade(&self) -> bool {
        self.tls_enabled && self.auth_provider.is_some() && self.topology != Topology::SingleNode
    }

    pub fn render(&self) -> String {
        format!(
            "Self-Hosted Observability\n\
             Topology: {}\n\
             Backend: {}\n\
             Metrics retention: {} days\n\
             Traces retention: {} days\n\
             Logs retention: {} days\n\
             TLS: {}\n\
             Auth: {}\n\
             Production grade: {}",
            self.topology,
            self.backend,
            self.metrics_retention_days,
            self.traces_retention_days,
            self.logs_retention_days,
            self.tls_enabled,
            self.auth_provider.as_deref().unwrap_or("none"),
            self.is_production_grade(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_node_not_production_grade() {
        let s = SelfHostedSummary::new(Topology::SingleNode, "prometheus");
        assert!(!s.is_production_grade());
    }

    #[test]
    fn ha_with_auth_is_production_grade() {
        let s = SelfHostedSummary::new(Topology::HighAvailability, "victoria-metrics")
            .with_auth("oidc");
        assert!(s.is_production_grade());
    }

    #[test]
    fn render_includes_topology() {
        let s = SelfHostedSummary::new(Topology::FederatedMultiRegion, "thanos");
        assert!(s.render().contains("federated-multi-region"));
    }
}
