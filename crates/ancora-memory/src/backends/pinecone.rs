/// Pinecone backend for the `VectorStore` trait.
///
/// Pinecone is a fully managed vector database. This module generates request
/// bodies and URL strings for the Pinecone REST API without requiring a live
/// server.
///
/// Requires the `pinecone` feature: `ancora-memory = { features = ["pinecone"] }`.
use serde_json::{json, Value};

// ---- connection config ---------------------------------------------------

#[derive(Debug, Clone)]
pub struct PineconeConfig {
    pub api_key: String,
    pub environment: String,
    pub timeout_secs: u64,
}

impl PineconeConfig {
    pub fn new(api_key: impl Into<String>, environment: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            environment: environment.into(),
            timeout_secs: 30,
        }
    }

    pub fn auth_header(&self) -> String {
        format!("Api-Key {}", self.api_key)
    }

    /// Returns the controller base URL (for index management).
    pub fn controller_url(&self) -> String {
        "https://api.pinecone.io".to_string()
    }

    /// Returns the data-plane URL for a given index host.
    pub fn index_url(&self, host: &str) -> String {
        format!("https://{host}")
    }
}

// ---- URL builders --------------------------------------------------------

pub fn list_indexes_url(base: &str) -> String {
    format!("{base}/indexes")
}
pub fn create_index_url(base: &str) -> String {
    format!("{base}/indexes")
}
pub fn describe_index_url(base: &str, name: &str) -> String {
    format!("{base}/indexes/{name}")
}
pub fn delete_index_url(base: &str, name: &str) -> String {
    format!("{base}/indexes/{name}")
}
pub fn upsert_url(host: &str) -> String {
    format!("https://{host}/vectors/upsert")
}
pub fn query_url(host: &str) -> String {
    format!("https://{host}/query")
}
pub fn fetch_url(host: &str) -> String {
    format!("https://{host}/vectors/fetch")
}
pub fn delete_vectors_url(host: &str) -> String {
    format!("https://{host}/vectors/delete")
}
pub fn describe_index_stats_url(host: &str) -> String {
    format!("https://{host}/describe_index_stats")
}
pub fn list_vectors_url(host: &str) -> String {
    format!("https://{host}/vectors/list")
}

// ---- metric constants ---------------------------------------------------

pub mod metric {
    pub const COSINE: &str = "cosine";
    pub const DOT_PRODUCT: &str = "dotproduct";
    pub const EUCLIDEAN: &str = "euclidean";
}

// ---- index management bodies --------------------------------------------

/// Build a serverless index creation body.
pub fn create_serverless_index_body(
    name: &str,
    dims: usize,
    metric: &str,
    cloud: &str,
    region: &str,
) -> Value {
    json!({
        "name": name,
        "dimension": dims,
        "metric": metric,
        "spec": {
            "serverless": { "cloud": cloud, "region": region }
        }
    })
}

/// Build a pod-based index creation body.
pub fn create_pod_index_body(
    name: &str,
    dims: usize,
    metric: &str,
    pod_type: &str,
    replicas: u32,
) -> Value {
    json!({
        "name": name,
        "dimension": dims,
        "metric": metric,
        "spec": {
            "pod": { "pod_type": pod_type, "replicas": replicas }
        }
    })
}

// ---- data-plane bodies --------------------------------------------------

/// Build an upsert request body.
pub fn upsert_body(vectors: &[(&str, Vec<f32>, Value)]) -> Value {
    let vs: Vec<Value> = vectors
        .iter()
        .map(|(id, values, metadata)| json!({ "id": id, "values": values, "metadata": metadata }))
        .collect();
    json!({ "vectors": vs })
}

/// Build an upsert body for a specific namespace.
pub fn upsert_namespace_body(vectors: &[(&str, Vec<f32>, Value)], namespace: &str) -> Value {
    let vs: Vec<Value> = vectors
        .iter()
        .map(|(id, values, metadata)| json!({ "id": id, "values": values, "metadata": metadata }))
        .collect();
    json!({ "vectors": vs, "namespace": namespace })
}

/// Build a query body.
pub fn query_body(
    vector: &[f32],
    top_k: usize,
    filter: Option<Value>,
    include_metadata: bool,
) -> Value {
    let mut body = json!({
        "vector": vector,
        "topK": top_k,
        "includeMetadata": include_metadata,
    });
    if let Some(f) = filter {
        body["filter"] = f;
    }
    body
}

/// Build a namespace-scoped query body.
pub fn query_namespace_body(vector: &[f32], top_k: usize, namespace: &str) -> Value {
    json!({ "vector": vector, "topK": top_k, "namespace": namespace, "includeMetadata": true })
}

/// Build a delete-by-IDs body.
pub fn delete_ids_body(ids: &[&str]) -> Value {
    json!({ "ids": ids })
}

/// Build a delete-by-filter body.
pub fn delete_filter_body(filter: Value) -> Value {
    json!({ "filter": filter })
}

/// Build a delete-all-in-namespace body.
pub fn delete_namespace_body(namespace: &str) -> Value {
    json!({ "deleteAll": true, "namespace": namespace })
}

// ---- metadata filter helpers --------------------------------------------

pub fn filter_eq(field: &str, value: Value) -> Value {
    json!({ field: { "$eq": value } })
}

pub fn filter_ne(field: &str, value: Value) -> Value {
    json!({ field: { "$ne": value } })
}

pub fn filter_gte(field: &str, value: Value) -> Value {
    json!({ field: { "$gte": value } })
}

pub fn filter_lte(field: &str, value: Value) -> Value {
    json!({ field: { "$lte": value } })
}

pub fn filter_and(filters: Vec<Value>) -> Value {
    json!({ "$and": filters })
}

pub fn filter_or(filters: Vec<Value>) -> Value {
    json!({ "$or": filters })
}

pub fn filter_in(field: &str, values: Vec<Value>) -> Value {
    json!({ field: { "$in": values } })
}

// ---- response parsing ---------------------------------------------------

pub fn parse_matches(body: &Value) -> Vec<(String, f32, Value)> {
    body["matches"]
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .map(|m| {
            let id = m["id"].as_str().unwrap_or("").to_owned();
            let score = m["score"].as_f64().unwrap_or(0.0) as f32;
            let meta = m["metadata"].clone();
            (id, score, meta)
        })
        .collect()
}

pub fn parse_index_host(body: &Value) -> Option<String> {
    body["host"].as_str().map(|s| s.to_owned())
}

pub fn parse_index_stats(body: &Value) -> (u64, u64) {
    let total = body["totalVectorCount"].as_u64().unwrap_or(0);
    let dims = body["dimension"].as_u64().unwrap_or(0);
    (total, dims)
}

// ---- error handling -----------------------------------------------------

#[derive(Debug, PartialEq)]
pub enum PineconeError {
    NotFound(String),
    AlreadyExists(String),
    BadRequest(String),
    Unauthorized,
    RateLimited,
    InternalError(String),
    Unknown(u16, String),
}

impl PineconeError {
    pub fn from_response(status: u16, body: &str) -> Self {
        let msg = serde_json::from_str::<Value>(body)
            .ok()
            .and_then(|v| v["message"].as_str().map(|s| s.to_owned()))
            .unwrap_or_else(|| body.to_owned());
        match status {
            401 | 403 => Self::Unauthorized,
            404 => Self::NotFound(msg),
            409 => Self::AlreadyExists(msg),
            400 | 422 => Self::BadRequest(msg),
            429 => Self::RateLimited,
            500..=599 => Self::InternalError(msg),
            _ => Self::Unknown(status, msg),
        }
    }

    pub fn is_transient(&self) -> bool {
        matches!(self, Self::InternalError(_) | Self::RateLimited)
    }
}

pub const MAX_RETRIES: u32 = 4;

pub fn pinecone_retry_delay_ms(attempt: u32) -> u64 {
    let base: u64 = 200u64.saturating_mul(1u64 << attempt.min(6));
    base.min(10_000)
}

// ---- unit tests ---------------------------------------------------------

#[cfg(test)]
mod pinecone_tests {
    use super::*;

    #[test]
    fn config_auth_header_format() {
        let cfg = PineconeConfig::new("key-123", "us-east1-gcp");
        assert_eq!(cfg.auth_header(), "Api-Key key-123");
    }

    #[test]
    fn create_serverless_index_has_spec() {
        let body =
            create_serverless_index_body("my-index", 384, metric::COSINE, "aws", "us-east-1");
        assert_eq!(body["name"], "my-index");
        assert!(body["spec"]["serverless"].is_object());
    }

    #[test]
    fn create_pod_index_has_pod_spec() {
        let body = create_pod_index_body("my-index", 768, metric::DOT_PRODUCT, "s1.x1", 2);
        assert_eq!(body["spec"]["pod"]["replicas"], 2);
    }

    #[test]
    fn upsert_body_wraps_vectors() {
        let body = upsert_body(&[("id1", vec![0.1f32], json!({"tag": "x"}))]);
        assert_eq!(body["vectors"].as_array().unwrap().len(), 1);
    }

    #[test]
    fn query_body_includes_top_k() {
        let body = query_body(&[0.1f32], 10, None, true);
        assert_eq!(body["topK"], 10);
    }

    #[test]
    fn query_body_with_filter_sets_filter() {
        let f = filter_eq("tag", json!("ml"));
        let body = query_body(&[0.1f32], 5, Some(f), false);
        assert!(body["filter"].is_object());
    }

    #[test]
    fn filter_and_wraps_list() {
        let f = filter_and(vec![filter_eq("a", json!(1)), filter_eq("b", json!(2))]);
        assert_eq!(f["$and"].as_array().unwrap().len(), 2);
    }

    #[test]
    fn filter_in_produces_array() {
        let f = filter_in("tag", vec![json!("a"), json!("b")]);
        assert!(f["tag"]["$in"].is_array());
    }

    #[test]
    fn parse_matches_extracts_results() {
        let body = json!({ "matches": [{ "id": "v1", "score": 0.9, "metadata": {} }] });
        let results = parse_matches(&body);
        assert_eq!(results[0].0, "v1");
        assert!((results[0].1 - 0.9f32).abs() < 1e-4);
    }

    #[test]
    fn parse_index_stats_extracts_counts() {
        let body = json!({ "totalVectorCount": 1000, "dimension": 384 });
        let (total, dims) = parse_index_stats(&body);
        assert_eq!(total, 1000);
        assert_eq!(dims, 384);
    }

    #[test]
    fn pinecone_error_429_is_transient() {
        let err = PineconeError::from_response(429, "too many");
        assert!(err.is_transient());
    }

    #[test]
    fn retry_delay_caps_at_10s() {
        assert!(pinecone_retry_delay_ms(20) <= 10_000);
    }
}
