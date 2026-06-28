use crate::connector_entry::{ConnectorEntry, McpConfig};
use crate::install::{InstalledKind, ProjectRegistry};
use crate::metadata::{Author, License, Metadata, Version};
use crate::tool_entry::{ToolEffect, ToolEntry};
use crate::vectorstore_entry::{VectorStoreBackend, VectorStoreEntry};

fn meta() -> Metadata {
    Metadata::new(
        Version::new(1, 0, 0),
        Author::new("YASSERRMD"),
        License::apache2(),
    )
}

#[test]
fn installing_tool_increases_registry_count() {
    let mut reg = ProjectRegistry::new();
    let tool = ToolEntry::new("t1", "Tool One", "A tool.", ToolEffect::None, meta());
    reg.install_tool(&tool).expect("install should succeed");
    assert_eq!(reg.installed_count(), 1);
}

#[test]
fn installed_tool_is_found_by_id() {
    let mut reg = ProjectRegistry::new();
    let tool = ToolEntry::new("t2", "Tool Two", "Another tool.", ToolEffect::Write, meta());
    reg.install_tool(&tool).unwrap();
    assert!(reg.is_installed("t2"));
}

#[test]
fn duplicate_install_returns_error() {
    let mut reg = ProjectRegistry::new();
    let tool = ToolEntry::new("t3", "Tool Three", "Desc.", ToolEffect::None, meta());
    reg.install_tool(&tool).unwrap();
    let result = reg.install_tool(&tool);
    assert!(result.is_err());
}

#[test]
fn install_connector_adds_to_registry() {
    let mut reg = ProjectRegistry::new();
    let conn = ConnectorEntry::new(
        "slack-conn",
        "Slack Connector",
        "Slack MCP connector.",
        McpConfig::stdio("slack-mcp", vec![]),
        meta(),
    );
    reg.install_connector(&conn).unwrap();
    let entry = reg.find_installed("slack-conn").unwrap();
    assert_eq!(entry.kind, InstalledKind::Connector);
}

#[test]
fn install_vector_store_adds_to_registry() {
    let mut reg = ProjectRegistry::new();
    let vs = VectorStoreEntry::new(
        "lancedb-1",
        "LanceDB",
        "Fast local vector store.",
        VectorStoreBackend::LanceDb,
        meta(),
    );
    reg.install_vector_store(&vs).unwrap();
    assert!(reg.is_installed("lancedb-1"));
}
