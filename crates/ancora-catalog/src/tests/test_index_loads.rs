use crate::connector_entry::{ConnectorEntry, McpConfig};
use crate::index::CatalogIndex;
use crate::metadata::{Author, License, Metadata, Version};
use crate::tool_entry::{ToolEffect, ToolEntry};

fn meta() -> Metadata {
    Metadata::new(
        Version::new(0, 1, 0),
        Author::new("test-author"),
        License::mit(),
    )
}

#[test]
fn empty_index_has_zero_length() {
    let index = CatalogIndex::new();
    assert_eq!(index.len(), 0);
    assert!(index.is_empty());
}

#[test]
fn adding_tool_increases_length() {
    let mut index = CatalogIndex::new();
    index.add_tool(ToolEntry::new(
        "t1",
        "Tool One",
        "desc",
        ToolEffect::None,
        meta(),
    ));
    assert_eq!(index.len(), 1);
    assert!(!index.is_empty());
}

#[test]
fn find_tool_by_id_returns_correct_entry() {
    let mut index = CatalogIndex::new();
    index.add_tool(ToolEntry::new(
        "my-tool",
        "My Tool",
        "does things",
        ToolEffect::Write,
        meta(),
    ));
    let found = index.find_tool("my-tool");
    assert!(found.is_some());
    assert_eq!(found.unwrap().name, "My Tool");
}

#[test]
fn find_connector_by_id_returns_correct_entry() {
    let mut index = CatalogIndex::new();
    index.add_connector(ConnectorEntry::new(
        "my-conn",
        "My Connector",
        "connects things",
        McpConfig::stdio("my-server", vec![]),
        meta(),
    ));
    let found = index.find_connector("my-conn");
    assert!(found.is_some());
    assert_eq!(found.unwrap().name, "My Connector");
}

#[test]
fn find_returns_none_for_missing_id() {
    let index = CatalogIndex::new();
    assert!(index.find_tool("nonexistent").is_none());
}
