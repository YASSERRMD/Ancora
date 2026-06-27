/// Full retrieval pipeline: embed, chunk, store, query.
///
/// `RetrievalPipeline` wires together:
///   1. A `Chunker` that splits documents into passages.
///   2. An `Embedder` that converts each passage to a dense vector.
///   3. An in-memory vector store (`Vec<(text, embedding)>`).
///   4. A query path that embeds the query and returns top-k passages.
///
/// Everything is offline; no network calls, no external processes.

use std::sync::Arc;
use crate::embedders::embedder::{Embedding, EmbedResult, Embedder, cosine_similarity};
use crate::embedders::chunker::FixedSizeChunker;
use crate::embedders::rerank::ScoredPassage;
use crate::embedders::citation::CitationRecord;

// ---- pipeline config ---------------------------------------------------

#[derive(Debug, Clone)]
pub struct PipelineConfig {
    pub chunk_size: usize,
    pub chunk_overlap: usize,
    pub top_k: usize,
    pub min_score: f32,
}

impl PipelineConfig {
    pub fn new(chunk_size: usize, chunk_overlap: usize, top_k: usize) -> Self {
        Self { chunk_size, chunk_overlap, top_k, min_score: 0.0 }
    }

    pub fn with_min_score(mut self, s: f32) -> Self { self.min_score = s; self }
}

impl Default for PipelineConfig {
    fn default() -> Self { Self::new(256, 32, 5) }
}

// ---- in-memory vector store entry --------------------------------------

#[derive(Debug, Clone)]
struct StoreEntry {
    source: String,
    text: String,
    embedding: Embedding,
}

// ---- retrieval pipeline ------------------------------------------------

pub struct RetrievalPipeline {
    embedder: Arc<dyn Embedder>,
    config: PipelineConfig,
    store: Vec<StoreEntry>,
}

impl RetrievalPipeline {
    pub fn new(embedder: Arc<dyn Embedder>, config: PipelineConfig) -> Self {
        Self { embedder, config, store: Vec::new() }
    }

    /// Ingest a document: chunk it and embed each chunk.
    pub fn ingest(&mut self, source: &str, text: &str) -> EmbedResult<usize> {
        let chunker = FixedSizeChunker::new(self.config.chunk_size, self.config.chunk_overlap);
        let chunks = chunker.chunk(text);
        let count = chunks.len();
        for chunk in chunks {
            let embedding = self.embedder.embed(&chunk)?;
            self.store.push(StoreEntry {
                source: source.to_owned(),
                text: chunk,
                embedding,
            });
        }
        Ok(count)
    }

    /// Return the number of passages stored.
    pub fn passage_count(&self) -> usize { self.store.len() }

    /// Query the store and return top-k `ScoredPassage`s.
    pub fn query(&self, query_text: &str) -> EmbedResult<Vec<ScoredPassage>> {
        let q_emb = self.embedder.embed(query_text)?;
        let mut scored: Vec<(usize, f32)> = self.store.iter().enumerate()
            .map(|(i, entry)| {
                let score = cosine_similarity(&q_emb, &entry.embedding);
                (i, score)
            })
            .filter(|(_, s)| *s >= self.config.min_score)
            .collect();
        scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        scored.truncate(self.config.top_k);
        Ok(scored.into_iter().map(|(i, score)| {
            ScoredPassage::new(i, self.store[i].text.clone(), score)
        }).collect())
    }

    /// Query and also return citation records with source metadata.
    pub fn query_with_citations(&self, query_text: &str) -> EmbedResult<Vec<CitationRecord>> {
        let q_emb = self.embedder.embed(query_text)?;
        let mut scored: Vec<(usize, f32)> = self.store.iter().enumerate()
            .map(|(i, entry)| (i, cosine_similarity(&q_emb, &entry.embedding)))
            .filter(|(_, s)| *s >= self.config.min_score)
            .collect();
        scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        scored.truncate(self.config.top_k);
        Ok(scored.into_iter().enumerate().map(|(num, (i, score))| {
            CitationRecord::new(num + 1, self.store[i].source.clone(), self.store[i].text.clone(), score)
        }).collect())
    }

    /// Remove all stored passages.
    pub fn clear(&mut self) { self.store.clear(); }
}

// ---- tests ---------------------------------------------------------------

#[cfg(test)]
mod pipeline_tests {
    use super::*;
    use crate::embedders::local::HashEmbedder;

    fn make_pipeline(top_k: usize) -> RetrievalPipeline {
        let embedder = Arc::new(HashEmbedder::new(64));
        let config = PipelineConfig::new(10, 2, top_k);
        RetrievalPipeline::new(embedder, config)
    }

    #[test]
    fn pipeline_ingest_adds_chunks() {
        let mut p = make_pipeline(5);
        let count = p.ingest("doc.txt", "The quick brown fox jumps over the lazy dog").unwrap();
        assert!(count >= 1, "expected at least one chunk");
        assert!(p.passage_count() >= 1);
    }

    #[test]
    fn pipeline_ingest_multiple_docs() {
        let mut p = make_pipeline(5);
        p.ingest("a.txt", "Document one content here").unwrap();
        p.ingest("b.txt", "Document two content here").unwrap();
        assert!(p.passage_count() >= 2);
    }

    #[test]
    fn pipeline_query_returns_top_k() {
        let mut p = make_pipeline(3);
        p.ingest("doc.txt", "word0 word1 word2 word3 word4 word5 word6 word7 word8 word9 word10").unwrap();
        let results = p.query("word0 word1").unwrap();
        assert!(results.len() <= 3, "results: {}", results.len());
    }

    #[test]
    fn pipeline_query_returns_scored_passages() {
        let mut p = make_pipeline(5);
        p.ingest("doc.txt", "alpha beta gamma delta epsilon zeta").unwrap();
        let results = p.query("alpha beta").unwrap();
        assert!(!results.is_empty());
        for r in &results {
            assert!(r.score >= 0.0 && r.score <= 1.0 + 1e-4, "score out of range: {}", r.score);
        }
    }

    #[test]
    fn pipeline_clear_removes_all_passages() {
        let mut p = make_pipeline(5);
        p.ingest("doc.txt", "some content").unwrap();
        p.clear();
        assert_eq!(p.passage_count(), 0);
    }

    #[test]
    fn pipeline_query_with_citations_returns_citations() {
        let mut p = make_pipeline(5);
        p.ingest("src.txt", "the retrieval pipeline fetches relevant passages from the store").unwrap();
        let cits = p.query_with_citations("retrieval pipeline").unwrap();
        assert!(!cits.is_empty());
        assert_eq!(cits[0].number, 1);
        assert!(!cits[0].source.is_empty());
    }

    #[test]
    fn pipeline_empty_store_query_returns_empty() {
        let p = make_pipeline(5);
        let results = p.query("anything").unwrap();
        assert!(results.is_empty());
    }

    #[test]
    fn pipeline_min_score_filters_low_similarity() {
        let embedder = Arc::new(HashEmbedder::new(64));
        let config = PipelineConfig::new(5, 0, 10).with_min_score(0.99);
        let mut p = RetrievalPipeline::new(embedder, config);
        p.ingest("doc.txt", "abc def ghi jkl mno pqr").unwrap();
        // With min_score=0.99 almost nothing matches unless it's the same chunk.
        let results = p.query("xyz uvw").unwrap();
        for r in &results {
            assert!(r.score >= 0.99, "score below threshold: {}", r.score);
        }
    }
}
