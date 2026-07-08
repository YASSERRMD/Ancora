//! Extension metadata schema for the marketplace.
//!
//! Every published extension must supply a complete `ExtensionMetadata` record.
//! Fields are validated before the manifest is accepted by the registry.

#[derive(Debug, Clone, PartialEq)]
pub struct ExtensionMetadata {
    /// Unique reverse-DNS style identifier, e.g. "com.example.my-ext".
    pub id: String,
    /// Human-readable display name.
    pub name: String,
    /// Semantic version string (major.minor.patch).
    pub version: String,
    /// Short description (max 256 chars).
    pub description: String,
    /// SPDX license identifier, e.g. "Apache-2.0".
    pub license: String,
    /// Homepage or repository URL.
    pub homepage: Option<String>,
    /// Categories that help users discover the extension.
    pub categories: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MetadataError {
    EmptyId,
    EmptyName,
    InvalidVersion(String),
    EmptyLicense,
    DescriptionTooLong(usize),
}

impl std::fmt::Display for MetadataError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MetadataError::EmptyId => write!(f, "extension id must not be empty"),
            MetadataError::EmptyName => write!(f, "extension name must not be empty"),
            MetadataError::InvalidVersion(v) => write!(f, "invalid version string: '{}'", v),
            MetadataError::EmptyLicense => write!(f, "license must not be empty"),
            MetadataError::DescriptionTooLong(n) => {
                write!(f, "description is {} chars, max 256", n)
            }
        }
    }
}

impl ExtensionMetadata {
    /// Construct and validate a new `ExtensionMetadata`.
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        version: impl Into<String>,
        description: impl Into<String>,
        license: impl Into<String>,
    ) -> Result<Self, MetadataError> {
        let id = id.into();
        let name = name.into();
        let version = version.into();
        let description = description.into();
        let license = license.into();

        if id.is_empty() {
            return Err(MetadataError::EmptyId);
        }
        if name.is_empty() {
            return Err(MetadataError::EmptyName);
        }
        if !is_valid_semver(&version) {
            return Err(MetadataError::InvalidVersion(version));
        }
        if license.is_empty() {
            return Err(MetadataError::EmptyLicense);
        }
        let desc_len = description.chars().count();
        if desc_len > 256 {
            return Err(MetadataError::DescriptionTooLong(desc_len));
        }

        Ok(ExtensionMetadata {
            id,
            name,
            version,
            description,
            license,
            homepage: None,
            categories: Vec::new(),
        })
    }

    /// Validate an existing record.
    pub fn validate(&self) -> Result<(), MetadataError> {
        if self.id.is_empty() {
            return Err(MetadataError::EmptyId);
        }
        if self.name.is_empty() {
            return Err(MetadataError::EmptyName);
        }
        if !is_valid_semver(&self.version) {
            return Err(MetadataError::InvalidVersion(self.version.clone()));
        }
        if self.license.is_empty() {
            return Err(MetadataError::EmptyLicense);
        }
        let desc_len = self.description.chars().count();
        if desc_len > 256 {
            return Err(MetadataError::DescriptionTooLong(desc_len));
        }
        Ok(())
    }
}

/// Minimal semver check: three dot-separated numeric components.
fn is_valid_semver(v: &str) -> bool {
    let parts: Vec<&str> = v.split('.').collect();
    if parts.len() != 3 {
        return false;
    }
    parts.iter().all(|p| p.parse::<u64>().is_ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_metadata_constructs() {
        let m = ExtensionMetadata::new(
            "com.example.ext",
            "My Ext",
            "1.0.0",
            "A test extension",
            "Apache-2.0",
        );
        assert!(m.is_ok());
    }

    #[test]
    fn empty_id_rejected() {
        let m = ExtensionMetadata::new("", "Name", "1.0.0", "desc", "MIT");
        assert_eq!(m, Err(MetadataError::EmptyId));
    }

    #[test]
    fn bad_version_rejected() {
        let m = ExtensionMetadata::new("id", "Name", "1.0", "desc", "MIT");
        assert!(matches!(m, Err(MetadataError::InvalidVersion(_))));
    }
}
