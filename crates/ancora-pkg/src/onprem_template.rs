//! On-premises appliance deployment template.
//!
//! Generates configuration for deploying Ancora as an on-premises appliance
//! with network isolation and local secret management.

use std::collections::HashMap;

/// Network isolation level for on-prem deployments.
#[derive(Debug, Clone, PartialEq)]
pub enum NetworkIsolation {
    /// Full network, outbound allowed.
    Connected,
    /// Restricted outbound, allow-list only.
    Restricted,
    /// No external network access.
    AirGapped,
}

impl NetworkIsolation {
    pub fn as_str(&self) -> &'static str {
        match self {
            NetworkIsolation::Connected => "connected",
            NetworkIsolation::Restricted => "restricted",
            NetworkIsolation::AirGapped => "air-gapped",
        }
    }
}

/// Secret storage backend for on-prem.
#[derive(Debug, Clone, PartialEq)]
pub enum SecretBackend {
    LocalKeyring,
    Vault,
    Hsm,
}

impl SecretBackend {
    pub fn as_str(&self) -> &'static str {
        match self {
            SecretBackend::LocalKeyring => "local-keyring",
            SecretBackend::Vault => "vault",
            SecretBackend::Hsm => "hsm",
        }
    }
}

/// On-premises deployment configuration.
#[derive(Debug, Clone)]
pub struct OnPremConfig {
    pub product_name: String,
    pub hostname: String,
    pub network_isolation: NetworkIsolation,
    pub secret_backend: SecretBackend,
    pub node_count: u32,
    pub data_path: String,
    pub extra_labels: HashMap<String, String>,
}

impl OnPremConfig {
    pub fn new(
        product_name: impl Into<String>,
        hostname: impl Into<String>,
        node_count: u32,
    ) -> Self {
        Self {
            product_name: product_name.into(),
            hostname: hostname.into(),
            network_isolation: NetworkIsolation::Restricted,
            secret_backend: SecretBackend::Vault,
            node_count,
            data_path: "/var/lib/ancora".to_string(),
            extra_labels: HashMap::new(),
        }
    }

    pub fn with_isolation(mut self, isolation: NetworkIsolation) -> Self {
        self.network_isolation = isolation;
        self
    }

    pub fn with_secret_backend(mut self, backend: SecretBackend) -> Self {
        self.secret_backend = backend;
        self
    }

    pub fn with_data_path(mut self, path: impl Into<String>) -> Self {
        self.data_path = path.into();
        self
    }
}

/// Rendered on-prem appliance template.
#[derive(Debug, Clone)]
pub struct OnPremTemplate {
    pub config: OnPremConfig,
    pub rendered: String,
}

impl OnPremTemplate {
    pub fn render(config: OnPremConfig) -> Result<Self, OnPremError> {
        if config.product_name.is_empty() {
            return Err(OnPremError::InvalidConfig("product_name is required".to_string()));
        }
        if config.hostname.is_empty() {
            return Err(OnPremError::InvalidConfig("hostname is required".to_string()));
        }
        if config.node_count == 0 {
            return Err(OnPremError::InvalidConfig("node_count must be >= 1".to_string()));
        }

        let rendered = format!(
            "# ancora-pkg on-prem appliance template\n\
             product: {product}\n\
             hostname: {host}\n\
             network_isolation: {network}\n\
             secret_backend: {secrets}\n\
             node_count: {nodes}\n\
             data_path: {data}\n\
             security:\n\
             \x20\x20tls: required\n\
             \x20\x20mtls_internal: true\n\
             \x20\x20firewall: enabled\n\
             \x20\x20audit_log: /var/log/ancora/audit.log\n",
            product = config.product_name,
            host = config.hostname,
            network = config.network_isolation.as_str(),
            secrets = config.secret_backend.as_str(),
            nodes = config.node_count,
            data = config.data_path,
        );

        Ok(Self { config, rendered })
    }

    pub fn contains(&self, field: &str) -> bool {
        self.rendered.contains(field)
    }
}

/// Errors for on-prem template rendering.
#[derive(Debug, Clone, PartialEq)]
pub enum OnPremError {
    InvalidConfig(String),
}

impl std::fmt::Display for OnPremError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OnPremError::InvalidConfig(msg) => write!(f, "OnPremError: {}", msg),
        }
    }
}

impl std::error::Error for OnPremError {}
