/// Integration tests for the Pinecone backend.
/// Skipped by default; set ANCORA_PINECONE_API_KEY and ANCORA_PINECONE_HOST to run.

#[cfg(test)]
mod pinecone_integration {
    use crate::backends::pinecone::*;

    fn pinecone_host() -> Option<String> {
        std::env::var("ANCORA_PINECONE_HOST").ok()
    }

    #[test]
    #[ignore = "requires ANCORA_PINECONE_API_KEY and ANCORA_PINECONE_HOST"]
    fn live_upsert_and_query() {
        // Stub: upsert a vector then query by nearest neighbor.
        let _ = pinecone_host();
    }

    #[test]
    #[ignore = "requires ANCORA_PINECONE_API_KEY and ANCORA_PINECONE_HOST"]
    fn live_delete_by_filter() {
        let _ = pinecone_host();
    }

    #[test]
    #[ignore = "requires ANCORA_PINECONE_API_KEY and ANCORA_PINECONE_HOST"]
    fn live_namespace_isolation() {
        let _ = pinecone_host();
    }
}
