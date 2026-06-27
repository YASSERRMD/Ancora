/// pgvector RAG (Retrieval-Augmented Generation) example.
///
/// Shows how to wire the pgvector SQL generation layer to a hypothetical
/// Postgres connection for a document retrieval pipeline.
///
/// Run with:
///   cargo run --example pgvector_rag --features ancora-memory/pgvector
///
/// Set TEST_DATABASE_URL to point at a running pgvector-enabled Postgres.
/// If the env var is absent this example prints the SQL it would execute
/// and exits cleanly -- no database required.

use ancora_memory::backends::pgvector::{
    create_table_sql, create_hnsw_index_sql, upsert_sql,
    cosine_query_with_threshold_sql, hybrid_query_sql,
    serialize_payload, encode_vector,
};
use ancora_memory::backends::pgvector_migration::full_setup_sql;
use ancora_memory::vector_store::{Payload, PayloadValue};

const COLLECTION: &str = "rag_documents";
const DIMS: usize = 1536;

fn main() {
    println!("=== pgvector RAG example ===\n");

    // 1. Setup SQL
    let setup = full_setup_sql(COLLECTION, DIMS, 16, 100)
        .expect("valid collection name");
    println!("-- Setup SQL --");
    println!("{setup}\n");

    // 2. Upsert SQL
    let upsert = upsert_sql(COLLECTION);
    println!("-- Upsert SQL --");
    println!("{upsert}\n");

    // 3. Encode a document payload
    let mut payload = Payload::new();
    payload.insert("title".to_owned(), PayloadValue::String("Introduction to RAG".to_owned()));
    payload.insert("source".to_owned(), PayloadValue::String("arxiv".to_owned()));
    payload.insert("year".to_owned(), PayloadValue::Integer(2024));
    let payload_json = serialize_payload(&payload).expect("serializable");
    println!("-- Payload JSON --");
    println!("{payload_json}\n");

    // 4. Encode a dummy embedding
    let embedding: Vec<f32> = (0..DIMS).map(|i| (i as f32) / (DIMS as f32)).collect();
    let encoded = encode_vector(&embedding[..8]); // first 8 dims for display
    println!("-- Embedding (first 8 dims) --");
    println!("{encoded}...\n");

    // 5. Cosine query with threshold
    let query_sql = cosine_query_with_threshold_sql(COLLECTION, 5, 0, 0.75);
    println!("-- Cosine query (threshold=0.75) --");
    println!("{query_sql}\n");

    // 6. Hybrid query
    let hybrid_sql = hybrid_query_sql(COLLECTION, 5, 0.7);
    println!("-- Hybrid query (alpha=0.7) --");
    println!("{hybrid_sql}\n");

    println!("Set TEST_DATABASE_URL to run against a real pgvector database.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_setup_sql_compiles_and_is_valid() {
        let sql = full_setup_sql(COLLECTION, DIMS, 16, 100).unwrap();
        assert!(sql.contains("CREATE TABLE"), "SQL: {sql}");
    }

    #[test]
    fn example_payload_serializes() {
        let mut p = Payload::new();
        p.insert("k".to_owned(), PayloadValue::String("v".to_owned()));
        assert!(serialize_payload(&p).is_ok());
    }

    #[test]
    fn example_encode_vector_non_empty() {
        let v = encode_vector(&[0.1f32, 0.2, 0.3]);
        assert!(!v.is_empty());
    }
}
