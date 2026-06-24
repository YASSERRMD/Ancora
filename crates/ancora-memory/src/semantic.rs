use std::sync::Mutex;

use crate::embed::{Embedding, EmbeddingProvider};
use crate::vector::{VectorEntry, VectorIndex};

/// Semantic memory store backed by a flat cosine-similarity vector index.
pub struct SemanticMemoryStore {
    index: Mutex<VectorIndex>,
    provider: Box<dyn EmbeddingProvider>,
}

impl SemanticMemoryStore {
    pub fn new(provider: Box<dyn EmbeddingProvider>) -> Self {
        Self { index: Mutex::new(VectorIndex::new()), provider }
    }

    /// Embed `text` and insert it into the vector index.
    pub fn insert(&self, text: impl Into<String>) {
        let text = text.into();
        let embedding = self.provider.embed(&text);
        self.index.lock().unwrap().insert(VectorEntry { text, embedding });
    }

    /// Return the top-`k` texts most semantically similar to `query`.
    pub fn search(&self, query: &str, k: usize) -> Vec<String> {
        let query_vec: Embedding = self.provider.embed(query);
        self.index.lock().unwrap()
            .search(&query_vec, k)
            .into_iter()
            .map(|e| e.text.clone())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::embed::HashEmbeddingProvider;

    #[test]
    fn semantic_recall_returns_nearest_items() {
        let store = SemanticMemoryStore::new(Box::new(HashEmbeddingProvider::new(64)));
        store.insert("apple");
        store.insert("banana");
        store.insert("cherry");
        let results = store.search("apple", 1);
        assert_eq!(results, vec!["apple"]);
    }

    #[test]
    fn semantic_recall_top_k_limits_results() {
        let store = SemanticMemoryStore::new(Box::new(HashEmbeddingProvider::new(64)));
        for i in 0..10 {
            store.insert(format!("item-{i}"));
        }
        let results = store.search("item-0", 3);
        assert_eq!(results.len(), 3);
        assert_eq!(results[0], "item-0");
    }
}
