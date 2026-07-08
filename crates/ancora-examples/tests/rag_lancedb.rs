use ancora_examples::{keyword_retrieve, Passage};

fn corpus() -> Vec<Passage> {
    vec![
        Passage::new(
            "lancedb.md",
            "LanceDB stores vectors with column-level compression for efficient similarity search.",
        ),
        Passage::new(
            "ancora.md",
            "Ancora is a multi-agent runtime for AI applications.",
        ),
        Passage::new(
            "qdrant.md",
            "Qdrant is a vector database supporting filtering and payload-based search.",
        ),
    ]
}

#[test]
fn top_result_for_lancedb_query_is_lancedb_doc() {
    let c = corpus();
    let hits = keyword_retrieve(&c, "lancedb vector", 1);
    assert_eq!("lancedb.md", hits[0].key);
}

#[test]
fn retrieve_top2_returns_two_passages() {
    let c = corpus();
    let hits = keyword_retrieve(&c, "vector", 2);
    assert_eq!(2, hits.len());
}

#[test]
fn retrieve_all_returns_full_corpus() {
    let c = corpus();
    let hits = keyword_retrieve(&c, "ai", 100);
    assert_eq!(c.len(), hits.len());
}

#[test]
fn context_can_be_assembled_from_passages() {
    let c = corpus();
    let hits = keyword_retrieve(&c, "lancedb vector", 2);
    let context: String = hits
        .iter()
        .map(|p| p.content.as_str())
        .collect::<Vec<_>>()
        .join("\n");
    assert!(context.contains("LanceDB"));
}

#[test]
fn empty_query_still_returns_requested_count() {
    let c = corpus();
    let hits = keyword_retrieve(&c, "", 2);
    assert_eq!(2, hits.len());
}
