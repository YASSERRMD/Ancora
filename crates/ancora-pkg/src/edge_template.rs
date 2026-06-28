//! Single-binary edge deployment template.
//!
//! Generates configuration and build specs for packaging Ancora as a
//! single static binary suitable for edge and IoT deployments.

use std::collections::HashMap;

/// Target architecture for the edge binary.
#[derive(Debug, Clone, PartialEq)]
pub enum EdgeArch {
    X86_64,
    Aarch64,
    Armv7,
    RiscV64,
}

impl EdgeArch {
    pub fn triple(&self) -> &'static str {
        match self {
            EdgeArch::X86_64 => "x86_64-unknown-linux-musl",
            EdgeArch::Aarch64 => "aarch64-unknown-linux-musl",
            EdgeArch::Armv7 => "armv7-unknown-linux-musleabihf",
            EdgeArch::RiscV64 => "riscv64gc-unknown-linux-gnu",
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            EdgeArch::X86_64 => "x86_64",
            EdgeArch::Aarch64 => "aarch64",
            EdgeArch::Armv7 => "armv7",
            EdgeArch::RiscV64 => "riscv64",
        }
    }
}

/// Resource constraints for an edge device.
#[derive(Debug, Clone)]
pub struct EdgeConstraints {
    pub max_memory_mb: u32,
    pub max_cpu_percent: u8,
    pub storage_limit_mb: u32,
}

impl Default for EdgeConstraints {
    fn default() -> Self {
        Self {
            max_memory_mb: 256,
            max_cpu_percent: 50,
            storage_limit_mb: 512,
        }
    }
}

/// Configuration for an edge binary build.
#[derive(Debug, Clone)]
pub struct EdgeConfig {
    pub product_name: String,
    pub version: String,
    pub arch: EdgeArch,
    pub constraints: EdgeConstraints,
    pub features: Vec<String>,
    pub env: HashMap<String, String>,
    pub static_binary: bool,
    pub strip_symbols: bool,
}

impl EdgeConfig {
    pub fn new(
        product_name: impl Into<String>,
        version: impl Into<String>,
        arch: EdgeArch,
    ) -> Self {
        Self {
            product_name: product_name.into(),
            version: version.into(),
            arch,
            constraints: EdgeConstraints::default(),
            features: vec![],
            env: HashMap::new(),
            static_binary: true,
            strip_symbols: true,
        }
    }

    pub fn with_feature(mut self, feature: impl Into<String>) -> Self {
        self.features.push(feature.into());
        self
    }

    pub fn with_constraints(mut self, c: EdgeConstraints) -> Self {
        self.constraints = c;
        self
    }
}

/// Rendered edge build artifact template.
#[derive(Debug, Clone)]
pub struct EdgeTemplate {
    pub config: EdgeConfig,
    pub build_spec: String,
    pub runtime_config: String,
}

impl EdgeTemplate {
    pub fn render(config: EdgeConfig) -> Result<Self, EdgeError> {
        if config.product_name.is_empty() {
            return Err(EdgeError::InvalidConfig("product_name is required".to_string()));
        }
        if config.version.is_empty() {
            return Err(EdgeError::InvalidConfig("version is required".to_string()));
        }

        let features_line = if config.features.is_empty() {
            "default".to_string()
        } else {
            config.features.join(",")
        };

        let build_spec = format!(
            "# ancora-pkg edge build spec\n\
             product: {product}\n\
             version: {version}\n\
             target: {triple}\n\
             arch: {arch}\n\
             static_binary: {static_b}\n\
             strip_symbols: {strip}\n\
             features: [{features}]\n\
             cargo_flags:\n\
             \x20\x20- --release\n\
             \x20\x20- --target {triple}\n\
             \x20\x20- --features {features}\n",
            product = config.product_name,
            version = config.version,
            triple = config.arch.triple(),
            arch = config.arch.as_str(),
            static_b = config.static_binary,
            strip = config.strip_symbols,
            features = features_line,
        );

        let runtime_config = format!(
            "# ancora-pkg edge runtime config\n\
             product: {product}\n\
             max_memory_mb: {mem}\n\
             max_cpu_percent: {cpu}\n\
             storage_limit_mb: {storage}\n\
             security:\n\
             \x20\x20tls: required\n\
             \x20\x20audit_log: enabled\n\
             \x20\x20no_root: true\n",
            product = config.product_name,
            mem = config.constraints.max_memory_mb,
            cpu = config.constraints.max_cpu_percent,
            storage = config.constraints.storage_limit_mb,
        );

        Ok(Self { config, build_spec, runtime_config })
    }

    pub fn binary_name(&self) -> String {
        format!("{}-{}-{}", self.config.product_name, self.config.version, self.config.arch.as_str())
    }

    pub fn contains_in_build(&self, field: &str) -> bool {
        self.build_spec.contains(field)
    }
}

/// Errors for edge template rendering.
#[derive(Debug, Clone, PartialEq)]
pub enum EdgeError {
    InvalidConfig(String),
}

impl std::fmt::Display for EdgeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EdgeError::InvalidConfig(msg) => write!(f, "EdgeError: {}", msg),
        }
    }
}

impl std::error::Error for EdgeError {}
