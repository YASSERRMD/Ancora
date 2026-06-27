//! Shared helpers used across the Ancora Rust example tests.
//!
//! These utilities are intentionally minimal and self-contained so each
//! example remains easy to read without diving into library internals.

use std::collections::HashMap;
use std::time::Instant;

// --------------------------------------------------------------------
// RunJournal -- in-memory event store for durable-restart examples
// --------------------------------------------------------------------

/// In-memory event store that mimics a durable restart journal.
#[derive(Default)]
pub struct RunJournal {
    store: HashMap<String, Vec<String>>,
}

impl RunJournal {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a run ID. Idempotent: calling twice keeps `run_count()` at 1.
    pub fn record_run(&mut self, run_id: &str) {
        self.store.entry(run_id.to_string()).or_default();
    }

    pub fn append_event(&mut self, run_id: &str, event_json: &str) {
        self.store
            .entry(run_id.to_string())
            .or_default()
            .push(event_json.to_string());
    }

    pub fn events_for_run(&self, run_id: &str) -> &[String] {
        self.store.get(run_id).map(|v| v.as_slice()).unwrap_or(&[])
    }

    pub fn run_count(&self) -> usize {
        self.store.len()
    }
}

// --------------------------------------------------------------------
// Span -- lightweight OTEL-style span for cost-trace examples
// --------------------------------------------------------------------

/// Lightweight in-process span mirroring what an OTEL exporter would consume.
pub struct Span {
    pub name: String,
    pub attributes: HashMap<String, String>,
    start: Instant,
    pub duration_ms: Option<u128>,
}

impl Span {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            attributes: HashMap::new(),
            start: Instant::now(),
            duration_ms: None,
        }
    }

    pub fn set_attribute(&mut self, key: &str, value: impl ToString) {
        self.attributes.insert(key.to_string(), value.to_string());
    }

    /// Mark the span as ended and return elapsed milliseconds.
    pub fn end_ms(&mut self) -> u128 {
        let ms = self.start.elapsed().as_millis();
        self.duration_ms = Some(ms);
        ms
    }
}

// --------------------------------------------------------------------
// TokenEstimator -- rough 4-chars-per-token estimate
// --------------------------------------------------------------------

pub struct TokenEstimator;

impl TokenEstimator {
    pub fn estimate_tokens(text: &str) -> usize {
        if text.is_empty() {
            return 1;
        }
        ((text.len() as f64) / 4.0).ceil() as usize
    }
}

// --------------------------------------------------------------------
// Keyword retrieval -- offline RAG stand-in
// --------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct Passage {
    pub key: String,
    pub content: String,
}

impl Passage {
    pub fn new(key: &str, content: &str) -> Self {
        Self { key: key.to_string(), content: content.to_string() }
    }
}

/// Rank passages by keyword overlap with `query`. Returns the top-`k` results.
pub fn keyword_retrieve<'a>(corpus: &'a [Passage], query: &str, top_k: usize) -> Vec<&'a Passage> {
    let terms: Vec<String> = query
        .to_lowercase()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();

    let mut scored: Vec<(&Passage, usize)> = corpus
        .iter()
        .map(|p| {
            let score = terms
                .iter()
                .filter(|t| p.content.to_lowercase().contains(t.as_str()))
                .count();
            (p, score)
        })
        .collect();

    scored.sort_by(|a, b| b.1.cmp(&a.1));
    scored.into_iter().take(top_k).map(|(p, _)| p).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_journal_record_and_replay() {
        let mut j = RunJournal::new();
        j.record_run("r1");
        j.append_event("r1", r#"{"kind":"started"}"#);
        assert_eq!(1, j.events_for_run("r1").len());
        assert_eq!(1, j.run_count());
    }

    #[test]
    fn token_estimator_four_chars_per_token() {
        assert_eq!(1, TokenEstimator::estimate_tokens(""));
        assert_eq!(1, TokenEstimator::estimate_tokens("abcd"));
        assert_eq!(2, TokenEstimator::estimate_tokens("abcde"));
        assert_eq!(25, TokenEstimator::estimate_tokens(&"x".repeat(100)));
    }

    #[test]
    fn keyword_retrieve_ranks_best_match_first() {
        let corpus = vec![
            Passage::new("a.md", "lancedb vector similarity search"),
            Passage::new("b.md", "ancora is a multi-agent runtime"),
        ];
        let hits = keyword_retrieve(&corpus, "lancedb vector", 1);
        assert_eq!("a.md", hits[0].key);
    }
}
