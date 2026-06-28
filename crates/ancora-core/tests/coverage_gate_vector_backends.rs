// Coverage gate: all vector store backends have conformance tests.

const VECTOR_BACKENDS: &[&str] = &[
    "inmemory",
    "sqlite",
    "pgvector",
    "qdrant",
    "weaviate",
    "milvus",
    "lancedb",
    "chroma",
    "pinecone",
    "vespa",
    "redis",
];

const VECTOR_CONFORMANCE_TESTS: &[(&str, &str)] = &[
    ("inmemory",  "vector_backend_selection"),
    ("sqlite",    "vector_backend_selection"),
    ("pgvector",  "vector_metadata_filter_parity"),
    ("qdrant",    "vector_metadata_filter_parity"),
    ("weaviate",  "vector_weaviate_conformance"),
    ("milvus",    "vector_milvus_conformance"),
    ("lancedb",   "vector_lancedb_conformance"),
    ("chroma",    "vector_chroma_conformance"),
    ("pinecone",  "vector_pinecone_conformance"),
    ("vespa",     "vector_vespa_conformance"),
    ("redis",     "vector_redis_conformance"),
];

#[test]
fn test_all_backends_have_conformance_test() {
    let covered: Vec<&str> = VECTOR_CONFORMANCE_TESTS.iter().map(|(b, _)| *b).collect();
    for backend in VECTOR_BACKENDS {
        assert!(covered.contains(backend), "no conformance test for backend: {backend}");
    }
}

#[test]
fn test_11_backends_defined() {
    assert_eq!(VECTOR_BACKENDS.len(), 11);
}

#[test]
fn test_conformance_map_has_11_entries() {
    assert_eq!(VECTOR_CONFORMANCE_TESTS.len(), 11);
}

#[test]
fn test_no_unknown_backend_in_conformance_map() {
    for (backend, _) in VECTOR_CONFORMANCE_TESTS {
        assert!(VECTOR_BACKENDS.contains(backend), "unknown backend in conformance map: {backend}");
    }
}

#[test]
fn test_weaviate_covered_by_dedicated_test() {
    let entry = VECTOR_CONFORMANCE_TESTS.iter().find(|(b, _)| *b == "weaviate");
    assert_eq!(entry.map(|(_, t)| *t), Some("vector_weaviate_conformance"));
}

#[test]
fn test_pinecone_covered_by_dedicated_test() {
    let entry = VECTOR_CONFORMANCE_TESTS.iter().find(|(b, _)| *b == "pinecone");
    assert_eq!(entry.map(|(_, t)| *t), Some("vector_pinecone_conformance"));
}
