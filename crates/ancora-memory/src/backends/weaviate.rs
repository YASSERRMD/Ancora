/// Weaviate backend for the `VectorStore` trait.
///
/// Weaviate exposes both REST and GraphQL APIs. This module generates request
/// bodies and URL strings for all relevant operations without requiring a live
/// server during tests.
///
/// Requires the `weaviate` feature: `ancora-memory = { features = ["weaviate"] }`.
///
/// Integration tests require `WEAVIATE_URL` in the environment (e.g.
/// `http://localhost:8080`).

// ---- connection config ---------------------------------------------------

/// Configuration for a Weaviate HTTP connection.
#[derive(Debug, Clone)]
pub struct WeaviateConfig {
    /// Base URL, e.g. `http://localhost:8080`.
    pub url: String,
    /// Weaviate Cloud (WCD) API key.
    pub api_key: Option<String>,
    /// OpenAI API key for Weaviate's text2vec-openai module.
    pub openai_key: Option<String>,
    /// Request timeout in seconds.
    pub timeout_secs: u64,
}

impl WeaviateConfig {
    pub fn new(url: impl Into<String>) -> Self {
        Self { url: url.into(), api_key: None, openai_key: None, timeout_secs: 30 }
    }

    pub fn with_api_key(mut self, key: impl Into<String>) -> Self {
        self.api_key = Some(key.into()); self
    }

    pub fn with_openai_key(mut self, key: impl Into<String>) -> Self {
        self.openai_key = Some(key.into()); self
    }

    pub fn local() -> Self { Self::new("http://localhost:8080") }

    /// Returns auth header value if an API key is configured.
    pub fn auth_header(&self) -> Option<String> {
        self.api_key.as_ref().map(|k| format!("Bearer {k}"))
    }
}

// ---- URL builders --------------------------------------------------------

pub fn schema_url(base: &str) -> String {
    format!("{base}/v1/schema")
}

pub fn class_url(base: &str, class: &str) -> String {
    format!("{base}/v1/schema/{class}")
}

pub fn objects_url(base: &str) -> String {
    format!("{base}/v1/objects")
}

pub fn object_url(base: &str, class: &str, id: &str) -> String {
    format!("{base}/v1/objects/{class}/{id}")
}

pub fn graphql_url(base: &str) -> String {
    format!("{base}/v1/graphql")
}

pub fn batch_objects_url(base: &str) -> String {
    format!("{base}/v1/batch/objects")
}

pub fn batch_delete_url(base: &str) -> String {
    format!("{base}/v1/batch/objects")
}

pub fn readiness_url(base: &str) -> String {
    format!("{base}/v1/.well-known/ready")
}

pub fn liveness_url(base: &str) -> String {
    format!("{base}/v1/.well-known/live")
}

pub fn meta_url(base: &str) -> String {
    format!("{base}/v1/meta")
}

// ---- schema helpers ------------------------------------------------------

use serde_json::{json, Value};

/// Data type strings used in Weaviate property definitions.
pub mod data_type {
    pub const TEXT: &str = "text";
    pub const TEXT_ARRAY: &str = "text[]";
    pub const INT: &str = "int";
    pub const NUMBER: &str = "number";
    pub const BOOLEAN: &str = "boolean";
    pub const DATE: &str = "date";
}

/// Build the JSON body for creating a Weaviate class (schema definition).
///
/// `class_name` must start with an uppercase letter (Weaviate convention).
pub fn create_class_body(
    class_name: &str,
    description: &str,
    vectorizer: &str,
) -> Value {
    json!({
        "class": class_name,
        "description": description,
        "vectorizer": vectorizer,
        "properties": []
    })
}

/// Build the JSON body for creating a class with explicit properties.
pub fn create_class_with_properties_body(
    class_name: &str,
    description: &str,
    vectorizer: &str,
    properties: &[(&str, &str, &str)], // (name, data_type, description)
) -> Value {
    let props: Vec<Value> = properties.iter().map(|(name, dtype, desc)| json!({
        "name": name,
        "description": desc,
        "dataType": [dtype]
    })).collect();
    json!({
        "class": class_name,
        "description": description,
        "vectorizer": vectorizer,
        "properties": props
    })
}

/// Build the body for adding a property to an existing class.
pub fn add_property_body(name: &str, data_type: &str, description: &str) -> Value {
    json!({
        "name": name,
        "description": description,
        "dataType": [data_type]
    })
}

/// URL for adding a property to an existing class.
pub fn add_property_url(base: &str, class: &str) -> String {
    format!("{base}/v1/schema/{class}/properties")
}

// ---- object CRUD ---------------------------------------------------------

/// Build the JSON body for creating a single object.
pub fn create_object_body(class: &str, properties: &Value, vector: Option<&[f32]>) -> Value {
    let mut body = json!({
        "class": class,
        "properties": properties
    });
    if let Some(v) = vector {
        body["vector"] = json!(v);
    }
    body
}

/// Build the JSON body for upserting an object by ID.
///
/// Weaviate uses `PUT /v1/objects/{class}/{id}` for upsert semantics.
pub fn upsert_object_body(class: &str, properties: &Value, vector: Option<&[f32]>) -> Value {
    create_object_body(class, properties, vector)
}

// ---- batch upsert --------------------------------------------------------

/// Build the batch objects body for `POST /v1/batch/objects`.
pub fn batch_objects_body(objects: &[(String, Value, Option<Vec<f32>>)]) -> Value {
    let objs: Vec<Value> = objects.iter().map(|(class, props, vec)| {
        let mut obj = json!({ "class": class, "properties": props });
        if let Some(v) = vec { obj["vector"] = json!(v); }
        obj
    }).collect();
    json!({ "objects": objs })
}

/// Build the batch delete body for filter-based deletion.
pub fn batch_delete_body(class: &str, filter: &Value) -> Value {
    json!({
        "match": {
            "class": class,
            "where": filter
        }
    })
}

// ---- GraphQL near-vector search ------------------------------------------

/// Build a GraphQL query for near-vector search.
///
/// `fields` is the list of property names to return, e.g. `["title", "body"]`.
pub fn graphql_near_vector_query(
    class: &str,
    vector: &[f32],
    limit: usize,
    fields: &[&str],
) -> Value {
    let vec_str = format!("[{}]", vector.iter().map(|v| format!("{v}")).collect::<Vec<_>>().join(","));
    let field_str = fields.join(" ");
    let query = format!(
        r#"{{ Get {{ {class}(nearVector: {{ vector: {vec_str} }} limit: {limit}) {{ {field_str} _additional {{ id distance }} }} }} }}"#
    );
    json!({ "query": query })
}

/// Build a GraphQL query with a certainty threshold.
pub fn graphql_near_vector_with_certainty_query(
    class: &str,
    vector: &[f32],
    limit: usize,
    certainty: f32,
    fields: &[&str],
) -> Value {
    let vec_str = format!("[{}]", vector.iter().map(|v| format!("{v}")).collect::<Vec<_>>().join(","));
    let field_str = fields.join(" ");
    let query = format!(
        r#"{{ Get {{ {class}(nearVector: {{ vector: {vec_str} certainty: {certainty} }} limit: {limit}) {{ {field_str} _additional {{ id distance }} }} }} }}"#
    );
    json!({ "query": query })
}

// ---- GraphQL BM25 hybrid search -----------------------------------------

/// Build a GraphQL BM25 keyword search query.
pub fn graphql_bm25_query(class: &str, query_text: &str, limit: usize, fields: &[&str]) -> Value {
    let field_str = fields.join(" ");
    let q = format!(
        r#"{{ Get {{ {class}(bm25: {{ query: "{query_text}" }} limit: {limit}) {{ {field_str} _additional {{ id score }} }} }} }}"#
    );
    json!({ "query": q })
}

/// Build a GraphQL hybrid (vector + BM25) search query.
pub fn graphql_hybrid_query(
    class: &str,
    query_text: &str,
    vector: Option<&[f32]>,
    alpha: f32,
    limit: usize,
    fields: &[&str],
) -> Value {
    let alpha = alpha.clamp(0.0, 1.0);
    let field_str = fields.join(" ");
    let vector_part = if let Some(v) = vector {
        let vs = format!("[{}]", v.iter().map(|x| format!("{x}")).collect::<Vec<_>>().join(","));
        format!(", vector: {vs}")
    } else {
        String::new()
    };
    let q = format!(
        r#"{{ Get {{ {class}(hybrid: {{ query: "{query_text}" alpha: {alpha}{vector_part} }} limit: {limit}) {{ {field_str} _additional {{ id score }} }} }} }}"#
    );
    json!({ "query": q })
}

// ---- GraphQL where filter ------------------------------------------------

/// Build a Weaviate `where` filter value operand.
pub fn where_filter_text(path: &str, operator: &str, value_text: &str) -> Value {
    json!({
        "path": [path],
        "operator": operator,
        "valueText": value_text
    })
}

pub fn where_filter_int(path: &str, operator: &str, value: i64) -> Value {
    json!({
        "path": [path],
        "operator": operator,
        "valueInt": value
    })
}

pub fn where_filter_bool(path: &str, value: bool) -> Value {
    json!({
        "path": [path],
        "operator": "Equal",
        "valueBoolean": value
    })
}

pub fn where_filter_and(operands: &[Value]) -> Value {
    json!({ "operator": "And", "operands": operands })
}

pub fn where_filter_or(operands: &[Value]) -> Value {
    json!({ "operator": "Or", "operands": operands })
}

// ---- response parsing ----------------------------------------------------

/// Extract object UUIDs and their payloads from a `GET /v1/objects` list response.
pub fn parse_objects_list(body: &Value) -> Vec<(String, Value)> {
    body["objects"]
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .filter_map(|obj| {
            let id = obj["id"].as_str()?.to_owned();
            let props = obj["properties"].clone();
            Some((id, props))
        })
        .collect()
}

/// Extract the class vector count from a `GET /v1/schema/{class}` response.
pub fn parse_class_object_count(body: &Value) -> Option<u64> {
    body["objectCount"].as_u64()
}

/// Extract GraphQL Get results from a Weaviate GraphQL response.
pub fn parse_graphql_get(body: &Value, class: &str) -> Vec<Value> {
    body["data"]["Get"][class]
        .as_array()
        .unwrap_or(&vec![])
        .to_vec()
}

// ---- error classification ------------------------------------------------

#[derive(Debug, PartialEq)]
pub enum WeaviateError {
    NotFound(String),
    AlreadyExists(String),
    BadRequest(String),
    Unauthorized,
    InternalError(String),
    Unknown(u16, String),
}

impl WeaviateError {
    pub fn from_response(status: u16, body: &str) -> Self {
        let message = parse_error_body(body);
        match status {
            404 => Self::NotFound(message),
            409 | 422 => Self::AlreadyExists(message),
            400 => Self::BadRequest(message),
            401 | 403 => Self::Unauthorized,
            500 => Self::InternalError(message),
            _ => Self::Unknown(status, message),
        }
    }

    pub fn is_transient(&self) -> bool {
        matches!(self, Self::InternalError(_) | Self::Unknown(500..=599, _))
    }
}

fn parse_error_body(body: &str) -> String {
    if let Ok(v) = serde_json::from_str::<Value>(body) {
        if let Some(errs) = v["error"].as_array() {
            if let Some(msg) = errs.first().and_then(|e| e["message"].as_str()) {
                return msg.to_owned();
            }
        }
        if let Some(msg) = v["message"].as_str() {
            return msg.to_owned();
        }
    }
    body.chars().take(256).collect()
}

// ---- retry policy --------------------------------------------------------

pub const MAX_RETRIES: u32 = 4;

pub fn weaviate_retry_delay_ms(attempt: u32) -> u64 {
    let base: u64 = 150;
    let cap: u64 = 12_000;
    base.saturating_mul(2u64.saturating_pow(attempt)).min(cap)
}

pub fn should_retry_status(status: u16) -> bool {
    matches!(status, 429 | 500 | 502 | 503 | 504)
}

// ---- tests (all offline) ------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn weaviate_config_local_url() {
        let cfg = WeaviateConfig::local();
        assert_eq!(cfg.url, "http://localhost:8080");
    }

    #[test]
    fn weaviate_config_auth_header_with_key() {
        let cfg = WeaviateConfig::new("http://localhost:8080").with_api_key("wcs-key");
        assert!(cfg.auth_header().unwrap().contains("wcs-key"));
    }

    #[test]
    fn weaviate_config_auth_header_without_key() {
        assert!(WeaviateConfig::local().auth_header().is_none());
    }

    #[test]
    fn schema_url_format() {
        assert!(schema_url("http://localhost:8080").ends_with("/v1/schema"));
    }

    #[test]
    fn class_url_includes_class_name() {
        let url = class_url("http://localhost:8080", "Document");
        assert!(url.ends_with("/Document"), "url: {url}");
    }

    #[test]
    fn graphql_url_format() {
        assert!(graphql_url("http://localhost:8080").ends_with("/v1/graphql"));
    }

    #[test]
    fn create_class_body_has_required_fields() {
        let body = create_class_body("Document", "Test class", "none");
        assert_eq!(body["class"], "Document");
        assert_eq!(body["vectorizer"], "none");
        assert!(body["properties"].is_array());
    }

    #[test]
    fn create_class_with_properties_has_all_props() {
        let body = create_class_with_properties_body(
            "Document", "test", "none",
            &[("title", data_type::TEXT, "The title"), ("year", data_type::INT, "Year")],
        );
        let props = body["properties"].as_array().unwrap();
        assert_eq!(props.len(), 2);
        assert_eq!(props[0]["name"], "title");
    }

    #[test]
    fn create_object_body_with_vector() {
        let props = json!({"title": "hello"});
        let body = create_object_body("Document", &props, Some(&[0.1f32, 0.2]));
        assert_eq!(body["class"], "Document");
        assert!(body["vector"].is_array());
    }

    #[test]
    fn create_object_body_without_vector() {
        let props = json!({"title": "hello"});
        let body = create_object_body("Document", &props, None);
        assert!(body["vector"].is_null());
    }

    #[test]
    fn batch_objects_body_contains_all_objects() {
        let objects = vec![
            ("Document".to_owned(), json!({"title": "a"}), Some(vec![0.1f32])),
            ("Document".to_owned(), json!({"title": "b"}), None),
        ];
        let body = batch_objects_body(&objects);
        assert_eq!(body["objects"].as_array().unwrap().len(), 2);
    }

    #[test]
    fn graphql_near_vector_query_contains_class_and_limit() {
        let body = graphql_near_vector_query("Document", &[0.1f32, 0.2], 5, &["title"]);
        let q = body["query"].as_str().unwrap();
        assert!(q.contains("Document"), "query: {q}");
        assert!(q.contains("limit: 5"), "query: {q}");
        assert!(q.contains("nearVector"), "query: {q}");
    }

    #[test]
    fn graphql_hybrid_query_contains_alpha() {
        let body = graphql_hybrid_query("Document", "search", None, 0.7, 5, &["title"]);
        let q = body["query"].as_str().unwrap();
        assert!(q.contains("hybrid"), "query: {q}");
        assert!(q.contains("0.7"), "query: {q}");
    }

    #[test]
    fn graphql_bm25_query_has_bm25() {
        let body = graphql_bm25_query("Document", "machine learning", 5, &["title"]);
        let q = body["query"].as_str().unwrap();
        assert!(q.contains("bm25"), "query: {q}");
        assert!(q.contains("machine learning"), "query: {q}");
    }

    #[test]
    fn where_filter_text_has_correct_structure() {
        let f = where_filter_text("source", "Equal", "wiki");
        assert_eq!(f["path"][0], "source");
        assert_eq!(f["operator"], "Equal");
        assert_eq!(f["valueText"], "wiki");
    }

    #[test]
    fn where_filter_and_wraps_operands() {
        let f1 = where_filter_text("lang", "Equal", "en");
        let f2 = where_filter_int("year", "GreaterThan", 2020);
        let compound = where_filter_and(&[f1, f2]);
        assert_eq!(compound["operator"], "And");
        assert_eq!(compound["operands"].as_array().unwrap().len(), 2);
    }

    #[test]
    fn parse_graphql_get_extracts_results() {
        let body = json!({
            "data": { "Get": { "Document": [{ "title": "hello", "_additional": { "id": "abc" } }] } }
        });
        let results = parse_graphql_get(&body, "Document");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0]["title"], "hello");
    }

    #[test]
    fn parse_objects_list_extracts_id_and_props() {
        let body = json!({
            "objects": [{ "id": "uuid-1", "properties": { "title": "test" } }]
        });
        let objs = parse_objects_list(&body);
        assert_eq!(objs.len(), 1);
        assert_eq!(objs[0].0, "uuid-1");
    }

    #[test]
    fn weaviate_error_404_is_not_found() {
        let err = WeaviateError::from_response(404, r#"{"error":[{"message":"class not found"}]}"#);
        assert!(matches!(err, WeaviateError::NotFound(_)));
    }

    #[test]
    fn weaviate_error_500_is_transient() {
        let err = WeaviateError::from_response(500, "error");
        assert!(err.is_transient());
    }

    #[test]
    fn retry_delay_caps_at_12s() {
        assert_eq!(weaviate_retry_delay_ms(100), 12_000);
    }

    #[test]
    fn retry_delay_exponential_for_small_n() {
        assert_eq!(weaviate_retry_delay_ms(0), 150);
        assert_eq!(weaviate_retry_delay_ms(1), 300);
    }

    #[test]
    fn should_retry_429_and_503() {
        assert!(should_retry_status(429));
        assert!(should_retry_status(503));
        assert!(!should_retry_status(400));
        assert!(!should_retry_status(404));
    }
}
