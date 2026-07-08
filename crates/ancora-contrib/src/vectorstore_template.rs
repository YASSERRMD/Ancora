/// ancora-contrib: vector-store adapter template
///
/// Copy this module as the starting point for a new vector-store plugin.
/// Replace `MyVectorStore` and `"my-vectorstore"` with your own identifier,
/// then implement the four required trait methods.

/// A document fragment stored in the vector store.
#[derive(Debug, Clone, PartialEq)]
pub struct Document {
    pub id: String,
    pub content: String,
    /// Pre-computed embedding vector. May be empty before indexing.
    pub embedding: Vec<f32>,
    pub metadata: Vec<(String, String)>,
}

impl Document {
    pub fn new(id: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            content: content.into(),
            embedding: Vec::new(),
            metadata: Vec::new(),
        }
    }

    pub fn with_embedding(mut self, emb: Vec<f32>) -> Self {
        self.embedding = emb;
        self
    }

    pub fn with_meta(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.push((key.into(), value.into()));
        self
    }
}

/// A retrieved document with its similarity score.
#[derive(Debug, Clone)]
pub struct SearchResult {
    pub document: Document,
    /// Cosine similarity in [0, 1].
    pub score: f32,
}

/// Errors a vector-store adapter may return.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VectorStoreError {
    NotFound(String),
    DuplicateId(String),
    DimensionMismatch { expected: usize, got: usize },
    StorageError(String),
}

impl std::fmt::Display for VectorStoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VectorStoreError::NotFound(id) => write!(f, "document not found: {id}"),
            VectorStoreError::DuplicateId(id) => write!(f, "duplicate document id: {id}"),
            VectorStoreError::DimensionMismatch { expected, got } => {
                write!(f, "dimension mismatch: expected {expected}, got {got}")
            }
            VectorStoreError::StorageError(s) => write!(f, "storage error: {s}"),
        }
    }
}

impl std::error::Error for VectorStoreError {}

/// Trait all vector-store adapters must implement.
pub trait VectorStoreAdapter: Send + Sync {
    /// Stable identifier (e.g. "pinecone", "chroma").
    fn store_id(&self) -> &str;

    /// Upsert a document. Returns the stored document id.
    fn upsert(&mut self, doc: Document) -> Result<String, VectorStoreError>;

    /// Retrieve the top-`k` most similar documents to `query_embedding`.
    fn search(
        &self,
        query_embedding: &[f32],
        k: usize,
    ) -> Result<Vec<SearchResult>, VectorStoreError>;

    /// Delete a document by id.
    fn delete(&mut self, id: &str) -> Result<(), VectorStoreError>;
}

// ---------------------------------------------------------------------------
// Template implementation - rename and replace with your real store logic.
// ---------------------------------------------------------------------------

/// In-memory vector store using brute-force cosine similarity.
/// Use as a test harness; replace with a real store in production.
pub struct MyVectorStore {
    pub name: String,
    docs: Vec<Document>,
}

impl MyVectorStore {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            docs: Vec::new(),
        }
    }

    fn cosine(a: &[f32], b: &[f32]) -> f32 {
        if a.len() != b.len() || a.is_empty() {
            return 0.0;
        }
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

impl VectorStoreAdapter for MyVectorStore {
    fn store_id(&self) -> &str {
        // TODO: replace with your store's identifier.
        "my-vectorstore"
    }

    fn upsert(&mut self, doc: Document) -> Result<String, VectorStoreError> {
        let id = doc.id.clone();
        // Overwrite if already present.
        if let Some(pos) = self.docs.iter().position(|d| d.id == id) {
            self.docs[pos] = doc;
        } else {
            self.docs.push(doc);
        }
        Ok(id)
    }

    fn search(
        &self,
        query_embedding: &[f32],
        k: usize,
    ) -> Result<Vec<SearchResult>, VectorStoreError> {
        let mut scored: Vec<SearchResult> = self
            .docs
            .iter()
            .map(|d| SearchResult {
                score: Self::cosine(query_embedding, &d.embedding),
                document: d.clone(),
            })
            .collect();
        scored.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        scored.truncate(k);
        Ok(scored)
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
}
