use crate::connector_entry::ConnectorEntry;
use crate::index::CatalogIndex;
use crate::provider_entry::ProviderEntry;
use crate::tool_entry::ToolEntry;
use crate::vectorstore_entry::VectorStoreEntry;

/// A validation error describing why a catalog entry was rejected.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationError {
    pub field: String,
    pub message: String,
}

impl ValidationError {
    pub fn new(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            message: message.into(),
        }
    }
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.field, self.message)
    }
}

/// Validate a [`ToolEntry`] and return all errors found.
pub fn validate_tool(entry: &ToolEntry) -> Vec<ValidationError> {
    let mut errors = Vec::new();
    if entry.id.is_empty() {
        errors.push(ValidationError::new("id", "must not be empty"));
    }
    if entry.name.is_empty() {
        errors.push(ValidationError::new("name", "must not be empty"));
    }
    if entry.description.is_empty() {
        errors.push(ValidationError::new("description", "must not be empty"));
    }
    if entry.metadata.license.is_empty() {
        errors.push(ValidationError::new("license", "must not be empty"));
    }
    if !entry.input_schema.is_valid() {
        errors.push(ValidationError::new(
            "input_schema",
            "contains unnamed fields",
        ));
    }
    if !entry.output_schema.is_valid() {
        errors.push(ValidationError::new(
            "output_schema",
            "contains unnamed fields",
        ));
    }
    errors
}

/// Validate a [`ConnectorEntry`] and return all errors found.
pub fn validate_connector(entry: &ConnectorEntry) -> Vec<ValidationError> {
    let mut errors = Vec::new();
    if entry.id.is_empty() {
        errors.push(ValidationError::new("id", "must not be empty"));
    }
    if entry.name.is_empty() {
        errors.push(ValidationError::new("name", "must not be empty"));
    }
    if entry.description.is_empty() {
        errors.push(ValidationError::new("description", "must not be empty"));
    }
    if !entry.is_valid() {
        errors.push(ValidationError::new(
            "mcp_config",
            "transport command or url must not be empty",
        ));
    }
    errors
}

/// Validate a [`ProviderEntry`] and return all errors found.
pub fn validate_provider(entry: &ProviderEntry) -> Vec<ValidationError> {
    let mut errors = Vec::new();
    if entry.id.is_empty() {
        errors.push(ValidationError::new("id", "must not be empty"));
    }
    if entry.name.is_empty() {
        errors.push(ValidationError::new("name", "must not be empty"));
    }
    errors
}

/// Validate a [`VectorStoreEntry`] and return all errors found.
pub fn validate_vector_store(entry: &VectorStoreEntry) -> Vec<ValidationError> {
    let mut errors = Vec::new();
    if entry.id.is_empty() {
        errors.push(ValidationError::new("id", "must not be empty"));
    }
    if entry.name.is_empty() {
        errors.push(ValidationError::new("name", "must not be empty"));
    }
    errors
}

/// Validate every entry in the index and collect all errors.
pub fn validate_index(index: &CatalogIndex) -> Vec<ValidationError> {
    let mut errors = Vec::new();
    for t in &index.tools {
        errors.extend(validate_tool(t));
    }
    for c in &index.connectors {
        errors.extend(validate_connector(c));
    }
    for p in &index.providers {
        errors.extend(validate_provider(p));
    }
    for v in &index.vector_stores {
        errors.extend(validate_vector_store(v));
    }
    errors
}
