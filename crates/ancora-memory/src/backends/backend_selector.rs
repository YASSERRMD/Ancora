//! Backend selector: maps a config string to a backend descriptor.
//!
//! Callers pass a short name (e.g. `"qdrant"`, `"pgvector"`) and receive a
//! `BackendInfo` describing the backend's requirements, so application code
//! can validate that the right Cargo feature is enabled at startup.

// ---- backend kind -------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub enum BackendKind {
    PgVector,
    Qdrant,
    Weaviate,
    Milvus,
    LanceDb,
    Chroma,
    Pinecone,
    Vespa,
    RedisVector,
}

impl BackendKind {
    /// The canonical feature flag string required in `Cargo.toml`.
    pub fn feature_flag(&self) -> &'static str {
        match self {
            Self::PgVector => "pgvector",
            Self::Qdrant => "qdrant",
            Self::Weaviate => "weaviate",
            Self::Milvus => "milvus",
            Self::LanceDb => "lancedb",
            Self::Chroma => "chroma",
            Self::Pinecone => "pinecone",
            Self::Vespa => "vespa",
            Self::RedisVector => "redis-vector",
        }
    }

    /// Whether this backend requires a network connection to a remote server.
    pub fn is_embedded(&self) -> bool {
        matches!(self, Self::LanceDb)
    }

    /// Whether this backend is a managed cloud service.
    pub fn is_managed_cloud(&self) -> bool {
        matches!(self, Self::Pinecone)
    }
}

// ---- backend info -------------------------------------------------------

#[derive(Debug, Clone)]
pub struct BackendInfo {
    pub kind: BackendKind,
    /// Human-readable display name.
    pub display_name: &'static str,
    /// Default port used by the backend in a local development setup.
    pub default_port: Option<u16>,
    /// Short description for help text.
    pub description: &'static str,
}

impl BackendInfo {
    fn new(
        kind: BackendKind,
        display_name: &'static str,
        default_port: Option<u16>,
        description: &'static str,
    ) -> Self {
        Self {
            kind,
            display_name,
            default_port,
            description,
        }
    }
}

// ---- selector -----------------------------------------------------------

/// Return `BackendInfo` for the given short name, or `None` if unrecognised.
///
/// Accepted names are case-insensitive and also match common aliases
/// (e.g. `"pg"` for pgvector, `"redis"` for redis-vector).
pub fn select_backend(name: &str) -> Option<BackendInfo> {
    match name.to_lowercase().as_str() {
        "pgvector" | "pg" | "postgres" | "postgresql" => Some(BackendInfo::new(
            BackendKind::PgVector,
            "pgvector (PostgreSQL)",
            Some(5432),
            "PostgreSQL extension for vector similarity search",
        )),
        "qdrant" => Some(BackendInfo::new(
            BackendKind::Qdrant,
            "Qdrant",
            Some(6333),
            "High-performance vector database with rich filtering",
        )),
        "weaviate" => Some(BackendInfo::new(
            BackendKind::Weaviate,
            "Weaviate",
            Some(8080),
            "Vector database with graph-linked data and generative modules",
        )),
        "milvus" => Some(BackendInfo::new(
            BackendKind::Milvus,
            "Milvus",
            Some(19530),
            "Distributed vector database for large-scale similarity search",
        )),
        "lancedb" | "lance" => Some(BackendInfo::new(
            BackendKind::LanceDb,
            "LanceDB",
            None,
            "Embedded columnar vector store; also supports S3/GCS/Azure",
        )),
        "chroma" | "chromadb" => Some(BackendInfo::new(
            BackendKind::Chroma,
            "Chroma",
            Some(8000),
            "Open-source embedding database with metadata filtering",
        )),
        "pinecone" => Some(BackendInfo::new(
            BackendKind::Pinecone,
            "Pinecone",
            None,
            "Managed cloud vector database with serverless and pod tiers",
        )),
        "vespa" => Some(BackendInfo::new(
            BackendKind::Vespa,
            "Vespa",
            Some(8080),
            "Search and recommendation platform with ANN and BM25 hybrid",
        )),
        "redis-vector" | "redis" | "redisearch" | "redis_vector" => Some(BackendInfo::new(
            BackendKind::RedisVector,
            "Redis Vector (RediSearch)",
            Some(6379),
            "Redis Stack vector search via HNSW or FLAT index",
        )),
        _ => None,
    }
}

/// List all known backend short names (canonical only).
pub fn known_backends() -> &'static [&'static str] {
    &[
        "pgvector",
        "qdrant",
        "weaviate",
        "milvus",
        "lancedb",
        "chroma",
        "pinecone",
        "vespa",
        "redis-vector",
    ]
}

// ---- tests --------------------------------------------------------------

#[cfg(test)]
mod backend_selector_tests {
    use super::*;

    #[test]
    fn pgvector_resolved_by_alias_pg() {
        let info = select_backend("pg").unwrap();
        assert_eq!(info.kind, BackendKind::PgVector);
    }

    #[test]
    fn pgvector_feature_flag_is_pgvector() {
        let info = select_backend("pgvector").unwrap();
        assert_eq!(info.kind.feature_flag(), "pgvector");
    }

    #[test]
    fn qdrant_resolved() {
        let info = select_backend("qdrant").unwrap();
        assert_eq!(info.kind, BackendKind::Qdrant);
        assert_eq!(info.default_port, Some(6333));
    }

    #[test]
    fn weaviate_resolved() {
        let info = select_backend("weaviate").unwrap();
        assert_eq!(info.kind, BackendKind::Weaviate);
        assert_eq!(info.default_port, Some(8080));
    }

    #[test]
    fn milvus_resolved() {
        let info = select_backend("milvus").unwrap();
        assert_eq!(info.kind, BackendKind::Milvus);
        assert_eq!(info.default_port, Some(19530));
    }

    #[test]
    fn lancedb_resolved_by_alias_lance() {
        let info = select_backend("lance").unwrap();
        assert_eq!(info.kind, BackendKind::LanceDb);
        assert!(info.kind.is_embedded());
    }

    #[test]
    fn lancedb_has_no_default_port() {
        let info = select_backend("lancedb").unwrap();
        assert!(info.default_port.is_none());
    }

    #[test]
    fn chroma_resolved_by_chromadb_alias() {
        let info = select_backend("chromadb").unwrap();
        assert_eq!(info.kind, BackendKind::Chroma);
        assert_eq!(info.default_port, Some(8000));
    }

    #[test]
    fn pinecone_is_managed_cloud() {
        let info = select_backend("pinecone").unwrap();
        assert!(info.kind.is_managed_cloud());
    }

    #[test]
    fn vespa_resolved() {
        let info = select_backend("vespa").unwrap();
        assert_eq!(info.kind, BackendKind::Vespa);
        assert_eq!(info.kind.feature_flag(), "vespa");
    }

    #[test]
    fn redis_vector_resolved_by_redis_alias() {
        let info = select_backend("redis").unwrap();
        assert_eq!(info.kind, BackendKind::RedisVector);
        assert_eq!(info.default_port, Some(6379));
    }

    #[test]
    fn redis_vector_feature_flag() {
        let info = select_backend("redis-vector").unwrap();
        assert_eq!(info.kind.feature_flag(), "redis-vector");
    }

    #[test]
    fn unknown_name_returns_none() {
        assert!(select_backend("cassandra").is_none());
        assert!(select_backend("").is_none());
    }

    #[test]
    fn case_insensitive_lookup() {
        assert!(select_backend("QDRANT").is_some());
        assert!(select_backend("Weaviate").is_some());
        assert!(select_backend("LANCEDB").is_some());
    }

    #[test]
    fn known_backends_has_nine_entries() {
        assert_eq!(known_backends().len(), 9);
    }

    #[test]
    fn all_known_backends_resolve() {
        for name in known_backends() {
            assert!(select_backend(name).is_some(), "failed to resolve: {name}");
        }
    }

    #[test]
    fn embedded_only_lancedb() {
        let embedded: Vec<_> = known_backends()
            .iter()
            .filter_map(|n| select_backend(n))
            .filter(|i| i.kind.is_embedded())
            .collect();
        assert_eq!(embedded.len(), 1);
        assert_eq!(embedded[0].kind, BackendKind::LanceDb);
    }
}
