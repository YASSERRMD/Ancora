//! SDK extension ergonomics - builder helpers and convenience macros.
//!
//! Provides a fluent builder API so plugin authors can configure
//! their plugin metadata without verbose struct literals.

use crate::plugin_sdk::PluginMeta;

/// Fluent builder for `PluginMeta`.
#[derive(Debug, Default)]
pub struct PluginMetaBuilder {
    name: Option<String>,
    version: Option<String>,
    author: Option<String>,
    description: Option<String>,
}

impl PluginMetaBuilder {
    /// Create a new builder.
    pub fn new() -> Self {
        Self::default()
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }

    pub fn author(mut self, author: impl Into<String>) -> Self {
        self.author = Some(author.into());
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Build the `PluginMeta`, returning an error if any required field is missing.
    pub fn build(self) -> Result<PluginMeta, String> {
        Ok(PluginMeta {
            name: self.name.ok_or("name is required")?,
            version: self.version.ok_or("version is required")?,
            author: self.author.ok_or("author is required")?,
            description: self.description.unwrap_or_default(),
        })
    }
}

/// Convenience: create a `PluginMeta` in a single expression.
pub fn quick_meta(
    name: &str,
    version: &str,
    author: &str,
    description: &str,
) -> PluginMeta {
    PluginMeta {
        name: name.into(),
        version: version.into(),
        author: author.into(),
        description: description.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_produces_correct_meta() {
        let meta = PluginMetaBuilder::new()
            .name("test-plugin")
            .version("1.0.0")
            .author("Bob")
            .description("A test plugin")
            .build()
            .unwrap();

        assert_eq!(meta.name, "test-plugin");
        assert_eq!(meta.version, "1.0.0");
    }

    #[test]
    fn builder_missing_name_fails() {
        let result = PluginMetaBuilder::new()
            .version("1.0.0")
            .author("Bob")
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn quick_meta_works() {
        let meta = quick_meta("q", "0.1.0", "Alice", "quick");
        assert_eq!(meta.name, "q");
    }
}
