use crate::vectorstore_kit::{VecDoc, VectorStore, VectorStoreKit};

struct MemoryVectorStore {
    docs: Vec<VecDoc>,
}

impl MemoryVectorStore {
    fn new() -> Self {
        MemoryVectorStore { docs: Vec::new() }
    }
}

impl VectorStore for MemoryVectorStore {
    fn name(&self) -> &str {
        "memory-store"
    }

    fn upsert(&mut self, doc: VecDoc) -> Result<(), String> {
        self.docs.retain(|d| d.id != doc.id);
        self.docs.push(doc);
        Ok(())
    }

    fn search(&self, _query: &[f32], top_k: usize) -> Result<Vec<VecDoc>, String> {
        Ok(self.docs.iter().take(top_k).cloned().collect())
    }
}

#[test]
fn vector_kit_passes_for_memory_store() {
    let kit = VectorStoreKit::new();
    let mut store = MemoryVectorStore::new();
    let results = kit.run(&mut store);
    for r in &results {
        assert!(r.passed, "Check failed: {} - {}", r.name, r.message);
    }
    assert_eq!(results.len(), 2);
}

#[test]
fn memory_store_upsert_replaces_existing() {
    let mut store = MemoryVectorStore::new();
    store
        .upsert(VecDoc {
            id: "a".into(),
            content: "first".into(),
            embedding: vec![1.0],
        })
        .unwrap();
    store
        .upsert(VecDoc {
            id: "a".into(),
            content: "second".into(),
            embedding: vec![1.0],
        })
        .unwrap();
    let results = store.search(&[1.0], 10).unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].content, "second");
}
