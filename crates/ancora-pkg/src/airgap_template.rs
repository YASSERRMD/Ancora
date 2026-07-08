//! Air-gapped appliance deployment template.
//!
//! Generates configuration for fully air-gapped deployments where no external
//! network access is permitted. All artifacts must be bundled locally.

use std::collections::HashMap;

/// Artifact source for air-gapped environments.
#[derive(Debug, Clone, PartialEq)]
pub enum ArtifactSource {
    /// All images and packages are mirrored in a local registry.
    LocalRegistry(String),
    /// Artifacts are provided as tar bundles on disk.
    TarBundle(String),
}

impl ArtifactSource {
    pub fn registry_url(&self) -> Option<&str> {
        match self {
            ArtifactSource::LocalRegistry(url) => Some(url.as_str()),
            ArtifactSource::TarBundle(_) => None,
        }
    }

    pub fn bundle_path(&self) -> Option<&str> {
        match self {
            ArtifactSource::LocalRegistry(_) => None,
            ArtifactSource::TarBundle(path) => Some(path.as_str()),
        }
    }
}

/// Configuration for an air-gapped deployment.
#[derive(Debug, Clone)]
pub struct AirgapConfig {
    pub product_name: String,
    pub version: String,
    pub artifact_source: ArtifactSource,
    pub offline_license_path: String,
    pub node_count: u32,
    pub extra: HashMap<String, String>,
}

impl AirgapConfig {
    pub fn new(
        product_name: impl Into<String>,
        version: impl Into<String>,
        artifact_source: ArtifactSource,
        offline_license_path: impl Into<String>,
    ) -> Self {
        Self {
            product_name: product_name.into(),
            version: version.into(),
            artifact_source,
            offline_license_path: offline_license_path.into(),
            node_count: 1,
            extra: HashMap::new(),
        }
    }

    pub fn with_node_count(mut self, count: u32) -> Self {
        self.node_count = count;
        self
    }
}

/// Validation result for an air-gap config.
#[derive(Debug, Clone, PartialEq)]
pub struct ValidationReport {
    pub passed: bool,
    pub issues: Vec<String>,
}

impl ValidationReport {
    fn ok() -> Self {
        Self {
            passed: true,
            issues: vec![],
        }
    }

    fn fail(issues: Vec<String>) -> Self {
        Self {
            passed: false,
            issues,
        }
    }
}

/// Rendered air-gapped appliance template.
#[derive(Debug, Clone)]
pub struct AirgapTemplate {
    pub config: AirgapConfig,
    pub rendered: String,
}

impl AirgapTemplate {
    /// Validates the air-gap config before rendering.
    pub fn validate(config: &AirgapConfig) -> ValidationReport {
        let mut issues = vec![];
        if config.product_name.is_empty() {
            issues.push("product_name is required".to_string());
        }
        if config.version.is_empty() {
            issues.push("version is required".to_string());
        }
        if config.offline_license_path.is_empty() {
            issues.push("offline_license_path is required".to_string());
        }
        if config.node_count == 0 {
            issues.push("node_count must be >= 1".to_string());
        }
        if issues.is_empty() {
            ValidationReport::ok()
        } else {
            ValidationReport::fail(issues)
        }
    }

    /// Renders the air-gapped template. Returns an error if validation fails.
    pub fn render(config: AirgapConfig) -> Result<Self, AirgapError> {
        let report = Self::validate(&config);
        if !report.passed {
            return Err(AirgapError::ValidationFailed(report.issues));
        }

        let source_line = match &config.artifact_source {
            ArtifactSource::LocalRegistry(url) => format!("local_registry: {}", url),
            ArtifactSource::TarBundle(path) => format!("tar_bundle: {}", path),
        };

        let rendered = format!(
            "# ancora-pkg air-gapped appliance template\n\
             product: {product}\n\
             version: {version}\n\
             artifact_source:\n\
             \x20\x20{source}\n\
             offline_license: {license}\n\
             node_count: {nodes}\n\
             network:\n\
             \x20\x20external_access: false\n\
             \x20\x20dns_override: enabled\n\
             \x20\x20proxy: none\n\
             security:\n\
             \x20\x20tls: required\n\
             \x20\x20fips_mode: true\n\
             \x20\x20audit_log: enabled\n",
            product = config.product_name,
            version = config.version,
            source = source_line,
            license = config.offline_license_path,
            nodes = config.node_count,
        );

        Ok(Self { config, rendered })
    }

    pub fn contains(&self, field: &str) -> bool {
        self.rendered.contains(field)
    }
}

/// Errors for air-gapped template rendering.
#[derive(Debug, Clone, PartialEq)]
pub enum AirgapError {
    ValidationFailed(Vec<String>),
}

impl std::fmt::Display for AirgapError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AirgapError::ValidationFailed(issues) => {
                write!(f, "AirgapError: validation failed: {}", issues.join(", "))
            }
        }
    }
}

impl std::error::Error for AirgapError {}
