use crate::metadata::Metadata;

/// The storage backend used by a vector store.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VectorStoreBackend {
    LanceDb,
    Qdrant,
    Pinecone,
    InMemory,
    Custom(String),
}

/// A catalog entry describing an installable vector store.
#[derive(Debug, Clone)]
pub struct VectorStoreEntry {
    pub id: String,
    pub name: String,
    pub description: String,
    pub backend: VectorStoreBackend,
    /// Default embedding dimension, if known.
    pub embedding_dim: Option<usize>,
    /// Connection URI template, e.g. "lancedb://./data".
    pub uri_template: Option<String>,
    pub metadata: Metadata,
}

impl VectorStoreEntry {
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        description: impl Into<String>,
        backend: VectorStoreBackend,
        metadata: Metadata,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            description: description.into(),
            backend,
            embedding_dim: None,
            uri_template: None,
            metadata,
        }
    }

    pub fn with_embedding_dim(mut self, dim: usize) -> Self {
        self.embedding_dim = Some(dim);
        self
    }

    pub fn with_uri_template(mut self, uri: impl Into<String>) -> Self {
        self.uri_template = Some(uri.into());
        self
    }

    pub fn is_valid(&self) -> bool {
        !self.id.is_empty() && !self.name.is_empty()
    }
}
