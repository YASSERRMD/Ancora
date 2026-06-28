use crate::versioning::Version;

/// A search query submitted to the registry.
#[derive(Debug, Clone)]
pub struct SearchQuery {
    /// Free-text search term; matched as a case-insensitive substring against entry names.
    pub term: String,
    /// Optional tag filter; if set only entries carrying this tag are returned.
    pub tag: Option<String>,
    /// Maximum number of results to return; 0 means unlimited.
    pub limit: usize,
}

impl SearchQuery {
    /// Build a simple term-only query.
    pub fn new(term: impl Into<String>) -> Self {
        Self {
            term: term.into(),
            tag: None,
            limit: 0,
        }
    }

    /// Attach a tag filter.
    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tag = Some(tag.into());
        self
    }

    /// Restrict result count.
    pub fn with_limit(mut self, limit: usize) -> Self {
        self.limit = limit;
        self
    }
}

/// A single search result returned by the registry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchHit {
    /// Entry name.
    pub name: String,
    /// Latest version available, if any.
    pub latest: Option<Version>,
    /// Total number of versions published.
    pub version_count: usize,
}

/// Apply the limit from a query to a result set.
///
/// If `query.limit` is 0 the full set is returned.
pub fn apply_limit(hits: Vec<SearchHit>, query: &SearchQuery) -> Vec<SearchHit> {
    if query.limit == 0 || hits.len() <= query.limit {
        hits
    } else {
        hits.into_iter().take(query.limit).collect()
    }
}
