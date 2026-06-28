// Load: in-memory vector store -- insert and search 1k chunks within budget.

use std::time::Instant;

const STORE_N: usize = 1_000;
const STORE_BUDGET_MS: u128 = 500;

struct InMemChunk {
    id: String,
    text: String,
    embedding: Vec<f32>,
}

struct InMemStore {
    chunks: Vec<InMemChunk>,
}

impl InMemStore {
    fn new() -> Self { Self { chunks: Vec::new() } }

    fn insert(&mut self, id: String, text: String, embedding: Vec<f32>) {
        self.chunks.push(InMemChunk { id, text, embedding });
    }

    fn search_cosine(&self, query: &[f32], top_k: usize) -> Vec<&str> {
        let mut scored: Vec<(f32, &InMemChunk)> = self.chunks.iter().map(|c| {
            let dot: f32 = c.embedding.iter().zip(query).map(|(a, b)| a * b).sum();
            let na: f32 = c.embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
            let nb: f32 = query.iter().map(|x| x * x).sum::<f32>().sqrt();
            let score = if na * nb == 0.0 { 0.0 } else { dot / (na * nb) };
            (score, c)
        }).collect();
        scored.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
        scored.iter().take(top_k).map(|(_, c)| c.text.as_str()).collect()
    }
}

fn build_store(n: usize) -> InMemStore {
    let mut s = InMemStore::new();
    for i in 0..n {
        let emb = vec![i as f32, (i + 1) as f32, 1.0];
        s.insert(format!("id-{i}"), format!("chunk {i}"), emb);
    }
    s
}

#[test]
fn test_insert_and_search_1k_within_budget() {
    let t0 = Instant::now();
    let store = build_store(STORE_N);
    let q = vec![500.0, 501.0, 1.0];
    let results = store.search_cosine(&q, 5);
    let elapsed = t0.elapsed().as_millis();
    assert!(elapsed < STORE_BUDGET_MS, "took {}ms budget {}ms", elapsed, STORE_BUDGET_MS);
    assert!(!results.is_empty());
}

#[test]
fn test_store_has_correct_chunk_count() {
    let s = build_store(STORE_N);
    assert_eq!(s.chunks.len(), STORE_N);
}

#[test]
fn test_search_returns_top_k_results() {
    let s = build_store(20);
    let q = vec![1.0, 1.0, 1.0];
    let r = s.search_cosine(&q, 3);
    assert_eq!(r.len(), 3);
}

#[test]
fn test_exact_match_is_top_result() {
    let mut s = InMemStore::new();
    s.insert("a".to_string(), "target".to_string(), vec![1.0, 0.0, 0.0]);
    s.insert("b".to_string(), "other".to_string(), vec![0.0, 1.0, 0.0]);
    let r = s.search_cosine(&[1.0, 0.0, 0.0], 1);
    assert_eq!(r[0], "target");
}

#[test]
fn test_search_on_empty_store_returns_empty() {
    let s = InMemStore::new();
    let r = s.search_cosine(&[1.0, 0.0], 5);
    assert!(r.is_empty());
}
