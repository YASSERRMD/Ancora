//! Observability backend integrations: exporters and sink configurations.

/// Supported observability backends.
#[derive(Debug, Clone, PartialEq)]
pub enum Backend {
    /// OpenTelemetry Collector endpoint.
    OtelCollector { endpoint: String },
    /// Prometheus push gateway.
    Prometheus { push_gateway_url: String },
    /// Datadog agent.
    Datadog { site: String, api_key_env: String },
    /// Console output (useful for development).
    Console,
}

/// Configuration for an observability sink.
#[derive(Debug, Clone)]
pub struct SinkConfig {
    pub name: String,
    pub backend: Backend,
    pub enabled: bool,
    /// Additional headers to attach to export requests.
    pub headers: Vec<(String, String)>,
}

impl SinkConfig {
    pub fn new(name: impl Into<String>, backend: Backend) -> Self {
        Self {
            name: name.into(),
            backend,
            enabled: true,
            headers: Vec::new(),
        }
    }

    pub fn console(name: impl Into<String>) -> Self {
        Self::new(name, Backend::Console)
    }
}

/// Registry of all active observability sinks.
#[derive(Debug, Default)]
pub struct SinkRegistry {
    sinks: Vec<SinkConfig>,
}

impl SinkRegistry {
    pub fn register(&mut self, sink: SinkConfig) {
        self.sinks.push(sink);
    }

    pub fn enabled_sinks(&self) -> Vec<&SinkConfig> {
        self.sinks.iter().filter(|s| s.enabled).collect()
    }

    pub fn find_by_name(&self, name: &str) -> Option<&SinkConfig> {
        self.sinks.iter().find(|s| s.name == name)
    }
}
