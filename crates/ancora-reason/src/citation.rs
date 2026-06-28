//! Citation store: attach source citations to claims.

use std::collections::HashMap;

#[derive(Default)]
pub struct CitationStore {
    citations: HashMap<String, Vec<String>>,
}

impl CitationStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, claim: &str, citation: String) {
        self.citations
            .entry(claim.to_string())
            .or_default()
            .push(citation);
    }

    pub fn get(&self, claim: &str) -> &[String] {
        self.citations
            .get(claim)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    pub fn has_citations(&self, claim: &str) -> bool {
        !self.get(claim).is_empty()
    }

    pub fn all_cited_claims(&self) -> Vec<&str> {
        self.citations.keys().map(|s| s.as_str()).collect()
    }
}
