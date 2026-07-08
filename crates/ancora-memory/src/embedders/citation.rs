/// Citation metadata passthrough for the retrieval pipeline.
///
/// When passages are retrieved and assembled into a context, the source
/// metadata (URL, title, page number, chunk index) must survive so that the
/// LLM can cite the source or the application can display attribution.
///
/// This module provides a lightweight `CitationRecord` and helpers to attach
/// and recover citations from assembled context.
use serde_json::{json, Value};

// ---- citation record ---------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub struct CitationRecord {
    /// Sequential citation number (1-based).
    pub number: usize,
    /// Source identifier (file path, URL, document ID, etc.).
    pub source: String,
    /// Optional human-readable title.
    pub title: Option<String>,
    /// Optional page or section within the source.
    pub location: Option<String>,
    /// The exact chunk text that was retrieved.
    pub chunk_text: String,
    /// Retrieval score from the vector store.
    pub score: f32,
    /// Arbitrary extra metadata.
    pub metadata: Value,
}

impl CitationRecord {
    pub fn new(
        number: usize,
        source: impl Into<String>,
        chunk_text: impl Into<String>,
        score: f32,
    ) -> Self {
        Self {
            number,
            source: source.into(),
            title: None,
            location: None,
            chunk_text: chunk_text.into(),
            score,
            metadata: json!({}),
        }
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn with_location(mut self, loc: impl Into<String>) -> Self {
        self.location = Some(loc.into());
        self
    }

    pub fn with_metadata(mut self, meta: Value) -> Self {
        self.metadata = meta;
        self
    }

    /// Format as an inline citation marker, e.g. `[1]`.
    pub fn inline_marker(&self) -> String {
        format!("[{}]", self.number)
    }

    /// Format as a footnote entry for appending to the response.
    pub fn footnote(&self) -> String {
        let title = self.title.as_deref().unwrap_or(&self.source);
        let loc = self
            .location
            .as_deref()
            .map(|l| format!(", {l}"))
            .unwrap_or_default();
        format!("[{}] {}{}", self.number, title, loc)
    }

    pub fn to_json(&self) -> Value {
        json!({
            "number": self.number,
            "source": self.source,
            "title": self.title,
            "location": self.location,
            "score": self.score,
            "chunk_text": self.chunk_text,
        })
    }
}

// ---- citation list helpers ---------------------------------------------

/// Build a citation list from (source, score, chunk_text) triples.
pub fn build_citations(triples: &[(&str, f32, &str)]) -> Vec<CitationRecord> {
    triples
        .iter()
        .enumerate()
        .map(|(i, (source, score, text))| CitationRecord::new(i + 1, *source, *text, *score))
        .collect()
}

/// Format a list of citations as a Markdown footnote block.
pub fn format_footnote_block(citations: &[CitationRecord]) -> String {
    citations
        .iter()
        .map(|c| c.footnote())
        .collect::<Vec<_>>()
        .join("\n")
}

/// Format citations as a JSON array for programmatic use.
pub fn citations_to_json(citations: &[CitationRecord]) -> Value {
    Value::Array(citations.iter().map(|c| c.to_json()).collect())
}

/// Filter citations by minimum score threshold.
pub fn filter_by_score(citations: Vec<CitationRecord>, min_score: f32) -> Vec<CitationRecord> {
    citations
        .into_iter()
        .filter(|c| c.score >= min_score)
        .collect()
}

/// Deduplicate citations by source (keep the highest-scoring occurrence).
pub fn dedup_by_source(mut citations: Vec<CitationRecord>) -> Vec<CitationRecord> {
    citations.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    let mut seen = std::collections::HashSet::new();
    citations.retain(|c| seen.insert(c.source.clone()));
    citations
}

// ---- tests ---------------------------------------------------------------

#[cfg(test)]
mod citation_tests {
    use super::*;

    #[test]
    fn inline_marker_format() {
        let c = CitationRecord::new(3, "doc.txt", "chunk", 0.9);
        assert_eq!(c.inline_marker(), "[3]");
    }

    #[test]
    fn footnote_includes_number_and_source() {
        let c =
            CitationRecord::new(1, "https://example.com", "chunk", 0.8).with_title("Example Page");
        let fn_text = c.footnote();
        assert!(fn_text.contains("[1]"), "footnote: {fn_text}");
        assert!(fn_text.contains("Example Page"), "footnote: {fn_text}");
    }

    #[test]
    fn footnote_with_location() {
        // Shadowed by the next let binding below.
        let _c = CitationRecord::new(2, "book.pdf", "text", 0.7);
        // CitationRecord::new with only 3 args -- need 4 args per signature
        let c = CitationRecord::new(2, "book.pdf", "text", 0.7).with_location("page 42");
        let fn_text = c.footnote();
        assert!(fn_text.contains("page 42"), "footnote: {fn_text}");
    }

    #[test]
    fn build_citations_assigns_sequential_numbers() {
        let triples = &[("a.txt", 0.9, "chunk a"), ("b.txt", 0.8, "chunk b")];
        let cits = build_citations(triples);
        assert_eq!(cits[0].number, 1);
        assert_eq!(cits[1].number, 2);
    }

    #[test]
    fn build_citations_preserves_score() {
        let triples = &[("src", 0.75, "text")];
        let cits = build_citations(triples);
        assert!((cits[0].score - 0.75f32).abs() < 1e-5);
    }

    #[test]
    fn format_footnote_block_joins_with_newline() {
        let triples = &[("a", 0.9, "A"), ("b", 0.8, "B")];
        let cits = build_citations(triples);
        let block = format_footnote_block(&cits);
        assert!(block.contains('\n'), "block: {block}");
    }

    #[test]
    fn citations_to_json_returns_array() {
        let triples = &[("src", 0.5, "text")];
        let cits = build_citations(triples);
        let json = citations_to_json(&cits);
        assert!(json.as_array().is_some());
        assert_eq!(json.as_array().unwrap().len(), 1);
    }

    #[test]
    fn filter_by_score_removes_low_scores() {
        let triples = &[("a", 0.9, "A"), ("b", 0.3, "B"), ("c", 0.7, "C")];
        let cits = build_citations(triples);
        let filtered = filter_by_score(cits, 0.5);
        assert_eq!(filtered.len(), 2);
        assert!(filtered.iter().all(|c| c.score >= 0.5));
    }

    #[test]
    fn dedup_by_source_keeps_highest_score() {
        let cits = vec![
            CitationRecord::new(1, "dup.txt", "A", 0.6),
            CitationRecord::new(2, "dup.txt", "B", 0.9),
            CitationRecord::new(3, "other.txt", "C", 0.5),
        ];
        let deduped = dedup_by_source(cits);
        assert_eq!(deduped.len(), 2);
        let dup = deduped.iter().find(|c| c.source == "dup.txt").unwrap();
        assert!((dup.score - 0.9f32).abs() < 1e-5, "kept wrong score");
    }

    #[test]
    fn citation_to_json_has_required_fields() {
        let c = CitationRecord::new(1, "src", "text", 0.8).with_title("Title");
        let j = c.to_json();
        assert_eq!(j["number"], 1);
        assert_eq!(j["source"], "src");
        assert!((j["score"].as_f64().unwrap() - 0.8f64).abs() < 1e-4);
    }
}
