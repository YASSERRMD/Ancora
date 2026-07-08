/// Conformance kit for vector-store extensions.

/// A vector document with an id, content, and embedding.
#[derive(Debug, Clone)]
pub struct VecDoc {
    pub id: String,
    pub content: String,
    pub embedding: Vec<f32>,
}

/// Trait that every vector-store extension must satisfy.
pub trait VectorStore {
    fn name(&self) -> &str;
    fn upsert(&mut self, doc: VecDoc) -> Result<(), String>;
    fn search(&self, query: &[f32], top_k: usize) -> Result<Vec<VecDoc>, String>;
}

/// A single conformance check result.
#[derive(Debug, Clone, PartialEq)]
pub struct CheckResult {
    pub name: String,
    pub passed: bool,
    pub message: String,
}

/// Kit that runs conformance checks against a [`VectorStore`].
pub struct VectorStoreKit;

impl VectorStoreKit {
    pub fn new() -> Self {
        VectorStoreKit
    }

    pub fn run<V: VectorStore>(&self, store: &mut V) -> Vec<CheckResult> {
        vec![self.check_name(store), self.check_upsert_and_search(store)]
    }

    fn check_name<V: VectorStore>(&self, store: &V) -> CheckResult {
        if store.name().is_empty() {
            CheckResult {
                name: "vectorstore_name_nonempty".into(),
                passed: false,
                message: "VectorStore name must not be empty".into(),
            }
        } else {
            CheckResult {
                name: "vectorstore_name_nonempty".into(),
                passed: true,
                message: format!("Store name: {}", store.name()),
            }
        }
    }

    fn check_upsert_and_search<V: VectorStore>(&self, store: &mut V) -> CheckResult {
        let doc = VecDoc {
            id: "itk-probe".into(),
            content: "hello world".into(),
            embedding: vec![0.1, 0.2, 0.3],
        };
        if let Err(e) = store.upsert(doc) {
            return CheckResult {
                name: "vectorstore_upsert_search".into(),
                passed: false,
                message: format!("upsert failed: {e}"),
            };
        }
        match store.search(&[0.1, 0.2, 0.3], 1) {
            Ok(results) if !results.is_empty() => CheckResult {
                name: "vectorstore_upsert_search".into(),
                passed: true,
                message: format!("search returned {} result(s)", results.len()),
            },
            Ok(_) => CheckResult {
                name: "vectorstore_upsert_search".into(),
                passed: false,
                message: "search returned 0 results after upsert".into(),
            },
            Err(e) => CheckResult {
                name: "vectorstore_upsert_search".into(),
                passed: false,
                message: format!("search failed: {e}"),
            },
        }
    }
}

impl Default for VectorStoreKit {
    fn default() -> Self {
        Self::new()
    }
}
