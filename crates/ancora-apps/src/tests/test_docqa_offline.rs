use crate::document_qa::{Document, DocumentQaEngine, DocumentStore};

#[test]
fn docqa_returns_answer_without_network() {
    let mut store = DocumentStore::new();
    store.ingest(Document::new(
        "policy-001",
        "Data Retention Policy",
        "All records must be retained for a minimum of 7 years. Retention applies to digital and physical records.",
    ));
    store.ingest(Document::new(
        "policy-002",
        "Security Policy",
        "Access to sensitive systems requires multi-factor authentication. All sessions are logged.",
    ));

    let engine = DocumentQaEngine::new(store);

    let answer = engine.ask("retention");
    assert!(!answer.source_ids.is_empty(), "should find relevant documents");
    assert!(answer.source_ids.contains(&"policy-001".to_string()));
    assert!(!answer.excerpts.is_empty(), "should extract excerpts");
}

#[test]
fn docqa_returns_empty_for_unknown_query() {
    let mut store = DocumentStore::new();
    store.ingest(Document::new("d1", "Title", "Some content about apples."));
    let engine = DocumentQaEngine::new(store);
    let answer = engine.ask("xylophone");
    assert!(answer.source_ids.is_empty());
}

#[test]
fn docqa_handles_empty_store() {
    let store = DocumentStore::new();
    let engine = DocumentQaEngine::new(store);
    let answer = engine.ask("anything");
    assert!(answer.source_ids.is_empty());
    assert!(answer.excerpts.is_empty());
}
