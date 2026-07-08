/// Vector-store extension point - provide a vector similarity search back-end.

/// A dense embedding vector.
pub type Embedding = Vec<f32>;

/// A stored document with its embedding.
#[derive(Debug, Clone)]
pub struct Document {
    pub id: String,
    pub content: String,
    pub embedding: Embedding,
    pub metadata: std::collections::HashMap<String, String>,
}

/// Parameters for a similarity query.
#[derive(Debug, Clone)]
pub struct QueryRequest {
    pub embedding: Embedding,
    pub top_k: usize,
    /// Optional namespace / collection to search within.
    pub namespace: Option<String>,
}

/// A single result from a similarity query.
#[derive(Debug, Clone)]
pub struct QueryResult {
    pub document: Document,
    /// Cosine similarity score in [-1.0, 1.0].
    pub score: f32,
}

/// Errors from the vector store.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VectorStoreError {
    NotFound(String),
    DimensionMismatch { expected: usize, got: usize },
    StorageFull,
    Unknown(String),
}

impl std::fmt::Display for VectorStoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VectorStoreError::NotFound(id) => write!(f, "document not found: {id}"),
            VectorStoreError::DimensionMismatch { expected, got } => {
                write!(f, "dimension mismatch: expected {expected}, got {got}")
            }
            VectorStoreError::StorageFull => write!(f, "storage is full"),
            VectorStoreError::Unknown(s) => write!(f, "unknown error: {s}"),
        }
    }
}

impl std::error::Error for VectorStoreError {}

/// Trait that vector-store plugins must implement.
pub trait VectorStorePlugin: Send + Sync {
    fn store_id(&self) -> &str;

    /// Insert or replace a document.
    fn upsert(&mut self, doc: Document) -> Result<(), VectorStoreError>;

    /// Delete a document by id.
    fn delete(&mut self, id: &str) -> Result<(), VectorStoreError>;

    /// Return the top-k most similar documents.
    fn query(&self, req: QueryRequest) -> Result<Vec<QueryResult>, VectorStoreError>;

    /// Current number of documents stored.
    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Simple in-memory vector store for testing.
pub struct InMemoryVectorStore {
    id: String,
    docs: Vec<Document>,
}

impl InMemoryVectorStore {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            docs: Vec::new(),
        }
    }

    fn cosine(a: &[f32], b: &[f32]) -> f32 {
        let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
        let na: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
        let nb: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
        if na == 0.0 || nb == 0.0 {
            0.0
        } else {
            dot / (na * nb)
        }
    }
}

impl VectorStorePlugin for InMemoryVectorStore {
    fn store_id(&self) -> &str {
        &self.id
    }

    fn upsert(&mut self, doc: Document) -> Result<(), VectorStoreError> {
        if let Some(existing) = self.docs.iter_mut().find(|d| d.id == doc.id) {
            *existing = doc;
        } else {
            self.docs.push(doc);
        }
        Ok(())
    }

    fn delete(&mut self, id: &str) -> Result<(), VectorStoreError> {
        let before = self.docs.len();
        self.docs.retain(|d| d.id != id);
        if self.docs.len() == before {
            Err(VectorStoreError::NotFound(id.to_string()))
        } else {
            Ok(())
        }
    }

    fn query(&self, req: QueryRequest) -> Result<Vec<QueryResult>, VectorStoreError> {
        let mut scored: Vec<QueryResult> = self
            .docs
            .iter()
            .filter(|d| {
                req.namespace
                    .as_ref()
                    .map(|ns| {
                        d.metadata
                            .get("namespace")
                            .map(|v| v == ns)
                            .unwrap_or(false)
                    })
                    .unwrap_or(true)
            })
            .map(|d| {
                let score = Self::cosine(&req.embedding, &d.embedding);
                QueryResult {
                    document: d.clone(),
                    score,
                }
            })
            .collect();
        scored.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        scored.truncate(req.top_k);
        Ok(scored)
    }

    fn len(&self) -> usize {
        self.docs.len()
    }
}
