use crate::connector_entry::{ConnectorEntry, McpConfig};
use crate::install::ProjectRegistry;
use crate::metadata::{Author, License, Metadata, Version};
use crate::tool_entry::{ToolEffect, ToolEntry};
use crate::validation::{validate_connector, validate_tool};

fn meta() -> Metadata {
    Metadata::new(
        Version::new(1, 0, 0),
        Author::new("YASSERRMD"),
        License::apache2(),
    )
}

fn empty_license_meta() -> Metadata {
    Metadata::new(
        Version::new(1, 0, 0),
        Author::new("YASSERRMD"),
        License::new(""),
    )
}

#[test]
fn tool_with_empty_id_fails_validation() {
    let entry = ToolEntry::new("", "Tool", "desc", ToolEffect::None, meta());
    let errors = validate_tool(&entry);
    assert!(!errors.is_empty());
    assert!(errors.iter().any(|e| e.field == "id"));
}

#[test]
fn tool_with_empty_name_fails_validation() {
    let entry = ToolEntry::new("my-id", "", "desc", ToolEffect::None, meta());
    let errors = validate_tool(&entry);
    assert!(errors.iter().any(|e| e.field == "name"));
}

#[test]
fn tool_with_empty_description_fails_validation() {
    let entry = ToolEntry::new("my-id", "My Tool", "", ToolEffect::None, meta());
    let errors = validate_tool(&entry);
    assert!(errors.iter().any(|e| e.field == "description"));
}

#[test]
fn tool_with_empty_license_fails_validation() {
    let entry = ToolEntry::new("my-id", "My Tool", "desc", ToolEffect::None, empty_license_meta());
    let errors = validate_tool(&entry);
    assert!(errors.iter().any(|e| e.field == "license"));
}

#[test]
fn connector_with_empty_command_fails_validation() {
    let conn = ConnectorEntry::new(
        "c1",
        "Bad Conn",
        "desc",
        McpConfig::stdio("", vec![]),
        meta(),
    );
    let errors = validate_connector(&conn);
    assert!(!errors.is_empty());
}

#[test]
fn install_of_invalid_tool_returns_error() {
    let mut reg = ProjectRegistry::new();
    let bad = ToolEntry::new("", "Tool", "desc", ToolEffect::None, meta());
    let result = reg.install_tool(&bad);
    assert!(result.is_err());
}
