//! Example: using the managed and cloud-hosted vector backends in Ancora.
//!
//! This example demonstrates how to configure and build request payloads for
//! Chroma, Pinecone, Vespa, and Redis Vector without a live server. It shows
//! the builder pattern and JSON descriptors produced by each backend module.
//!
//! No network calls are made.

fn main() {
    chroma_demo();
    pinecone_demo();
    vespa_demo();
    redis_demo();
    backend_selector_demo();
}

// ---- Chroma demo --------------------------------------------------------

#[cfg(feature = "chroma")]
fn chroma_demo() {
    use ancora_memory::backends::chroma::*;

    let cfg = ChromaConfig::local();
    println!("Chroma local URL: {}", cfg.url);

    let body = create_collection_body("docs", Some(serde_json::json!({"hnsw:space": "cosine"})));
    println!("Chroma create collection: {body}");

    let add = add_body(
        &["doc-1", "doc-2"],
        &[vec![0.1f32, 0.2f32], vec![0.3f32, 0.4f32]],
        &[
            serde_json::json!({"source": "user"}),
            serde_json::json!({"source": "system"}),
        ],
        Some(&["Hello world", "Bonjour le monde"]),
    );
    println!(
        "Chroma add body vector count: {}",
        add["ids"].as_array().unwrap().len()
    );

    let filter = where_and(
        where_eq("lang", serde_json::json!("en")),
        where_gt("score", serde_json::json!(0.5)),
    );
    let query = query_body(
        &[vec![0.1f32, 0.2f32]],
        5,
        Some(filter),
        &["distances", "ids"],
    );
    println!("Chroma query n_results: {}", query["n_results"]);
}

#[cfg(not(feature = "chroma"))]
fn chroma_demo() {
    println!("[chroma feature not enabled -- skipping Chroma demo]");
}

// ---- Pinecone demo ------------------------------------------------------

#[cfg(feature = "pinecone")]
fn pinecone_demo() {
    use ancora_memory::backends::pinecone::*;

    let idx_body =
        create_serverless_index_body("my-index", 768, metric::COSINE, "aws", "us-east-1");
    println!("Pinecone index spec: {}", idx_body["spec"]["serverless"]);

    let upsert = upsert_body(&[
        (
            "v1",
            vec![0.1f32, 0.2f32],
            serde_json::json!({"lang": "en"}),
        ),
        (
            "v2",
            vec![0.3f32, 0.4f32],
            serde_json::json!({"lang": "fr"}),
        ),
    ]);
    println!(
        "Pinecone upsert count: {}",
        upsert["vectors"].as_array().unwrap().len()
    );

    let filter = filter_and(vec![
        filter_eq("lang", serde_json::json!("en")),
        filter_gte("score", serde_json::json!(0.7)),
    ]);
    let qbody = query_body(&[0.1f32, 0.2f32], 10, Some(filter), true);
    println!("Pinecone query topK: {}", qbody["topK"]);
}

#[cfg(not(feature = "pinecone"))]
fn pinecone_demo() {
    println!("[pinecone feature not enabled -- skipping Pinecone demo]");
}

// ---- Vespa demo ---------------------------------------------------------

#[cfg(feature = "vespa")]
fn vespa_demo() {
    use ancora_memory::backends::vespa::*;

    let cfg = VespaConfig::local();
    println!("Vespa local URL: {}", cfg.url);

    let ann = ann_query("doc", 10, "embedding", "query_embedding");
    println!("Vespa ANN YQL: {}", ann["yql"]);

    let bm25 = bm25_query("doc", "body", "vector search tutorial", 5);
    println!("Vespa BM25 profile: {}", bm25["ranking.profile"]);

    let hybrid = hybrid_query("doc", 10, "emb", "qEmb", "body", 5, 0.5);
    println!("Vespa hybrid YQL: {}", hybrid["yql"]);

    let profile = hybrid_ranking_profile(0.5);
    println!("Vespa hybrid ranking profile: {}", profile["name"]);
}

#[cfg(not(feature = "vespa"))]
fn vespa_demo() {
    println!("[vespa feature not enabled -- skipping Vespa demo]");
}

// ---- Redis Vector demo --------------------------------------------------

#[cfg(feature = "redis-vector")]
fn redis_demo() {
    use ancora_memory::backends::redis_vector::*;

    let cfg = RedisVectorConfig::local();
    println!("Redis URL: {}", cfg.url());

    let idx = CreateIndexArgs::new("docs_idx", "doc:", 128)
        .hnsw_params(200, 16)
        .add_field("lang", field_type::TAG)
        .add_field("score", field_type::NUMERIC);
    let idx_json = idx.to_json();
    println!("Redis FT.CREATE command: {}", idx_json["command"]);

    let search = SearchArgs::filtered_ann(
        "docs_idx",
        format!(
            "({}) ({})",
            tag_filter("lang", "en"),
            numeric_range("score", 0.8, 1.0)
        ),
        "embedding",
        10,
    )
    .returns(&["score", "payload", "lang"]);
    println!("Redis FT.SEARCH query: {}", search.to_json()["query"]);

    let key = document_key("doc", 42);
    println!("Redis document key: {key}");
}

#[cfg(not(feature = "redis-vector"))]
fn redis_demo() {
    println!("[redis-vector feature not enabled -- skipping Redis Vector demo]");
}

// ---- Backend selector demo ----------------------------------------------

fn backend_selector_demo() {
    use ancora_memory::backends::backend_selector::*;

    for name in known_backends() {
        let info = select_backend(name).unwrap();
        let port = info
            .default_port
            .map(|p| p.to_string())
            .unwrap_or_else(|| "n/a".to_owned());
        println!(
            "Backend {:15} feature={:14} port={:5} embedded={} managed={}",
            info.display_name,
            info.kind.feature_flag(),
            port,
            info.kind.is_embedded(),
            info.kind.is_managed_cloud(),
        );
    }
}
