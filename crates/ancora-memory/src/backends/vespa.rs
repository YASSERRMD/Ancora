/// Vespa backend for the `VectorStore` trait.
///
/// Vespa is an open-source big-data serving engine that supports ANN vector
/// search, BM25 keyword search, and hybrid ranking. This module generates
/// request bodies and URL strings for the Vespa Document and Query APIs
/// without requiring a live server.
///
/// Requires the `vespa` feature: `ancora-memory = { features = ["vespa"] }`.

use serde_json::{json, Value};

// ---- connection config ---------------------------------------------------

#[derive(Debug, Clone)]
pub struct VespaConfig {
    pub url: String,
    pub application: String,
    pub api_key: Option<String>,
    pub timeout_secs: u64,
}

impl VespaConfig {
    pub fn new(url: impl Into<String>, application: impl Into<String>) -> Self {
        Self { url: url.into(), application: application.into(), api_key: None, timeout_secs: 30 }
    }

    pub fn with_api_key(mut self, key: impl Into<String>) -> Self {
        self.api_key = Some(key.into()); self
    }

    pub fn local() -> Self { Self::new("http://localhost:8080", "default") }

    pub fn auth_header(&self) -> Option<String> {
        self.api_key.as_ref().map(|k| format!("Bearer {k}"))
    }
}

// ---- URL builders --------------------------------------------------------

pub fn document_v1_url(base: &str, namespace: &str, doc_type: &str, doc_id: &str) -> String {
    format!("{base}/document/v1/{namespace}/{doc_type}/docid/{doc_id}")
}

pub fn query_url(base: &str) -> String { format!("{base}/search/") }

pub fn feed_url(base: &str) -> String { format!("{base}/document/v1/") }

pub fn delete_url(base: &str, namespace: &str, doc_type: &str, doc_id: &str) -> String {
    format!("{base}/document/v1/{namespace}/{doc_type}/docid/{doc_id}")
}

pub fn status_url(base: &str) -> String { format!("{base}/state/v1/health") }

// ---- document body builders ---------------------------------------------

/// Build a Vespa document PUT body.
pub fn put_document_body(fields: Value) -> Value {
    json!({ "fields": fields })
}

/// Build a Vespa document UPDATE (partial update) body.
pub fn update_document_body(field: &str, value: Value, create: bool) -> Value {
    json!({
        "create": create,
        "fields": { field: { "assign": value } }
    })
}

// ---- query builders -----------------------------------------------------

/// Build a simple ANN nearest-neighbor query.
pub fn ann_query(
    doc_type: &str,
    target_hits: usize,
    vector_field: &str,
    query_tensor: &str,
) -> Value {
    json!({
        "yql": format!(
            "select * from {doc_type} where {{targetHits: {target_hits}}}nearestNeighbor({vector_field},{query_tensor})"
        ),
        "ranking.profile": "closeness",
        "hits": target_hits,
    })
}

/// Build a BM25 keyword query.
pub fn bm25_query(doc_type: &str, field: &str, text: &str, hits: usize) -> Value {
    json!({
        "yql": format!("select * from {doc_type} where userQuery()"),
        "query": text,
        "ranking.profile": "bm25",
        "hits": hits,
    })
}

/// Build a hybrid ANN + BM25 query using Vespa's WAND/hybrid ranking.
pub fn hybrid_query(
    doc_type: &str,
    target_hits: usize,
    vector_field: &str,
    query_tensor: &str,
    keyword: &str,
    hits: usize,
    alpha: f32,
) -> Value {
    let yql = format!(
        "select * from {doc_type} where ({{targetHits: {target_hits}}}nearestNeighbor({vector_field},{query_tensor})) or userQuery()"
    );
    json!({
        "yql": yql,
        "query": keyword,
        "ranking.profile": "hybrid",
        "ranking.features.query(alpha)": alpha,
        "hits": hits,
    })
}

/// Build a YQL filter restriction (appended to any query).
pub fn yql_where_and(base_yql: &str, condition: &str) -> String {
    format!("{base_yql} and ({condition})")
}

// ---- ranking profile helpers --------------------------------------------

/// Describe a custom hybrid ranking profile.
pub fn hybrid_ranking_profile(alpha: f32) -> Value {
    json!({
        "name": "hybrid",
        "inherits": "default",
        "first-phase": {
            "expression": format!("query(alpha) * closeness(field, embedding) + (1 - query(alpha)) * bm25(body)")
        },
        "rank-properties": {
            "query(alpha)": alpha
        }
    })
}

// ---- response parsing ---------------------------------------------------

pub fn parse_hits(body: &Value) -> Vec<(String, f32, Value)> {
    body["root"]["children"]
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .map(|hit| {
            let id = hit["id"].as_str().unwrap_or("").to_owned();
            let score = hit["relevance"].as_f64().unwrap_or(0.0) as f32;
            let fields = hit["fields"].clone();
            (id, score, fields)
        })
        .collect()
}

pub fn parse_total_count(body: &Value) -> u64 {
    body["root"]["fields"]["totalCount"].as_u64().unwrap_or(0)
}

// ---- error handling -----------------------------------------------------

#[derive(Debug, PartialEq)]
pub enum VespaError {
    NotFound(String),
    BadRequest(String),
    Unauthorized,
    InternalError(String),
    Unknown(u16, String),
}

impl VespaError {
    pub fn from_response(status: u16, body: &str) -> Self {
        let msg = serde_json::from_str::<Value>(body)
            .ok()
            .and_then(|v| v["message"].as_str().map(|s| s.to_owned()))
            .unwrap_or_else(|| body.to_owned());
        match status {
            401 | 403 => Self::Unauthorized,
            400 | 404 => if status == 404 { Self::NotFound(msg) } else { Self::BadRequest(msg) },
            500..=599 => Self::InternalError(msg),
            _ => Self::Unknown(status, msg),
        }
    }

    pub fn is_transient(&self) -> bool {
        matches!(self, Self::InternalError(_))
    }
}

pub const MAX_RETRIES: u32 = 3;

pub fn vespa_retry_delay_ms(attempt: u32) -> u64 {
    let base: u64 = 150u64.saturating_mul(1u64 << attempt.min(6));
    base.min(8_000)
}

// ---- unit tests ---------------------------------------------------------

#[cfg(test)]
mod vespa_tests {
    use super::*;

    #[test]
    fn document_url_builds_correctly() {
        let url = document_v1_url("http://localhost:8080", "ns", "doc", "123");
        assert_eq!(url, "http://localhost:8080/document/v1/ns/doc/docid/123");
    }

    #[test]
    fn query_url_builds_correctly() {
        let url = query_url("http://localhost:8080");
        assert_eq!(url, "http://localhost:8080/search/");
    }

    #[test]
    fn ann_query_has_nearest_neighbor_yql() {
        let q = ann_query("doc", 10, "embedding", "query_emb");
        let yql = q["yql"].as_str().unwrap();
        assert!(yql.contains("nearestNeighbor"), "yql: {yql}");
    }

    #[test]
    fn bm25_query_has_user_query_yql() {
        let q = bm25_query("doc", "body", "machine learning", 5);
        assert_eq!(q["query"], "machine learning");
        assert_eq!(q["hits"], 5);
    }

    #[test]
    fn hybrid_query_has_both_ann_and_userquery() {
        let q = hybrid_query("doc", 10, "emb", "q_emb", "test", 5, 0.7);
        let yql = q["yql"].as_str().unwrap();
        assert!(yql.contains("nearestNeighbor"), "yql: {yql}");
        assert!(yql.contains("userQuery"), "yql: {yql}");
    }

    #[test]
    fn hybrid_query_alpha_propagates() {
        let q = hybrid_query("doc", 10, "emb", "q_emb", "test", 5, 0.75);
        // 0.75 is exact in f32 and f64, avoiding precision comparison issues
        assert_eq!(q["ranking.features.query(alpha)"], 0.75f64);
    }

    #[test]
    fn put_document_body_wraps_fields() {
        let body = put_document_body(json!({"title": "test", "embedding": [0.1f32]}));
        assert!(body["fields"]["title"].is_string());
    }

    #[test]
    fn update_document_assigns_field() {
        let body = update_document_body("title", json!("updated"), false);
        assert_eq!(body["fields"]["title"]["assign"], "updated");
    }

    #[test]
    fn yql_where_and_appends_condition() {
        let yql = "select * from doc where true";
        let result = yql_where_and(yql, "year > 2020");
        assert!(result.contains("and (year > 2020)"), "result: {result}");
    }

    #[test]
    fn parse_hits_extracts_relevance() {
        let body = json!({
            "root": { "children": [
                { "id": "id:ns:doc::1", "relevance": 0.85, "fields": {"title": "test"} }
            ]}
        });
        let hits = parse_hits(&body);
        assert_eq!(hits.len(), 1);
        assert!((hits[0].1 - 0.85f32).abs() < 1e-4);
    }

    #[test]
    fn parse_total_count_reads_total() {
        let body = json!({ "root": { "fields": { "totalCount": 42 } } });
        assert_eq!(parse_total_count(&body), 42);
    }

    #[test]
    fn vespa_error_500_is_transient() {
        let err = VespaError::from_response(500, "internal");
        assert!(err.is_transient());
    }

    #[test]
    fn retry_delay_caps_at_8s() {
        assert!(vespa_retry_delay_ms(20) <= 8_000);
    }
}
