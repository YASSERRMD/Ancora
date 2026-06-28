use crate::index::CatalogIndex;
use crate::metadata::{Author, License, Metadata, Version};
use crate::provider_entry::{ProviderBackend, ProviderEntry};
use crate::search::{search_by_name, search_by_tag, HitKind};
use crate::tool_entry::{ToolEffect, ToolEntry};

fn meta_with_tags(tags: &[&str]) -> Metadata {
    let mut m = Metadata::new(
        Version::new(1, 0, 0),
        Author::new("tester"),
        License::apache2(),
    );
    for t in tags {
        m = m.with_tag(*t);
    }
    m
}

fn build_index() -> CatalogIndex {
    let mut index = CatalogIndex::new();
    index.add_tool(ToolEntry::new(
        "web-search",
        "Web Search",
        "Searches the internet.",
        ToolEffect::ReadOnly,
        meta_with_tags(&["search", "web"]),
    ));
    index.add_tool(ToolEntry::new(
        "calculator",
        "Calculator",
        "Math tool.",
        ToolEffect::None,
        meta_with_tags(&["math"]),
    ));
    index.add_provider(ProviderEntry::new(
        "anthropic-provider",
        "Anthropic",
        "Claude models.",
        ProviderBackend::Anthropic,
        meta_with_tags(&["ai", "search"]),
    ));
    index
}

#[test]
fn search_by_tag_finds_tagged_entries() {
    let index = build_index();
    let hits = search_by_tag(&index, "search");
    assert_eq!(hits.len(), 2, "expected 2 hits for tag 'search'");
}

#[test]
fn search_by_tag_with_no_match_returns_empty() {
    let index = build_index();
    let hits = search_by_tag(&index, "nonexistent-tag");
    assert!(hits.is_empty());
}

#[test]
fn search_by_name_finds_case_insensitively() {
    let index = build_index();
    let hits = search_by_name(&index, "calc");
    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].id, "calculator");
}

#[test]
fn search_by_tag_returns_correct_kind() {
    let index = build_index();
    let hits = search_by_tag(&index, "math");
    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].kind, HitKind::Tool);
}
