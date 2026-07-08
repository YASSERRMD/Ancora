/// Chroma backend for the `VectorStore` trait.
///
/// Chroma is an open-source embedding database designed for quick prototyping.
/// It exposes a REST API that this module targets without requiring a live server.
///
/// Requires the `chroma` feature: `ancora-memory = { features = ["chroma"] }`.
use serde_json::{json, Value};

// ---- connection config ---------------------------------------------------

#[derive(Debug, Clone)]
pub struct ChromaConfig {
    pub url: String,
    pub tenant: String,
    pub database: String,
    pub api_key: Option<String>,
    pub timeout_secs: u64,
}

impl ChromaConfig {
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            tenant: "default_tenant".to_owned(),
            database: "default_database".to_owned(),
            api_key: None,
            timeout_secs: 30,
        }
    }

    pub fn with_api_key(mut self, key: impl Into<String>) -> Self {
        self.api_key = Some(key.into());
        self
    }

    pub fn with_tenant(mut self, t: impl Into<String>) -> Self {
        self.tenant = t.into();
        self
    }

    pub fn local() -> Self {
        Self::new("http://localhost:8000")
    }

    pub fn auth_header(&self) -> Option<String> {
        self.api_key.as_ref().map(|k| format!("Bearer {k}"))
    }
}

// ---- URL builders --------------------------------------------------------

pub fn collections_url(base: &str, tenant: &str, db: &str) -> String {
    format!("{base}/api/v2/tenants/{tenant}/databases/{db}/collections")
}

pub fn collection_url(base: &str, tenant: &str, db: &str, id: &str) -> String {
    format!("{base}/api/v2/tenants/{tenant}/databases/{db}/collections/{id}")
}

pub fn add_url(base: &str, tenant: &str, db: &str, collection_id: &str) -> String {
    format!("{base}/api/v2/tenants/{tenant}/databases/{db}/collections/{collection_id}/add")
}

pub fn query_url(base: &str, tenant: &str, db: &str, collection_id: &str) -> String {
    format!("{base}/api/v2/tenants/{tenant}/databases/{db}/collections/{collection_id}/query")
}

pub fn delete_url(base: &str, tenant: &str, db: &str, collection_id: &str) -> String {
    format!("{base}/api/v2/tenants/{tenant}/databases/{db}/collections/{collection_id}/delete")
}

pub fn get_url(base: &str, tenant: &str, db: &str, collection_id: &str) -> String {
    format!("{base}/api/v2/tenants/{tenant}/databases/{db}/collections/{collection_id}/get")
}

pub fn heartbeat_url(base: &str) -> String {
    format!("{base}/api/v2/heartbeat")
}

// ---- request body builders -----------------------------------------------

pub fn create_collection_body(name: &str, metadata: Option<Value>) -> Value {
    let mut body = json!({ "name": name });
    if let Some(m) = metadata {
        body["metadata"] = m;
    }
    body
}

pub fn add_body(
    ids: &[&str],
    embeddings: &[Vec<f32>],
    metadatas: &[Value],
    documents: Option<&[&str]>,
) -> Value {
    let mut body = json!({ "ids": ids, "embeddings": embeddings, "metadatas": metadatas });
    if let Some(docs) = documents {
        body["documents"] = json!(docs);
    }
    body
}

pub fn query_body(
    query_embeddings: &[Vec<f32>],
    n_results: usize,
    where_filter: Option<Value>,
    include: &[&str],
) -> Value {
    let mut body = json!({
        "query_embeddings": query_embeddings,
        "n_results": n_results,
        "include": include,
    });
    if let Some(w) = where_filter {
        body["where"] = w;
    }
    body
}

pub fn delete_body(ids: &[&str], where_filter: Option<Value>) -> Value {
    let mut body = json!({ "ids": ids });
    if let Some(w) = where_filter {
        body["where"] = w;
    }
    body
}

pub fn get_body(
    ids: Option<&[&str]>,
    where_filter: Option<Value>,
    limit: Option<usize>,
    include: &[&str],
) -> Value {
    let mut body = json!({ "include": include });
    if let Some(i) = ids {
        body["ids"] = json!(i);
    }
    if let Some(w) = where_filter {
        body["where"] = w;
    }
    if let Some(l) = limit {
        body["limit"] = json!(l);
    }
    body
}

// ---- metadata filter helpers ---------------------------------------------

/// `{ "field": { "$eq": value } }` equality filter.
pub fn where_eq(field: &str, value: Value) -> Value {
    json!({ field: { "$eq": value } })
}

/// `{ "field": { "$ne": value } }` not-equal filter.
pub fn where_ne(field: &str, value: Value) -> Value {
    json!({ field: { "$ne": value } })
}

/// `{ "field": { "$gt": value } }` greater-than filter.
pub fn where_gt(field: &str, value: Value) -> Value {
    json!({ field: { "$gt": value } })
}

/// `{ "field": { "$lt": value } }` less-than filter.
pub fn where_lt(field: &str, value: Value) -> Value {
    json!({ field: { "$lt": value } })
}

/// `{ "$and": [a, b] }` combinator.
pub fn where_and(a: Value, b: Value) -> Value {
    json!({ "$and": [a, b] })
}

/// `{ "$or": [a, b] }` combinator.
pub fn where_or(a: Value, b: Value) -> Value {
    json!({ "$or": [a, b] })
}

/// `{ "field": { "$in": values } }` membership filter.
pub fn where_in(field: &str, values: Vec<Value>) -> Value {
    json!({ field: { "$in": values } })
}

// ---- response parsing ----------------------------------------------------

pub fn parse_query_ids(body: &Value) -> Vec<Vec<String>> {
    body["ids"]
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .map(|row| {
            row.as_array()
                .unwrap_or(&vec![])
                .iter()
                .filter_map(|v| v.as_str().map(|s| s.to_owned()))
                .collect()
        })
        .collect()
}

pub fn parse_query_distances(body: &Value) -> Vec<Vec<f32>> {
    body["distances"]
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .map(|row| {
            row.as_array()
                .unwrap_or(&vec![])
                .iter()
                .filter_map(|v| v.as_f64().map(|d| d as f32))
                .collect()
        })
        .collect()
}

pub fn parse_collection_id(body: &Value) -> Option<String> {
    body["id"].as_str().map(|s| s.to_owned())
}

pub fn parse_collection_count(body: &Value) -> u64 {
    body["count"].as_u64().unwrap_or(0)
}

// ---- error handling ------------------------------------------------------

#[derive(Debug, PartialEq)]
pub enum ChromaError {
    NotFound(String),
    AlreadyExists(String),
    BadRequest(String),
    Unauthorized,
    InternalError(String),
    Unknown(u16, String),
}

impl ChromaError {
    pub fn from_response(status: u16, body: &str) -> Self {
        let msg = serde_json::from_str::<Value>(body)
            .ok()
            .and_then(|v| v["error"].as_str().map(|s| s.to_owned()))
            .unwrap_or_else(|| body.to_owned());
        match status {
            401 | 403 => Self::Unauthorized,
            404 => Self::NotFound(msg),
            409 => Self::AlreadyExists(msg),
            400 | 422 => Self::BadRequest(msg),
            500..=599 => Self::InternalError(msg),
            _ => Self::Unknown(status, msg),
        }
    }

    pub fn is_transient(&self) -> bool {
        matches!(self, Self::InternalError(_))
    }
}

pub const MAX_RETRIES: u32 = 3;

pub fn chroma_retry_delay_ms(attempt: u32) -> u64 {
    let base: u64 = 100u64.saturating_mul(1u64 << attempt.min(6));
    base.min(5_000)
}

// ---- unit tests ----------------------------------------------------------

#[cfg(test)]
mod chroma_tests {
    use super::*;

    #[test]
    fn url_builders_produce_expected_paths() {
        let base = "http://localhost:8000";
        assert_eq!(
            collections_url(base, "default_tenant", "default_database"),
            "http://localhost:8000/api/v2/tenants/default_tenant/databases/default_database/collections"
        );
        assert_eq!(
            heartbeat_url(base),
            "http://localhost:8000/api/v2/heartbeat"
        );
    }

    #[test]
    fn create_collection_body_has_name() {
        let body = create_collection_body("test_col", None);
        assert_eq!(body["name"], "test_col");
    }

    #[test]
    fn create_collection_body_with_metadata() {
        let body = create_collection_body("test_col", Some(json!({"hnsw_space": "cosine"})));
        assert_eq!(body["metadata"]["hnsw_space"], "cosine");
    }

    #[test]
    fn add_body_includes_all_fields() {
        let body = add_body(
            &["id1", "id2"],
            &[vec![0.1f32], vec![0.2f32]],
            &[json!({"tag": "a"}), json!({"tag": "b"})],
            Some(&["doc1", "doc2"]),
        );
        assert_eq!(body["ids"].as_array().unwrap().len(), 2);
        assert!(body["documents"].is_array());
    }

    #[test]
    fn query_body_includes_n_results() {
        let body = query_body(&[vec![0.1f32]], 5, None, &["documents", "distances"]);
        assert_eq!(body["n_results"], 5);
    }

    #[test]
    fn query_body_with_filter() {
        let filter = where_eq("tag", json!("ml"));
        let body = query_body(&[vec![0.1f32]], 5, Some(filter), &["distances"]);
        assert!(body["where"].is_object());
    }

    #[test]
    fn where_eq_filter_structure() {
        let f = where_eq("score", json!(5));
        assert!(f["score"]["$eq"] == 5);
    }

    #[test]
    fn where_and_combines_filters() {
        let f = where_and(where_eq("a", json!(1)), where_eq("b", json!(2)));
        assert!(f["$and"].is_array());
        assert_eq!(f["$and"].as_array().unwrap().len(), 2);
    }

    #[test]
    fn where_in_produces_array() {
        let f = where_in("tag", vec![json!("a"), json!("b")]);
        assert!(f["tag"]["$in"].is_array());
    }

    #[test]
    fn parse_query_ids_extracts_nested() {
        let body = json!({ "ids": [["id1", "id2"]] });
        let ids = parse_query_ids(&body);
        assert_eq!(ids.len(), 1);
        assert_eq!(ids[0], vec!["id1", "id2"]);
    }

    #[test]
    fn parse_collection_count_reads_count() {
        let body = json!({ "count": 42 });
        assert_eq!(parse_collection_count(&body), 42);
    }

    #[test]
    fn chroma_error_404_is_not_found() {
        let err = ChromaError::from_response(404, r#"{"error":"collection not found"}"#);
        assert!(matches!(err, ChromaError::NotFound(_)));
    }

    #[test]
    fn chroma_error_409_is_already_exists() {
        let err = ChromaError::from_response(409, "conflict");
        assert!(matches!(err, ChromaError::AlreadyExists(_)));
    }

    #[test]
    fn retry_delay_caps_at_5s() {
        assert!(chroma_retry_delay_ms(20) <= 5_000);
    }
}
