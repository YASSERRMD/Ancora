//! Framework adapter documentation for Ancora.
//!
//! Describes the adapter trait that bridges third-party orchestration
//! frameworks (e.g., LangChain-style, custom DAG runners) into Ancora.

/// The orchestration framework being adapted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FrameworkKind {
    Custom(String),
    Temporal,
    Airflow,
    Prefect,
}

impl std::fmt::Display for FrameworkKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Custom(name) => write!(f, "custom:{name}"),
            Self::Temporal => write!(f, "temporal"),
            Self::Airflow => write!(f, "airflow"),
            Self::Prefect => write!(f, "prefect"),
        }
    }
}

/// Adapter configuration passed at registration time.
#[derive(Debug, Clone)]
pub struct AdapterConfig {
    pub framework: FrameworkKind,
    pub endpoint: Option<String>,
    pub namespace: Option<String>,
}

/// Trait that framework adapters must implement.
pub trait FrameworkAdapter: Send + Sync {
    fn framework(&self) -> &FrameworkKind;
    fn connect(&self, config: &AdapterConfig) -> Result<(), AdapterError>;
    fn is_connected(&self) -> bool;
}

/// Errors produced by a framework adapter.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdapterError {
    ConnectionFailed(String),
    InvalidConfig(String),
    NotSupported(String),
}

impl std::fmt::Display for AdapterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ConnectionFailed(msg) => write!(f, "connection failed: {msg}"),
            Self::InvalidConfig(msg) => write!(f, "invalid config: {msg}"),
            Self::NotSupported(msg) => write!(f, "not supported: {msg}"),
        }
    }
}

/// A stub adapter used in tests and documentation examples.
pub struct StubAdapter {
    framework: FrameworkKind,
    connected: std::sync::atomic::AtomicBool,
}

impl StubAdapter {
    pub fn new(framework: FrameworkKind) -> Self {
        Self {
            framework,
            connected: std::sync::atomic::AtomicBool::new(false),
        }
    }
}

impl FrameworkAdapter for StubAdapter {
    fn framework(&self) -> &FrameworkKind {
        &self.framework
    }

    fn connect(&self, _config: &AdapterConfig) -> Result<(), AdapterError> {
        self.connected
            .store(true, std::sync::atomic::Ordering::SeqCst);
        Ok(())
    }

    fn is_connected(&self) -> bool {
        self.connected.load(std::sync::atomic::Ordering::SeqCst)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stub_adapter_connects() {
        let adapter = StubAdapter::new(FrameworkKind::Temporal);
        let config = AdapterConfig {
            framework: FrameworkKind::Temporal,
            endpoint: None,
            namespace: None,
        };
        assert!(!adapter.is_connected());
        adapter.connect(&config).unwrap();
        assert!(adapter.is_connected());
    }

    #[test]
    fn framework_kind_display() {
        assert_eq!(FrameworkKind::Airflow.to_string(), "airflow");
        assert_eq!(
            FrameworkKind::Custom("my-fw".into()).to_string(),
            "custom:my-fw"
        );
    }
}
