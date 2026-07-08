//! SaaS reference deployment template.
//!
//! Generates a secure-by-default configuration scaffold for multi-tenant SaaS
//! products running on public cloud infrastructure.

use std::collections::HashMap;

/// Deployment tiers supported by the SaaS template.
#[derive(Debug, Clone, PartialEq)]
pub enum SaasTier {
    Development,
    Staging,
    Production,
}

impl SaasTier {
    /// Returns the tier name as a static string slice.
    pub fn as_str(&self) -> &'static str {
        match self {
            SaasTier::Development => "development",
            SaasTier::Staging => "staging",
            SaasTier::Production => "production",
        }
    }
}

/// Secure defaults applied to every SaaS template.
#[derive(Debug, Clone)]
pub struct SecureDefaults {
    pub tls_min_version: String,
    pub hsts_max_age_seconds: u64,
    pub csp_enabled: bool,
    pub rate_limiting_enabled: bool,
    pub audit_logging_enabled: bool,
    pub mfa_required: bool,
}

impl Default for SecureDefaults {
    fn default() -> Self {
        Self {
            tls_min_version: "TLSv1.3".to_string(),
            hsts_max_age_seconds: 31_536_000, // 1 year
            csp_enabled: true,
            rate_limiting_enabled: true,
            audit_logging_enabled: true,
            mfa_required: true,
        }
    }
}

/// Configuration for a SaaS deployment.
#[derive(Debug, Clone)]
pub struct SaasConfig {
    pub product_name: String,
    pub tier: SaasTier,
    pub replicas: u32,
    pub region: String,
    pub secure_defaults: SecureDefaults,
    pub feature_flags: HashMap<String, bool>,
}

impl SaasConfig {
    /// Creates a new SaaS config with secure defaults.
    pub fn new(product_name: impl Into<String>, tier: SaasTier, region: impl Into<String>) -> Self {
        let replicas = match &tier {
            SaasTier::Development => 1,
            SaasTier::Staging => 2,
            SaasTier::Production => 3,
        };
        Self {
            product_name: product_name.into(),
            tier,
            replicas,
            region: region.into(),
            secure_defaults: SecureDefaults::default(),
            feature_flags: HashMap::new(),
        }
    }

    /// Sets a feature flag on the config.
    pub fn with_feature(mut self, key: impl Into<String>, value: bool) -> Self {
        self.feature_flags.insert(key.into(), value);
        self
    }
}

/// Rendered SaaS template artifact.
#[derive(Debug, Clone)]
pub struct SaasTemplate {
    pub config: SaasConfig,
    pub rendered_yaml: String,
}

impl SaasTemplate {
    /// Renders the SaaS template from the given config.
    pub fn render(config: SaasConfig) -> Result<Self, TemplateError> {
        if config.product_name.is_empty() {
            return Err(TemplateError::InvalidConfig(
                "product_name must not be empty".to_string(),
            ));
        }
        if config.replicas == 0 {
            return Err(TemplateError::InvalidConfig(
                "replicas must be at least 1".to_string(),
            ));
        }

        let yaml = format!(
            "# ancora-pkg SaaS template - {product}\n\
             apiVersion: apps/v1\n\
             kind: Deployment\n\
             metadata:\n\
             \x20\x20name: {product}\n\
             \x20\x20labels:\n\
             \x20\x20\x20\x20tier: {tier}\n\
             \x20\x20\x20\x20region: {region}\n\
             spec:\n\
             \x20\x20replicas: {replicas}\n\
             \x20\x20template:\n\
             \x20\x20\x20\x20spec:\n\
             \x20\x20\x20\x20\x20\x20securityContext:\n\
             \x20\x20\x20\x20\x20\x20\x20\x20runAsNonRoot: true\n\
             \x20\x20\x20\x20\x20\x20\x20\x20readOnlyRootFilesystem: true\n\
             \x20\x20\x20\x20\x20\x20containers:\n\
             \x20\x20\x20\x20\x20\x20- name: {product}\n\
             \x20\x20\x20\x20\x20\x20\x20\x20tls_min_version: {tls}\n\
             \x20\x20\x20\x20\x20\x20\x20\x20audit_logging: {audit}\n\
             \x20\x20\x20\x20\x20\x20\x20\x20mfa_required: {mfa}\n",
            product = config.product_name,
            tier = config.tier.as_str(),
            region = config.region,
            replicas = config.replicas,
            tls = config.secure_defaults.tls_min_version,
            audit = config.secure_defaults.audit_logging_enabled,
            mfa = config.secure_defaults.mfa_required,
        );

        Ok(Self {
            config,
            rendered_yaml: yaml,
        })
    }

    /// Returns true if the rendered template contains a required security field.
    pub fn has_security_field(&self, field: &str) -> bool {
        self.rendered_yaml.contains(field)
    }
}

/// Errors that can occur during template rendering.
#[derive(Debug, Clone, PartialEq)]
pub enum TemplateError {
    InvalidConfig(String),
    RenderFailed(String),
}

impl std::fmt::Display for TemplateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TemplateError::InvalidConfig(msg) => write!(f, "InvalidConfig: {}", msg),
            TemplateError::RenderFailed(msg) => write!(f, "RenderFailed: {}", msg),
        }
    }
}

impl std::error::Error for TemplateError {}
