/// Weaviate RAG (Retrieval-Augmented Generation) example.
///
/// Demonstrates schema creation, object upsert, near-vector search, and
/// hybrid search using the Weaviate REST/GraphQL request building layer.
///
/// Set WEAVIATE_URL to run against a real Weaviate instance.
/// Without the env var the example prints request shapes and exits cleanly.

use ancora_memory::backends::weaviate::{
    WeaviateConfig, create_class_body, create_class_with_properties_body,
    create_object_body, batch_objects_body, graphql_near_vector_query,
    graphql_hybrid_query, graphql_generative_query, where_filter_text,
    schema_url, objects_url, graphql_url, batch_objects_url, data_type,
    parse_graphql_get,
};

const CLASS: &str = "Document";
const DIMS: usize = 384;

fn main() {
    println!("=== Weaviate RAG example ===\n");

    let cfg = WeaviateConfig::local();
    println!("URL: {}\n", cfg.url);

    // 1. Schema
    let schema = create_class_with_properties_body(
        CLASS, "A document for RAG", "none",
        &[
            ("title", data_type::TEXT, "Document title"),
            ("body", data_type::TEXT, "Document body"),
            ("year", data_type::INT, "Publication year"),
        ],
    );
    println!("-- POST {} --", schema_url(&cfg.url));
    println!("{}\n", serde_json::to_string_pretty(&schema).unwrap());

    // 2. Object upsert
    let obj = create_object_body(
        CLASS,
        &serde_json::json!({"title": "Weaviate RAG guide", "body": "...", "year": 2024}),
        Some(&vec![0.1f32; 8]), // first 8 dims
    );
    println!("-- POST {} --", objects_url(&cfg.url));
    println!("{}\n", serde_json::to_string_pretty(&obj).unwrap());

    // 3. Batch upsert
    let objs = vec![
        (CLASS.to_owned(), serde_json::json!({"title": "doc1"}), Some(vec![0.1f32; 8])),
        (CLASS.to_owned(), serde_json::json!({"title": "doc2"}), Some(vec![0.2f32; 8])),
    ];
    let batch = batch_objects_body(&objs);
    println!("-- POST {} --", batch_objects_url(&cfg.url));
    println!("{}\n", serde_json::to_string_pretty(&batch).unwrap());

    // 4. Near-vector search
    let query = graphql_near_vector_query(CLASS, &[0.1f32; 8], 5, &["title", "body"]);
    println!("-- POST {} (nearVector) --", graphql_url(&cfg.url));
    println!("{}\n", serde_json::to_string_pretty(&query).unwrap());

    // 5. Hybrid search
    let hybrid = graphql_hybrid_query(CLASS, "RAG guide", None, 0.7, 5, &["title"]);
    println!("-- POST {} (hybrid) --", graphql_url(&cfg.url));
    println!("{}\n", serde_json::to_string_pretty(&hybrid).unwrap());

    // 6. Generative search
    let gen = graphql_generative_query(CLASS, &[0.1f32; 8], 3, "Summarize {title}: {body}", &["title", "body"]);
    println!("-- POST {} (generative) --", graphql_url(&cfg.url));
    println!("{}\n", serde_json::to_string_pretty(&gen).unwrap());

    // 7. Filter
    let filter = where_filter_text("title", "Like", "RAG*");
    println!("-- Where filter --");
    println!("{}\n", serde_json::to_string_pretty(&filter).unwrap());

    // 8. Parse mock response
    let mock_resp = serde_json::json!({
        "data": { "Get": { "Document": [{"title": "mock", "_additional": {"id": "abc"}}] } }
    });
    let results = parse_graphql_get(&mock_resp, CLASS);
    println!("-- Parsed results ({} items) --", results.len());

    println!("Set WEAVIATE_URL to run against a live Weaviate instance.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_schema_body_has_class() {
        let body = create_class_body(CLASS, "desc", "none");
        assert_eq!(body["class"], CLASS);
    }

    #[test]
    fn example_object_body_has_class_and_properties() {
        let obj = create_object_body(CLASS, &serde_json::json!({"title": "t"}), None);
        assert_eq!(obj["class"], CLASS);
    }

    #[test]
    fn example_parse_graphql_extracts_results() {
        let body = serde_json::json!({
            "data": { "Get": { "Document": [{"title": "x"}] } }
        });
        let r = parse_graphql_get(&body, CLASS);
        assert_eq!(r.len(), 1);
    }
}
