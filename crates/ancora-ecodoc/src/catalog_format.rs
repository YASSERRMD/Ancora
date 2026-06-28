//! Catalog format specification for the Ancora plugin registry.
//!
//! Defines the machine-readable manifest that each published plugin
//! must include in its crate root.

/// Current catalog schema version.
pub const CATALOG_SCHEMA_VERSION: u32 = 1;

/// A single entry in the plugin catalog.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CatalogEntry {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub license: String,
    pub keywords: Vec<String>,
    pub schema_version: u32,
}

impl CatalogEntry {
    /// Create a new catalog entry with the current schema version.
    pub fn new(
        name: impl Into<String>,
        version: impl Into<String>,
        description: impl Into<String>,
        author: impl Into<String>,
        license: impl Into<String>,
        keywords: Vec<String>,
    ) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
            description: description.into(),
            author: author.into(),
            license: license.into(),
            keywords,
            schema_version: CATALOG_SCHEMA_VERSION,
        }
    }

    /// Validate the entry for required fields.
    pub fn validate(&self) -> Result<(), String> {
        if self.name.is_empty() {
            return Err("name must not be empty".into());
        }
        if self.version.is_empty() {
            return Err("version must not be empty".into());
        }
        if self.license.is_empty() {
            return Err("license must not be empty".into());
        }
        if self.schema_version != CATALOG_SCHEMA_VERSION {
            return Err(format!(
                "unsupported schema version {}; expected {CATALOG_SCHEMA_VERSION}",
                self.schema_version
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> CatalogEntry {
        CatalogEntry::new(
            "my-plugin",
            "0.1.0",
            "A sample plugin",
            "Alice",
            "MIT",
            vec!["sample".into()],
        )
    }

    #[test]
    fn valid_entry_passes() {
        assert!(sample().validate().is_ok());
    }

    #[test]
    fn empty_name_fails() {
        let mut e = sample();
        e.name = String::new();
        assert!(e.validate().is_err());
    }

    #[test]
    fn wrong_schema_version_fails() {
        let mut e = sample();
        e.schema_version = 99;
        assert!(e.validate().is_err());
    }
}
