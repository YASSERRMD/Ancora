/// Qdrant RAG (Retrieval-Augmented Generation) example.
///
/// Demonstrates the full upsert -> search -> hybrid pipeline using the
/// Qdrant REST request building layer without a live server.
///
/// Set QDRANT_URL to point at a running Qdrant instance to execute live.
/// Without QDRANT_URL the example prints the request shapes and exits.
use ancora_memory::backends::qdrant::{
    collection_url, create_collection_body, filter_to_qdrant, json_to_payload,
    parse_search_response, payload_to_json, search_body_with_threshold, search_url, upsert_body,
    upsert_url, QdrantConfig,
};
use ancora_memory::vector_store::{Distance, Filter, PayloadValue};

const COLLECTION: &str = "rag_documents";
const DIMS: usize = 384;

fn main() {
    println!("=== Qdrant RAG example ===\n");

    let cfg = QdrantConfig::local();
    println!("-- Config --");
    println!("url: {}, timeout: {}s\n", cfg.url, cfg.timeout_secs);

    // 1. Collection creation body
    let create_body = create_collection_body(DIMS, &Distance::Cosine);
    println!("-- PUT {} --", collection_url(&cfg.url, COLLECTION));
    println!("{}\n", serde_json::to_string_pretty(&create_body).unwrap());

    // 2. Upsert body
    let mut payload = ancora_memory::vector_store::Payload::new();
    payload.insert(
        "title".to_owned(),
        PayloadValue::String("Qdrant RAG guide".to_owned()),
    );
    payload.insert("year".to_owned(), PayloadValue::Integer(2024));
    let payload_json = payload_to_json(&payload);
    let embedding: Vec<f32> = (0..DIMS).map(|i| (i as f32) / (DIMS as f32)).collect();
    let pts = vec![(1u64, embedding.clone(), payload_json)];
    let upsert = upsert_body(&pts);
    println!("-- PUT {} --", upsert_url(&cfg.url, COLLECTION));
    println!("{}\n", serde_json::to_string_pretty(&upsert).unwrap());

    // 3. Search with threshold
    let query_body = search_body_with_threshold(&embedding[..8], 5, 0.75, None);
    println!("-- POST {} --", search_url(&cfg.url, COLLECTION));
    println!("{}\n", serde_json::to_string_pretty(&query_body).unwrap());

    // 4. Filtered search
    let f = Filter::Gt("year".to_owned(), PayloadValue::Integer(2020));
    let filter_json = filter_to_qdrant(&f);
    println!("-- Filter JSON --");
    println!("{}\n", serde_json::to_string_pretty(&filter_json).unwrap());

    // 5. Parse a mock response
    let mock_response = serde_json::json!({
        "result": [{ "id": 1, "score": 0.98, "payload": { "title": "test" } }]
    });
    let results = parse_search_response(&mock_response);
    println!("-- Parsed results --");
    for (id, score, payload) in &results {
        let p = json_to_payload(payload);
        println!("  id={id}, score={score:.3}, payload_keys={}", p.len());
    }

    println!("\nSet QDRANT_URL to run against a live Qdrant instance.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_collection_body_is_valid() {
        let body = create_collection_body(DIMS, &Distance::Cosine);
        assert_eq!(body["vectors"]["size"], DIMS);
    }

    #[test]
    fn example_payload_round_trips() {
        let mut p = ancora_memory::vector_store::Payload::new();
        p.insert("k".to_owned(), PayloadValue::String("v".to_owned()));
        let json = payload_to_json(&p);
        let back = json_to_payload(&json);
        assert_eq!(back.get("k"), Some(&PayloadValue::String("v".to_owned())));
    }
}
