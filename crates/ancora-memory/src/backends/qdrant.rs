/// Qdrant backend for the `VectorStore` trait.
///
/// Uses the Qdrant REST API over `ureq` (synchronous, no tokio runtime needed).
/// Requires the `qdrant` feature: `ancora-memory = { features = ["qdrant"] }`.
///
/// Integration tests that hit a live Qdrant are `#[ignore]` and require
/// `QDRANT_URL` in the environment (e.g. `http://localhost:6333`).

// ---- connection config ---------------------------------------------------

/// Configuration for a Qdrant REST connection.
#[derive(Debug, Clone)]
pub struct QdrantConfig {
    /// Base URL of the Qdrant REST API (e.g. `http://localhost:6333`).
    pub url: String,
    /// Optional API key for Qdrant Cloud or authenticated deployments.
    pub api_key: Option<String>,
    /// Timeout for individual HTTP requests in seconds.
    pub timeout_secs: u64,
}

impl QdrantConfig {
    pub fn new(url: impl Into<String>) -> Self {
        Self { url: url.into(), api_key: None, timeout_secs: 30 }
    }

    pub fn with_api_key(mut self, key: impl Into<String>) -> Self {
        self.api_key = Some(key.into()); self
    }

    pub fn with_timeout(mut self, secs: u64) -> Self {
        self.timeout_secs = secs; self
    }

    pub fn local() -> Self { Self::new("http://localhost:6333") }

    /// Build the authorization header value if an API key is set.
    pub fn auth_header(&self) -> Option<String> {
        self.api_key.as_ref().map(|k| format!("Bearer {k}"))
    }
}

// ---- URL builders --------------------------------------------------------

pub fn collections_url(base: &str) -> String {
    format!("{base}/collections")
}

pub fn collection_url(base: &str, name: &str) -> String {
    format!("{base}/collections/{name}")
}

pub fn points_url(base: &str, name: &str) -> String {
    format!("{base}/collections/{name}/points")
}

pub fn upsert_url(base: &str, name: &str) -> String {
    format!("{base}/collections/{name}/points?wait=true")
}

pub fn search_url(base: &str, name: &str) -> String {
    format!("{base}/collections/{name}/points/search")
}

pub fn delete_points_url(base: &str, name: &str) -> String {
    format!("{base}/collections/{name}/points/delete?wait=true")
}

pub fn scroll_url(base: &str, name: &str) -> String {
    format!("{base}/collections/{name}/points/scroll")
}

// ---- collection lifecycle helpers ----------------------------------------

/// Build the JSON body for a collection with multiple named vectors.
///
/// `named_vectors` is a list of `(name, dimensions, distance)` tuples.
pub fn create_multi_vector_collection_body(
    named_vectors: &[(&str, usize, Distance)],
) -> serde_json::Value {
    let mut vectors = serde_json::Map::new();
    for (name, dims, dist) in named_vectors {
        vectors.insert(
            name.to_string(),
            json!({ "size": dims, "distance": distance_name(dist) }),
        );
    }
    json!({ "vectors": serde_json::Value::Object(vectors) })
}

/// Build the body for updating a collection's optimizer config.
pub fn update_optimizer_body(indexing_threshold: u64, memmap_threshold: u64) -> serde_json::Value {
    json!({
        "optimizers_config": {
            "indexing_threshold": indexing_threshold,
            "memmap_threshold": memmap_threshold
        }
    })
}

/// Build the body for creating an HNSW index on payload field.
pub fn create_payload_index_body(field_name: &str, field_type: &str) -> serde_json::Value {
    json!({
        "field_name": field_name,
        "field_schema": field_type
    })
}

// ---- request body builders -----------------------------------------------

use crate::vector_store::{Distance, Filter, PayloadValue};
use serde_json::{json, Value};

/// Serialize the distance metric to the Qdrant enum string.
pub fn distance_name(d: &Distance) -> &'static str {
    match d {
        Distance::Cosine => "Cosine",
        Distance::Dot => "Dot",
        Distance::L2 => "Euclid",
    }
}

/// Build the JSON body for `PUT /collections/{name}`.
pub fn create_collection_body(dimensions: usize, distance: &Distance) -> Value {
    json!({
        "vectors": {
            "size": dimensions,
            "distance": distance_name(distance)
        }
    })
}

/// Build the JSON body for a points upsert request.
pub fn upsert_body(points: &[(u64, Vec<f32>, Value)]) -> Value {
    let pts: Vec<Value> = points.iter().map(|(id, vec, payload)| json!({
        "id": id,
        "vector": vec,
        "payload": payload
    })).collect();
    json!({ "points": pts })
}

/// Build the JSON body for a nearest-neighbour search request.
pub fn search_body(vector: &[f32], top_k: usize, filter: Option<&Filter>) -> Value {
    let mut body = json!({
        "vector": vector,
        "limit": top_k,
        "with_payload": true,
        "with_vector": false
    });
    if let Some(f) = filter {
        body["filter"] = filter_to_qdrant(f);
    }
    body
}

/// Build the JSON body for a delete-by-ids request.
pub fn delete_by_ids_body(ids: &[u64]) -> Value {
    json!({ "points": ids })
}

/// Build the JSON body for a scroll (paginated list) request.
pub fn scroll_body(limit: usize, offset: Option<u64>) -> Value {
    let mut body = json!({
        "limit": limit,
        "with_payload": true,
        "with_vector": false
    });
    if let Some(off) = offset {
        body["offset"] = json!(off);
    }
    body
}

// ---- filter translation --------------------------------------------------

/// Translate a `Filter` to the Qdrant REST filter JSON schema.
pub fn filter_to_qdrant(f: &Filter) -> Value {
    match f {
        Filter::Eq(key, val) => json!({
            "must": [{ "key": key, "match": { "value": payload_val_to_json(val) } }]
        }),
        Filter::Ne(key, val) => json!({
            "must_not": [{ "key": key, "match": { "value": payload_val_to_json(val) } }]
        }),
        Filter::Gt(key, val) => json!({
            "must": [{ "key": key, "range": { "gt": payload_val_to_json(val) } }]
        }),
        Filter::Lt(key, val) => json!({
            "must": [{ "key": key, "range": { "lt": payload_val_to_json(val) } }]
        }),
        Filter::And(a, b) => {
            let fa = filter_to_qdrant(a);
            let fb = filter_to_qdrant(b);
            merge_must(fa, fb)
        }
        Filter::Or(a, b) => {
            let fa = filter_to_qdrant(a);
            let fb = filter_to_qdrant(b);
            json!({ "should": [fa, fb] })
        }
    }
}

fn payload_val_to_json(v: &PayloadValue) -> Value {
    match v {
        PayloadValue::String(s) => json!(s),
        PayloadValue::Integer(n) => json!(n),
        PayloadValue::Float(f) => json!(f),
        PayloadValue::Bool(b) => json!(b),
        PayloadValue::Null => Value::Null,
    }
}

fn merge_must(a: Value, b: Value) -> Value {
    let mut must_conditions: Vec<Value> = Vec::new();
    if let Some(arr) = a.get("must").and_then(|v| v.as_array()) {
        must_conditions.extend(arr.iter().cloned());
    } else {
        must_conditions.push(a);
    }
    if let Some(arr) = b.get("must").and_then(|v| v.as_array()) {
        must_conditions.extend(arr.iter().cloned());
    } else {
        must_conditions.push(b);
    }
    json!({ "must": must_conditions })
}

// ---- response parsing helpers --------------------------------------------

/// Extract the scored points from a Qdrant search response.
pub fn parse_search_response(body: &Value) -> Vec<(u64, f32, Value)> {
    body["result"]
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .filter_map(|hit| {
            let id = hit["id"].as_u64()?;
            let score = hit["score"].as_f64()? as f32;
            let payload = hit["payload"].clone();
            Some((id, score, payload))
        })
        .collect()
}

/// Extract the collection info from a Qdrant describe-collection response.
pub fn parse_collection_info(body: &Value) -> Option<(usize, u64)> {
    let dims = body["result"]["config"]["params"]["vectors"]["size"].as_u64()? as usize;
    let count = body["result"]["points_count"].as_u64().unwrap_or(0);
    Some((dims, count))
}

// ---- tests (all offline) ------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vector_store::{Distance, Filter, PayloadValue};

    #[test]
    fn qdrant_config_default_url() {
        let cfg = QdrantConfig::local();
        assert_eq!(cfg.url, "http://localhost:6333");
    }

    #[test]
    fn qdrant_config_auth_header_with_key() {
        let cfg = QdrantConfig::new("http://localhost:6333").with_api_key("test-key");
        let header = cfg.auth_header().unwrap();
        assert!(header.contains("test-key"), "header: {header}");
    }

    #[test]
    fn qdrant_config_auth_header_without_key_is_none() {
        let cfg = QdrantConfig::local();
        assert!(cfg.auth_header().is_none());
    }

    #[test]
    fn collections_url_format() {
        assert_eq!(collections_url("http://localhost:6333"), "http://localhost:6333/collections");
    }

    #[test]
    fn collection_url_includes_name() {
        let url = collection_url("http://localhost:6333", "docs");
        assert!(url.contains("/collections/docs"), "url: {url}");
    }

    #[test]
    fn upsert_url_has_wait_param() {
        let url = upsert_url("http://localhost:6333", "docs");
        assert!(url.contains("wait=true"), "url: {url}");
    }

    #[test]
    fn create_collection_body_has_correct_distance() {
        let body = create_collection_body(384, &Distance::Cosine);
        assert_eq!(body["vectors"]["distance"], "Cosine");
        assert_eq!(body["vectors"]["size"], 384);
    }

    #[test]
    fn distance_name_maps_all_variants() {
        assert_eq!(distance_name(&Distance::Cosine), "Cosine");
        assert_eq!(distance_name(&Distance::Dot), "Dot");
        assert_eq!(distance_name(&Distance::L2), "Euclid");
    }

    #[test]
    fn upsert_body_has_points_array() {
        let pts = vec![(1u64, vec![0.1f32, 0.2], serde_json::json!({"k": "v"}))];
        let body = upsert_body(&pts);
        assert!(body["points"].is_array());
        assert_eq!(body["points"][0]["id"], 1);
    }

    #[test]
    fn search_body_has_required_fields() {
        let body = search_body(&[0.1, 0.2], 5, None);
        assert_eq!(body["limit"], 5);
        assert_eq!(body["with_payload"], true);
    }

    #[test]
    fn filter_eq_produces_must_clause() {
        let f = Filter::Eq("lang".to_owned(), PayloadValue::String("en".to_owned()));
        let json = filter_to_qdrant(&f);
        assert!(json["must"].is_array(), "json: {json}");
        assert_eq!(json["must"][0]["key"], "lang");
    }

    #[test]
    fn filter_ne_produces_must_not_clause() {
        let f = Filter::Ne("status".to_owned(), PayloadValue::String("archived".to_owned()));
        let json = filter_to_qdrant(&f);
        assert!(json["must_not"].is_array(), "json: {json}");
    }

    #[test]
    fn filter_gt_produces_range_clause() {
        let f = Filter::Gt("year".to_owned(), PayloadValue::Integer(2020));
        let json = filter_to_qdrant(&f);
        assert_eq!(json["must"][0]["range"]["gt"], 2020);
    }

    #[test]
    fn filter_and_merges_must_conditions() {
        let f = Filter::Eq("a".to_owned(), PayloadValue::Integer(1))
            .and(Filter::Eq("b".to_owned(), PayloadValue::Integer(2)));
        let json = filter_to_qdrant(&f);
        let must = json["must"].as_array().unwrap();
        assert_eq!(must.len(), 2);
    }

    #[test]
    fn filter_or_produces_should_clause() {
        let f = Filter::Eq("x".to_owned(), PayloadValue::String("a".to_owned()))
            .or(Filter::Eq("x".to_owned(), PayloadValue::String("b".to_owned())));
        let json = filter_to_qdrant(&f);
        assert!(json["should"].is_array(), "json: {json}");
    }

    #[test]
    fn parse_search_response_extracts_results() {
        let body = serde_json::json!({
            "result": [
                { "id": 42, "score": 0.95, "payload": { "k": "v" } }
            ]
        });
        let results = parse_search_response(&body);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].0, 42);
        assert!((results[0].1 - 0.95).abs() < 0.001);
    }

    #[test]
    fn create_multi_vector_collection_body_has_all_names() {
        let body = create_multi_vector_collection_body(&[
            ("text", 384, Distance::Cosine),
            ("image", 512, Distance::L2),
        ]);
        assert!(body["vectors"]["text"].is_object());
        assert!(body["vectors"]["image"].is_object());
        assert_eq!(body["vectors"]["image"]["distance"], "Euclid");
    }

    #[test]
    fn update_optimizer_body_has_config_keys() {
        let body = update_optimizer_body(20_000, 50_000);
        assert_eq!(body["optimizers_config"]["indexing_threshold"], 20_000);
    }

    #[test]
    fn create_payload_index_body_has_field_name() {
        let body = create_payload_index_body("source", "keyword");
        assert_eq!(body["field_name"], "source");
        assert_eq!(body["field_schema"], "keyword");
    }

    #[test]
    fn parse_collection_info_extracts_dims_and_count() {
        let body = serde_json::json!({
            "result": {
                "config": { "params": { "vectors": { "size": 1536 } } },
                "points_count": 100
            }
        });
        let (dims, count) = parse_collection_info(&body).unwrap();
        assert_eq!(dims, 1536);
        assert_eq!(count, 100);
    }
}
