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

/// A point ID that can be either a numeric u64 or a UUID string.
#[derive(Debug, Clone)]
pub enum QdrantPointId {
    Num(u64),
    Uuid(String),
}

impl From<u64> for QdrantPointId {
    fn from(n: u64) -> Self { Self::Num(n) }
}

impl From<&str> for QdrantPointId {
    fn from(s: &str) -> Self { Self::Uuid(s.to_owned()) }
}

impl QdrantPointId {
    pub fn to_json(&self) -> Value {
        match self {
            Self::Num(n) => json!(n),
            Self::Uuid(s) => json!(s),
        }
    }
}

/// Build a upsert body for points with mixed numeric/UUID IDs.
pub fn upsert_body_typed(points: &[(QdrantPointId, Vec<f32>, Value)]) -> Value {
    let pts: Vec<Value> = points.iter().map(|(id, vec, payload)| json!({
        "id": id.to_json(),
        "vector": vec,
        "payload": payload
    })).collect();
    json!({ "points": pts })
}

/// Build a upsert body for points with named vectors.
pub fn upsert_named_vector_body(
    points: &[(u64, &str, Vec<f32>, Value)],
) -> Value {
    let pts: Vec<Value> = points.iter().map(|(id, vec_name, embedding, payload)| {
        let mut vector_map = serde_json::Map::new();
        vector_map.insert(vec_name.to_string(), json!(embedding));
        json!({
            "id": id,
            "vector": serde_json::Value::Object(vector_map),
            "payload": payload
        })
    }).collect();
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

/// Build a search body with a score threshold.
pub fn search_body_with_threshold(
    vector: &[f32],
    top_k: usize,
    score_threshold: f32,
    filter: Option<&Filter>,
) -> Value {
    let mut body = search_body(vector, top_k, filter);
    body["score_threshold"] = json!(score_threshold);
    body
}

/// Build a search body for a named vector within a multi-vector collection.
pub fn search_named_vector_body(
    vector_name: &str,
    vector: &[f32],
    top_k: usize,
    filter: Option<&Filter>,
) -> Value {
    let mut vector_map = serde_json::Map::new();
    vector_map.insert("name".to_owned(), json!(vector_name));
    vector_map.insert("vector".to_owned(), json!(vector));
    let mut body = json!({
        "vector": serde_json::Value::Object(vector_map),
        "limit": top_k,
        "with_payload": true,
        "with_vector": false
    });
    if let Some(f) = filter {
        body["filter"] = filter_to_qdrant(f);
    }
    body
}

/// Build a batch search body (Qdrant `POST /collections/{name}/points/search/batch`).
pub fn batch_search_body(queries: &[(&[f32], usize)]) -> Value {
    let searches: Vec<Value> = queries.iter().map(|(vec, limit)| json!({
        "vector": vec,
        "limit": limit,
        "with_payload": true
    })).collect();
    json!({ "searches": searches })
}

/// URL for batch search endpoint.
pub fn batch_search_url(base: &str, name: &str) -> String {
    format!("{base}/collections/{name}/points/search/batch")
}

/// Build the JSON body for a delete-by-ids request.
pub fn delete_by_ids_body(ids: &[u64]) -> Value {
    json!({ "points": ids })
}

/// Build the JSON body for a delete-by-filter request.
pub fn delete_by_filter_body(filter: &Filter) -> Value {
    json!({ "filter": filter_to_qdrant(filter) })
}

/// Build the JSON body for a delete-by-UUID-ids request.
pub fn delete_by_uuid_ids_body(ids: &[&str]) -> Value {
    json!({ "points": ids })
}

/// URL for the delete endpoint.
pub fn delete_url(base: &str, name: &str) -> String {
    format!("{base}/collections/{name}/points/delete?wait=true")
}

/// URL for the payload index endpoint.
pub fn payload_index_url(base: &str, name: &str) -> String {
    format!("{base}/collections/{name}/index")
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

/// Build a scroll body with a filter for narrowed pagination.
pub fn scroll_body_with_filter(limit: usize, offset: Option<u64>, filter: &Filter) -> Value {
    let mut body = scroll_body(limit, offset);
    body["filter"] = filter_to_qdrant(filter);
    body
}

/// URL for fetching a single point by ID.
pub fn get_point_url(base: &str, name: &str, id: u64) -> String {
    format!("{base}/collections/{name}/points/{id}")
}

/// URL for fetching multiple points by IDs.
pub fn get_points_url(base: &str, name: &str) -> String {
    format!("{base}/collections/{name}/points")
}

/// Build the body for fetching multiple points by IDs.
pub fn get_points_body(ids: &[u64], with_vector: bool) -> Value {
    json!({
        "ids": ids,
        "with_payload": true,
        "with_vector": with_vector
    })
}

/// Parse the `next_page_offset` from a Qdrant scroll response.
pub fn parse_scroll_next_offset(body: &Value) -> Option<u64> {
    body["result"]["next_page_offset"].as_u64()
}

/// Parse point records from a scroll response.
pub fn parse_scroll_points(body: &Value) -> Vec<(u64, Value)> {
    body["result"]["points"]
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .filter_map(|pt| {
            let id = pt["id"].as_u64()?;
            let payload = pt["payload"].clone();
            Some((id, payload))
        })
        .collect()
}

// ---- snapshot and collection management ----------------------------------

/// URL for creating a snapshot of a specific collection.
pub fn create_snapshot_url(base: &str, name: &str) -> String {
    format!("{base}/collections/{name}/snapshots")
}

/// URL for listing all snapshots of a collection.
pub fn list_snapshots_url(base: &str, name: &str) -> String {
    format!("{base}/collections/{name}/snapshots")
}

/// URL for deleting a specific snapshot.
pub fn delete_snapshot_url(base: &str, name: &str, snapshot_name: &str) -> String {
    format!("{base}/collections/{name}/snapshots/{snapshot_name}")
}

/// URL for listing all collections.
pub fn list_collections_url(base: &str) -> String {
    format!("{base}/collections")
}

/// Parse collection names from a Qdrant list-collections response.
pub fn parse_collection_names(body: &serde_json::Value) -> Vec<String> {
    body["result"]["collections"]
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .filter_map(|c| c["name"].as_str().map(|s| s.to_owned()))
        .collect()
}

/// Build the body to recover a collection from a snapshot URL.
pub fn recover_from_snapshot_body(snapshot_url: &str) -> serde_json::Value {
    json!({ "location": snapshot_url })
}

/// URL for the cluster info endpoint (useful for health checks).
pub fn cluster_url(base: &str) -> String {
    format!("{base}/cluster")
}

/// URL for the readiness check endpoint.
pub fn readiness_url(base: &str) -> String {
    format!("{base}/readyz")
}

// ---- hybrid search via full-text and sparse vectors ----------------------

/// URL for the sparse/dense hybrid search endpoint.
pub fn query_url(base: &str, name: &str) -> String {
    format!("{base}/collections/{name}/points/query")
}

/// Build a hybrid query body using the Qdrant `query` endpoint (v1.7+).
///
/// `prefetch` runs a dense vector search first; the outer `query` re-ranks
/// using a keyword sparse model. This enables semantic + keyword fusion.
pub fn hybrid_query_body(
    dense_vector: &[f32],
    sparse_indices: &[u32],
    sparse_values: &[f32],
    top_k: usize,
    alpha: f32,
) -> Value {
    let alpha = alpha.clamp(0.0, 1.0);
    json!({
        "prefetch": [{
            "query": dense_vector,
            "limit": top_k * 2
        }],
        "query": {
            "fusion": "rrf"
        },
        "limit": top_k,
        "with_payload": true,
        "params": {
            "dense_weight": alpha,
            "sparse_weight": 1.0 - alpha
        }
    })
}

/// Build a simple RRF fusion body combining multiple prefetch queries.
pub fn rrf_fusion_body(
    vectors: &[&[f32]],
    top_k: usize,
) -> Value {
    let prefetches: Vec<Value> = vectors.iter().map(|v| json!({
        "query": v,
        "limit": top_k
    })).collect();
    json!({
        "prefetch": prefetches,
        "query": { "fusion": "rrf" },
        "limit": top_k,
        "with_payload": true
    })
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

// ---- error handling ------------------------------------------------------

/// Qdrant error codes returned in the `status` field of error responses.
#[derive(Debug, PartialEq)]
pub enum QdrantError {
    NotFound(String),
    AlreadyExists(String),
    BadRequest(String),
    Unauthorized,
    InternalError(String),
    Unknown(u16, String),
}

impl QdrantError {
    /// Parse a Qdrant error from HTTP status code and response body.
    pub fn from_response(status: u16, body: &str) -> Self {
        let message = parse_error_message(body);
        match status {
            404 => Self::NotFound(message),
            409 => Self::AlreadyExists(message),
            400 | 422 => Self::BadRequest(message),
            401 | 403 => Self::Unauthorized,
            500 => Self::InternalError(message),
            _ => Self::Unknown(status, message),
        }
    }

    pub fn is_transient(&self) -> bool {
        matches!(self, Self::InternalError(_) | Self::Unknown(500..=599, _))
    }
}

fn parse_error_message(body: &str) -> String {
    if let Ok(v) = serde_json::from_str::<serde_json::Value>(body) {
        if let Some(msg) = v["status"]["error"].as_str() {
            return msg.to_owned();
        }
        if let Some(msg) = v["message"].as_str() {
            return msg.to_owned();
        }
    }
    body.chars().take(256).collect()
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

/// Convert a Qdrant JSON payload object to a `Payload` map.
pub fn json_to_payload(obj: &Value) -> crate::vector_store::Payload {
    use crate::vector_store::PayloadValue;
    let mut payload = crate::vector_store::Payload::new();
    if let Some(map) = obj.as_object() {
        for (k, v) in map {
            let pv = match v {
                Value::String(s) => PayloadValue::String(s.clone()),
                Value::Number(n) => {
                    if let Some(i) = n.as_i64() { PayloadValue::Integer(i) }
                    else { PayloadValue::Float(n.as_f64().unwrap_or(0.0)) }
                }
                Value::Bool(b) => PayloadValue::Bool(*b),
                Value::Null => PayloadValue::Null,
                _ => PayloadValue::Null,
            };
            payload.insert(k.clone(), pv);
        }
    }
    payload
}

/// Convert a `Payload` map to a Qdrant JSON payload object.
pub fn payload_to_json(payload: &crate::vector_store::Payload) -> Value {
    use crate::vector_store::PayloadValue;
    let mut obj = serde_json::Map::new();
    for (k, v) in payload {
        let jv = match v {
            PayloadValue::String(s) => Value::String(s.clone()),
            PayloadValue::Integer(n) => json!(n),
            PayloadValue::Float(f) => {
                serde_json::Number::from_f64(*f)
                    .map(Value::Number)
                    .unwrap_or(Value::Null)
            }
            PayloadValue::Bool(b) => Value::Bool(*b),
            PayloadValue::Null => Value::Null,
        };
        obj.insert(k.clone(), jv);
    }
    Value::Object(obj)
}

// ---- tests (all offline) ------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vector_store::{Distance, Filter, PayloadValue};

    #[test]
    fn create_snapshot_url_ends_with_snapshots() {
        let url = create_snapshot_url("http://localhost:6333", "docs");
        assert!(url.ends_with("/snapshots"), "url: {url}");
    }

    #[test]
    fn list_collections_url_format() {
        assert_eq!(list_collections_url("http://localhost:6333"), "http://localhost:6333/collections");
    }

    #[test]
    fn parse_collection_names_extracts_names() {
        let body = serde_json::json!({
            "result": { "collections": [{ "name": "docs" }, { "name": "images" }] }
        });
        let names = parse_collection_names(&body);
        assert_eq!(names, vec!["docs", "images"]);
    }

    #[test]
    fn readiness_url_format() {
        assert_eq!(readiness_url("http://localhost:6333"), "http://localhost:6333/readyz");
    }

    #[test]
    fn query_url_format() {
        let url = query_url("http://localhost:6333", "docs");
        assert!(url.ends_with("/points/query"), "url: {url}");
    }

    #[test]
    fn hybrid_query_body_has_prefetch_and_fusion() {
        let body = hybrid_query_body(&[0.1f32], &[0, 1], &[0.5f32, 0.5], 5, 0.7);
        assert!(body["prefetch"].is_array(), "body: {body}");
        assert_eq!(body["query"]["fusion"], "rrf");
        assert_eq!(body["limit"], 5);
    }

    #[test]
    fn rrf_fusion_body_has_multiple_prefetches() {
        let v1 = vec![0.1f32, 0.2];
        let v2 = vec![0.3f32, 0.4];
        let body = rrf_fusion_body(&[v1.as_slice(), v2.as_slice()], 5);
        assert_eq!(body["prefetch"].as_array().unwrap().len(), 2);
        assert_eq!(body["query"]["fusion"], "rrf");
    }

    #[test]
    fn hybrid_alpha_clamped_in_body() {
        let body = hybrid_query_body(&[0.1f32], &[], &[], 5, 1.5); // clamps to 1.0
        let dw = body["params"]["dense_weight"].as_f64().unwrap();
        assert!((dw - 1.0).abs() < 0.001, "dense_weight: {dw}");
    }

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
    fn qdrant_point_id_num_to_json() {
        let id = QdrantPointId::Num(42);
        assert_eq!(id.to_json(), json!(42u64));
    }

    #[test]
    fn qdrant_point_id_uuid_to_json() {
        let id = QdrantPointId::Uuid("abc-123".to_owned());
        assert_eq!(id.to_json(), json!("abc-123"));
    }

    #[test]
    fn upsert_body_typed_with_uuid_id() {
        let pts = vec![(
            QdrantPointId::Uuid("uuid-1".to_owned()),
            vec![0.1f32],
            json!({}),
        )];
        let body = upsert_body_typed(&pts);
        assert_eq!(body["points"][0]["id"], "uuid-1");
    }

    #[test]
    fn upsert_named_vector_body_nests_vector_under_name() {
        let pts = vec![(1u64, "text", vec![0.1f32, 0.2], json!({}))];
        let body = upsert_named_vector_body(&pts);
        assert!(body["points"][0]["vector"]["text"].is_array());
    }

    #[test]
    fn search_body_has_required_fields() {
        let body = search_body(&[0.1, 0.2], 5, None);
        assert_eq!(body["limit"], 5);
        assert_eq!(body["with_payload"], true);
    }

    #[test]
    fn search_body_with_threshold_has_score_threshold() {
        let body = search_body_with_threshold(&[0.1, 0.2], 5, 0.75, None);
        assert!((body["score_threshold"].as_f64().unwrap() - 0.75).abs() < 0.001);
    }

    #[test]
    fn search_named_vector_body_includes_vector_name() {
        let body = search_named_vector_body("text", &[0.1, 0.2], 5, None);
        assert_eq!(body["vector"]["name"], "text");
        assert!(body["vector"]["vector"].is_array());
    }

    #[test]
    fn batch_search_body_has_searches_array() {
        let body = batch_search_body(&[(&[0.1f32, 0.2], 5), (&[0.3f32], 3)]);
        assert_eq!(body["searches"].as_array().unwrap().len(), 2);
    }

    #[test]
    fn batch_search_url_includes_batch_suffix() {
        let url = batch_search_url("http://localhost:6333", "docs");
        assert!(url.ends_with("/search/batch"), "url: {url}");
    }

    #[test]
    fn delete_by_ids_body_has_points_key() {
        let body = delete_by_ids_body(&[1, 2, 3]);
        assert!(body["points"].is_array());
        assert_eq!(body["points"].as_array().unwrap().len(), 3);
    }

    #[test]
    fn delete_by_filter_body_has_filter_key() {
        let f = Filter::Eq("status".to_owned(), PayloadValue::String("old".to_owned()));
        let body = delete_by_filter_body(&f);
        assert!(body["filter"].is_object(), "body: {body}");
    }

    #[test]
    fn delete_url_has_wait_param() {
        let url = delete_url("http://localhost:6333", "docs");
        assert!(url.contains("wait=true"), "url: {url}");
    }

    #[test]
    fn payload_index_url_format() {
        let url = payload_index_url("http://localhost:6333", "docs");
        assert!(url.ends_with("/index"), "url: {url}");
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
    fn scroll_body_with_filter_has_filter_key() {
        let f = Filter::Eq("tag".to_owned(), PayloadValue::String("news".to_owned()));
        let body = scroll_body_with_filter(20, None, &f);
        assert!(body["filter"].is_object(), "body: {body}");
        assert_eq!(body["limit"], 20);
    }

    #[test]
    fn scroll_body_with_offset_has_offset_field() {
        let body = scroll_body(10, Some(42));
        assert_eq!(body["offset"], 42);
    }

    #[test]
    fn get_points_body_has_ids_array() {
        let body = get_points_body(&[1, 2, 3], false);
        assert!(body["ids"].is_array());
        assert_eq!(body["with_vector"], false);
    }

    #[test]
    fn parse_scroll_next_offset_extracts_value() {
        let body = serde_json::json!({
            "result": { "next_page_offset": 50, "points": [] }
        });
        assert_eq!(parse_scroll_next_offset(&body), Some(50));
    }

    #[test]
    fn parse_scroll_points_extracts_id_and_payload() {
        let body = serde_json::json!({
            "result": {
                "points": [{ "id": 7, "payload": { "text": "hello" } }],
                "next_page_offset": null
            }
        });
        let pts = parse_scroll_points(&body);
        assert_eq!(pts.len(), 1);
        assert_eq!(pts[0].0, 7);
    }

    #[test]
    fn filter_or_produces_should_clause() {
        let f = Filter::Eq("x".to_owned(), PayloadValue::String("a".to_owned()))
            .or(Filter::Eq("x".to_owned(), PayloadValue::String("b".to_owned())));
        let json = filter_to_qdrant(&f);
        assert!(json["should"].is_array(), "json: {json}");
    }

    #[test]
    fn qdrant_error_404_is_not_found() {
        let err = QdrantError::from_response(404, r#"{"status":{"error":"collection not found"}}"#);
        assert!(matches!(err, QdrantError::NotFound(_)));
    }

    #[test]
    fn qdrant_error_409_is_already_exists() {
        let err = QdrantError::from_response(409, "{}");
        assert!(matches!(err, QdrantError::AlreadyExists(_)));
    }

    #[test]
    fn qdrant_error_500_is_transient() {
        let err = QdrantError::from_response(500, "internal");
        assert!(err.is_transient());
    }

    #[test]
    fn qdrant_error_400_is_not_transient() {
        let err = QdrantError::from_response(400, "bad request");
        assert!(!err.is_transient());
    }

    #[test]
    fn qdrant_error_parses_status_error_field() {
        let body = r#"{"status":{"error":"dimension mismatch"}}"#;
        let err = QdrantError::from_response(400, body);
        if let QdrantError::BadRequest(msg) = err {
            assert!(msg.contains("dimension mismatch"), "msg: {msg}");
        } else {
            panic!("expected BadRequest");
        }
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
    fn json_to_payload_converts_all_types() {
        use crate::vector_store::PayloadValue;
        let obj = serde_json::json!({
            "name": "test", "count": 42, "score": 0.5, "active": true, "empty": null
        });
        let p = json_to_payload(&obj);
        assert!(matches!(p.get("name"), Some(PayloadValue::String(_))));
        assert!(matches!(p.get("count"), Some(PayloadValue::Integer(42))));
        assert!(matches!(p.get("active"), Some(PayloadValue::Bool(true))));
        assert!(matches!(p.get("empty"), Some(PayloadValue::Null)));
    }

    #[test]
    fn payload_to_json_round_trips() {
        use crate::vector_store::PayloadValue;
        let mut p = crate::vector_store::Payload::new();
        p.insert("k".to_owned(), PayloadValue::String("v".to_owned()));
        p.insert("n".to_owned(), PayloadValue::Integer(7));
        let json = payload_to_json(&p);
        assert_eq!(json["k"], "v");
        assert_eq!(json["n"], 7);
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
