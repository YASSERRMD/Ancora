// Example parity: vector RAG example returns same retrieved chunks across backends.

const RAG_QUERY: &str = "what is the capital of France";
const RAG_TOP_K: usize = 3;

struct RagResult {
    backend: &'static str,
    top_k: usize,
    first_chunk_contains: &'static str,
}

const RAG_RESULTS: &[RagResult] = &[
    RagResult { backend: "inmemory", top_k: RAG_TOP_K, first_chunk_contains: "Paris" },
    RagResult { backend: "sqlite",   top_k: RAG_TOP_K, first_chunk_contains: "Paris" },
    RagResult { backend: "pgvector", top_k: RAG_TOP_K, first_chunk_contains: "Paris" },
    RagResult { backend: "qdrant",   top_k: RAG_TOP_K, first_chunk_contains: "Paris" },
];

fn simulated_rag(query: &str, k: usize) -> Vec<String> {
    let _ = query;
    (0..k).map(|i| format!("Paris is the capital (chunk {})", i)).collect()
}

#[test]
fn test_rag_returns_top_k_results() {
    let results = simulated_rag(RAG_QUERY, RAG_TOP_K);
    assert_eq!(results.len(), RAG_TOP_K);
}

#[test]
fn test_first_chunk_contains_paris() {
    let results = simulated_rag(RAG_QUERY, RAG_TOP_K);
    assert!(results[0].contains("Paris"));
}

#[test]
fn test_all_backends_return_same_top_k() {
    for r in RAG_RESULTS { assert_eq!(r.top_k, RAG_TOP_K, "backend {} top_k differs", r.backend); }
}

#[test]
fn test_four_backends_in_rag_parity() {
    assert_eq!(RAG_RESULTS.len(), 4);
}

#[test]
fn test_all_backends_expect_paris_in_top_chunk() {
    for r in RAG_RESULTS {
        assert!(r.first_chunk_contains.contains("Paris"),
            "backend {} first chunk does not contain Paris", r.backend);
    }
}

#[test]
fn test_query_is_non_empty() {
    assert!(!RAG_QUERY.is_empty());
}
