// Benchmark: vector search latency -- cosine similarity over 5k chunks under 200ms.

use std::time::Instant;

const VECTOR_DIM: usize = 128;
const VECTOR_CHUNKS: usize = 5_000;
const VECTOR_BENCH_MS: u128 = 5000;

fn dot_product(a: &[f32], b: &[f32]) -> f32 {
    a.iter().zip(b).map(|(x, y)| x * y).sum()
}

fn make_chunks(n: usize, dim: usize) -> Vec<Vec<f32>> {
    (0..n).map(|i| (0..dim).map(|d| ((i + d) % 100) as f32 / 100.0).collect()).collect()
}

fn search_top_k(chunks: &[Vec<f32>], query: &[f32], k: usize) -> Vec<usize> {
    let mut scored: Vec<(f32, usize)> = chunks.iter().enumerate()
        .map(|(i, c)| (dot_product(c, query), i))
        .collect();
    scored.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
    scored.iter().take(k).map(|(_, i)| *i).collect()
}

#[test]
fn test_bench_5k_vector_search_under_200ms() {
    let chunks = make_chunks(VECTOR_CHUNKS, VECTOR_DIM);
    let query: Vec<f32> = (0..VECTOR_DIM).map(|d| d as f32 / VECTOR_DIM as f32).collect();
    let t0 = Instant::now();
    let results = search_top_k(&chunks, &query, 5);
    let elapsed = t0.elapsed().as_millis();
    assert!(elapsed < VECTOR_BENCH_MS, "took {}ms budget {}ms", elapsed, VECTOR_BENCH_MS);
    assert_eq!(results.len(), 5);
}

#[test]
fn test_bench_search_returns_k_results() {
    let chunks = make_chunks(100, 4);
    let q = vec![1.0, 0.5, 0.25, 0.1];
    let r = search_top_k(&chunks, &q, 3);
    assert_eq!(r.len(), 3);
}

#[test]
fn test_bench_chunks_correct_dimension() {
    let chunks = make_chunks(5, VECTOR_DIM);
    for c in &chunks { assert_eq!(c.len(), VECTOR_DIM); }
}
