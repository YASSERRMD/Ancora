//! White-label configuration for custom-branded deployments.
//!
//! Allows partners and OEM customers to rebrand the product with their own
//! name, logo, colours, and domain while preserving all security controls.

use std::collections::HashMap;

/// Colour scheme for a white-label deployment.
#[derive(Debug, Clone)]
pub struct ColorScheme {
    pub primary: String,
    pub secondary: String,
    pub accent: String,
    pub background: String,
}

impl Default for ColorScheme {
    fn default() -> Self {
        Self {
            primary: "#0f172a".to_string(),
            secondary: "#1e293b".to_string(),
            accent: "#6366f1".to_string(),
            background: "#ffffff".to_string(),
        }
    }
}

impl ColorScheme {
    /// Returns true if all colour values are non-empty.
    pub fn is_valid(&self) -> bool {
        !self.primary.is_empty()
            && !self.secondary.is_empty()
            && !self.accent.is_empty()
            && !self.background.is_empty()
    }
}

/// Brand identity for a white-label deployment.
#[derive(Debug, Clone)]
pub struct BrandIdentity {
    pub brand_name: String,
    pub logo_url: String,
    pub favicon_url: String,
    pub support_email: String,
    pub docs_url: String,
}

impl BrandIdentity {
    pub fn new(brand_name: impl Into<String>) -> Self {
        Self {
            brand_name: brand_name.into(),
            logo_url: String::new(),
            favicon_url: String::new(),
            support_email: String::new(),
            docs_url: String::new(),
        }
    }

    pub fn with_logo(mut self, url: impl Into<String>) -> Self {
        self.logo_url = url.into();
        self
    }

    pub fn with_support(mut self, email: impl Into<String>) -> Self {
        self.support_email = email.into();
        self
    }
}

/// Full white-label configuration.
#[derive(Debug, Clone)]
pub struct WhitelabelConfig {
    pub brand: BrandIdentity,
    pub colors: ColorScheme,
    pub custom_domain: String,
    pub feature_overrides: HashMap<String, bool>,
    pub terms_url: String,
    pub privacy_url: String,
}

impl WhitelabelConfig {
    pub fn new(brand: BrandIdentity, custom_domain: impl Into<String>) -> Self {
        Self {
            brand,
            colors: ColorScheme::default(),
            custom_domain: custom_domain.into(),
            feature_overrides: HashMap::new(),
            terms_url: String::new(),
            privacy_url: String::new(),
        }
    }

    pub fn with_colors(mut self, colors: ColorScheme) -> Self {
        self.colors = colors;
        self
    }

    pub fn with_feature_override(mut self, key: impl Into<String>, enabled: bool) -> Self {
        self.feature_overrides.insert(key.into(), enabled);
        self
    }
}

/// Validation result for a white-label config.
#[derive(Debug, Clone, PartialEq)]
pub struct WhitelabelValidation {
    pub valid: bool,
    pub errors: Vec<String>,
}

/// Applied white-label configuration artifact.
#[derive(Debug, Clone)]
pub struct WhitelabelTemplate {
    pub config: WhitelabelConfig,
    pub rendered: String,
}

impl WhitelabelTemplate {
    /// Validates the white-label configuration.
    pub fn validate(config: &WhitelabelConfig) -> WhitelabelValidation {
        let mut errors = vec![];
        if config.brand.brand_name.is_empty() {
            errors.push("brand_name is required".to_string());
        }
        if config.custom_domain.is_empty() {
            errors.push("custom_domain is required".to_string());
        }
        if !config.colors.is_valid() {
            errors.push("all color fields must be non-empty".to_string());
        }
        WhitelabelValidation {
            valid: errors.is_empty(),
            errors,
        }
    }

    /// Applies the white-label config and renders the output artifact.
    pub fn apply(config: WhitelabelConfig) -> Result<Self, WhitelabelError> {
        let validation = Self::validate(&config);
        if !validation.valid {
            return Err(WhitelabelError::ValidationFailed(validation.errors));
        }

        let overrides_section: String = config
            .feature_overrides
            .iter()
            .map(|(k, v)| format!("  {}: {}\n", k, v))
            .collect();

        let rendered = format!(
            "# ancora-pkg white-label configuration\n\
             brand_name: {brand}\n\
             custom_domain: {domain}\n\
             logo_url: {logo}\n\
             support_email: {support}\n\
             colors:\n\
             \x20\x20primary: {primary}\n\
             \x20\x20secondary: {secondary}\n\
             \x20\x20accent: {accent}\n\
             feature_overrides:\n\
             {overrides}\
             security:\n\
             \x20\x20branding_does_not_affect_security: true\n\
             \x20\x20audit_log_brand: {brand}\n",
            brand = config.brand.brand_name,
            domain = config.custom_domain,
            logo = config.brand.logo_url,
            support = config.brand.support_email,
            primary = config.colors.primary,
            secondary = config.colors.secondary,
            accent = config.colors.accent,
            overrides = if overrides_section.is_empty() {
                "  {}\n".to_string()
            } else {
                overrides_section
            },
        );

        Ok(Self { config, rendered })
    }

    pub fn contains(&self, field: &str) -> bool {
        self.rendered.contains(field)
    }
}

/// Errors for white-label configuration.
#[derive(Debug, Clone, PartialEq)]
pub enum WhitelabelError {
    ValidationFailed(Vec<String>),
}

impl std::fmt::Display for WhitelabelError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WhitelabelError::ValidationFailed(errs) => {
                write!(f, "WhitelabelError: {}", errs.join(", "))
            }
        }
    }
}

impl std::error::Error for WhitelabelError {}
