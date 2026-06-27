/// Extended tests for the backend selector covering edge cases and cross-feature
/// predicates that are not already covered inside backend_selector.rs.

#[cfg(test)]
mod backend_selector_ext_tests {
    use crate::backends::backend_selector::*;

    // ---- alias coverage -------------------------------------------------

    #[test]
    fn postgres_alias_maps_to_pgvector() {
        let info = select_backend("postgres").unwrap();
        assert_eq!(info.kind, BackendKind::PgVector);
    }

    #[test]
    fn postgresql_alias_maps_to_pgvector() {
        let info = select_backend("postgresql").unwrap();
        assert_eq!(info.kind, BackendKind::PgVector);
    }

    #[test]
    fn redisearch_alias_maps_to_redis_vector() {
        let info = select_backend("redisearch").unwrap();
        assert_eq!(info.kind, BackendKind::RedisVector);
    }

    #[test]
    fn redis_underscore_vector_alias_maps_to_redis_vector() {
        let info = select_backend("redis_vector").unwrap();
        assert_eq!(info.kind, BackendKind::RedisVector);
    }

    #[test]
    fn chromadb_alias_maps_to_chroma() {
        let info = select_backend("chromadb").unwrap();
        assert_eq!(info.kind, BackendKind::Chroma);
    }

    // ---- feature_flag coverage ------------------------------------------

    #[test]
    fn qdrant_feature_flag() {
        let info = select_backend("qdrant").unwrap();
        assert_eq!(info.kind.feature_flag(), "qdrant");
    }

    #[test]
    fn weaviate_feature_flag() {
        let info = select_backend("weaviate").unwrap();
        assert_eq!(info.kind.feature_flag(), "weaviate");
    }

    #[test]
    fn milvus_feature_flag() {
        let info = select_backend("milvus").unwrap();
        assert_eq!(info.kind.feature_flag(), "milvus");
    }

    #[test]
    fn lancedb_feature_flag() {
        let info = select_backend("lancedb").unwrap();
        assert_eq!(info.kind.feature_flag(), "lancedb");
    }

    #[test]
    fn chroma_feature_flag() {
        let info = select_backend("chroma").unwrap();
        assert_eq!(info.kind.feature_flag(), "chroma");
    }

    #[test]
    fn pinecone_feature_flag() {
        let info = select_backend("pinecone").unwrap();
        assert_eq!(info.kind.feature_flag(), "pinecone");
    }

    // ---- is_embedded / is_managed_cloud ---------------------------------

    #[test]
    fn no_non_lancedb_backend_is_embedded() {
        for name in known_backends().iter().filter(|&&n| n != "lancedb") {
            let info = select_backend(name).unwrap();
            assert!(
                !info.kind.is_embedded(),
                "{name} should not be embedded"
            );
        }
    }

    #[test]
    fn no_non_pinecone_backend_is_managed_cloud() {
        for name in known_backends().iter().filter(|&&n| n != "pinecone") {
            let info = select_backend(name).unwrap();
            assert!(
                !info.kind.is_managed_cloud(),
                "{name} should not be managed cloud"
            );
        }
    }

    // ---- default_port checks --------------------------------------------

    #[test]
    fn server_backed_backends_have_ports() {
        let server_backends = ["pgvector", "qdrant", "weaviate", "milvus", "chroma", "vespa", "redis-vector"];
        for name in &server_backends {
            let info = select_backend(name).unwrap();
            assert!(info.default_port.is_some(), "{name} should have a default port");
        }
    }

    #[test]
    fn portless_backends() {
        for name in &["lancedb", "pinecone"] {
            let info = select_backend(name).unwrap();
            assert!(info.default_port.is_none(), "{name} should have no default port");
        }
    }

    // ---- display_name non-empty -----------------------------------------

    #[test]
    fn all_backends_have_display_name() {
        for name in known_backends() {
            let info = select_backend(name).unwrap();
            assert!(!info.display_name.is_empty(), "{name} has empty display_name");
        }
    }

    // ---- description non-empty ------------------------------------------

    #[test]
    fn all_backends_have_description() {
        for name in known_backends() {
            let info = select_backend(name).unwrap();
            assert!(!info.description.is_empty(), "{name} has empty description");
        }
    }
}
