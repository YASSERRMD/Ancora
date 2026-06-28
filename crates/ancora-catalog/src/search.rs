use crate::connector_entry::ConnectorEntry;
use crate::index::CatalogIndex;
use crate::provider_entry::ProviderEntry;
use crate::tool_entry::ToolEntry;
use crate::vectorstore_entry::VectorStoreEntry;

/// A lightweight view of a search result, independent of entry kind.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchHit {
    pub id: String,
    pub name: String,
    pub kind: HitKind,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HitKind {
    Tool,
    Connector,
    Provider,
    VectorStore,
}

impl SearchHit {
    fn from_tool(t: &ToolEntry) -> Self {
        Self {
            id: t.id.clone(),
            name: t.name.clone(),
            kind: HitKind::Tool,
            tags: t.metadata.tags.clone(),
        }
    }

    fn from_connector(c: &ConnectorEntry) -> Self {
        Self {
            id: c.id.clone(),
            name: c.name.clone(),
            kind: HitKind::Connector,
            tags: c.metadata.tags.clone(),
        }
    }

    fn from_provider(p: &ProviderEntry) -> Self {
        Self {
            id: p.id.clone(),
            name: p.name.clone(),
            kind: HitKind::Provider,
            tags: p.metadata.tags.clone(),
        }
    }

    fn from_vector_store(v: &VectorStoreEntry) -> Self {
        Self {
            id: v.id.clone(),
            name: v.name.clone(),
            kind: HitKind::VectorStore,
            tags: v.metadata.tags.clone(),
        }
    }
}

/// Search the catalog index for entries whose tags contain `tag`.
pub fn search_by_tag<'a>(index: &'a CatalogIndex, tag: &str) -> Vec<SearchHit> {
    let mut hits = Vec::new();
    for t in &index.tools {
        if t.metadata.has_tag(tag) {
            hits.push(SearchHit::from_tool(t));
        }
    }
    for c in &index.connectors {
        if c.metadata.has_tag(tag) {
            hits.push(SearchHit::from_connector(c));
        }
    }
    for p in &index.providers {
        if p.metadata.has_tag(tag) {
            hits.push(SearchHit::from_provider(p));
        }
    }
    for v in &index.vector_stores {
        if v.metadata.has_tag(tag) {
            hits.push(SearchHit::from_vector_store(v));
        }
    }
    hits
}

/// Search the catalog index for entries whose name or description contains the query
/// (case-insensitive substring match).
pub fn search_by_name(index: &CatalogIndex, query: &str) -> Vec<SearchHit> {
    let q = query.to_lowercase();
    let mut hits = Vec::new();
    for t in &index.tools {
        if t.name.to_lowercase().contains(&q) || t.description.to_lowercase().contains(&q) {
            hits.push(SearchHit::from_tool(t));
        }
    }
    for c in &index.connectors {
        if c.name.to_lowercase().contains(&q) || c.description.to_lowercase().contains(&q) {
            hits.push(SearchHit::from_connector(c));
        }
    }
    for p in &index.providers {
        if p.name.to_lowercase().contains(&q) || p.description.to_lowercase().contains(&q) {
            hits.push(SearchHit::from_provider(p));
        }
    }
    for v in &index.vector_stores {
        if v.name.to_lowercase().contains(&q) || v.description.to_lowercase().contains(&q) {
            hits.push(SearchHit::from_vector_store(v));
        }
    }
    hits
}

/// Filter search results to a specific kind.
pub fn filter_by_kind(hits: Vec<SearchHit>, kind: HitKind) -> Vec<SearchHit> {
    hits.into_iter().filter(|h| h.kind == kind).collect()
}
