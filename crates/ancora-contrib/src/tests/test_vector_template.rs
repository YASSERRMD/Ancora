use crate::vectorstore_template::{Document, MyVectorStore, VectorStoreAdapter, VectorStoreError};

fn make_store() -> MyVectorStore {
    MyVectorStore::new("test-store")
}

fn doc(id: &str, emb: Vec<f32>) -> Document {
    Document::new(id, format!("content of {id}")).with_embedding(emb)
}

#[test]
fn store_id_is_correct() {
    let store = make_store();
    assert_eq!(store.store_id(), "my-vectorstore");
}

#[test]
fn upsert_and_search_basic() {
    let mut store = make_store();
    let d1 = doc("d1", vec![1.0, 0.0]);
    let d2 = doc("d2", vec![0.0, 1.0]);
    store.upsert(d1).expect("upsert d1");
    store.upsert(d2).expect("upsert d2");

    // Query aligned with d1
    let results = store.search(&[1.0, 0.0], 1).expect("search must succeed");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].document.id, "d1");
    assert!((results[0].score - 1.0).abs() < 1e-5);
}

#[test]
fn upsert_overwrites_existing_document() {
    let mut store = make_store();
    store.upsert(doc("d1", vec![1.0, 0.0])).unwrap();
    store.upsert(doc("d1", vec![0.0, 1.0])).unwrap();

    let results = store.search(&[0.0, 1.0], 1).unwrap();
    assert_eq!(results[0].document.id, "d1");
    // New embedding should dominate
    assert!((results[0].score - 1.0).abs() < 1e-5);
}

#[test]
fn delete_removes_document() {
    let mut store = make_store();
    store.upsert(doc("d1", vec![1.0, 0.0])).unwrap();
    store.delete("d1").expect("delete must succeed");
    let results = store.search(&[1.0, 0.0], 5).unwrap();
    assert!(results.iter().all(|r| r.document.id != "d1"));
}

#[test]
fn delete_nonexistent_returns_error() {
    let mut store = make_store();
    match store.delete("ghost") {
        Err(VectorStoreError::NotFound(id)) => assert_eq!(id, "ghost"),
        other => panic!("expected NotFound, got {other:?}"),
    }
}

#[test]
fn search_returns_at_most_k_results() {
    let mut store = make_store();
    for i in 0..10u32 {
        store
            .upsert(doc(&format!("d{i}"), vec![i as f32, 0.0]))
            .unwrap();
    }
    let results = store.search(&[1.0, 0.0], 3).unwrap();
    assert_eq!(results.len(), 3);
}

#[test]
fn search_empty_store_returns_empty() {
    let store = make_store();
    let results = store.search(&[1.0, 0.0], 5).unwrap();
    assert!(results.is_empty());
}

#[test]
fn document_metadata() {
    let d = Document::new("id1", "content")
        .with_embedding(vec![1.0])
        .with_meta("source", "test")
        .with_meta("lang", "en");
    assert_eq!(d.metadata.len(), 2);
    assert_eq!(d.metadata[0], ("source".to_string(), "test".to_string()));
}

#[test]
fn vector_store_error_display() {
    assert!(VectorStoreError::NotFound("x".into())
        .to_string()
        .contains("x"));
    assert!(VectorStoreError::DuplicateId("y".into())
        .to_string()
        .contains("y"));
    let e = VectorStoreError::DimensionMismatch {
        expected: 4,
        got: 3,
    };
    assert!(e.to_string().contains("4") && e.to_string().contains("3"));
}
