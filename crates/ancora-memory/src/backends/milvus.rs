/// Milvus backend for the `VectorStore` trait.
///
/// Milvus is a distributed, cloud-native vector database designed for
/// billion-scale similarity search. This module generates request bodies and
/// URL strings for the Milvus REST API v2 without requiring a live server.
///
/// Requires the `milvus` feature: `ancora-memory = { features = ["milvus"] }`.
///
/// Integration tests require `MILVUS_URL` in the environment.
use serde_json::{json, Value};

// ---- connection config ---------------------------------------------------

/// Configuration for a Milvus REST connection.
#[derive(Debug, Clone)]
pub struct MilvusConfig {
    /// Base URL, e.g. `http://localhost:19530`.
    pub url: String,
    /// Optional API key (Zilliz Cloud or token-based auth).
    pub api_key: Option<String>,
    /// Database name (default is `"default"`).
    pub database: String,
    /// Request timeout in seconds.
    pub timeout_secs: u64,
}

impl MilvusConfig {
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            api_key: None,
            database: "default".to_owned(),
            timeout_secs: 30,
        }
    }

    pub fn with_api_key(mut self, key: impl Into<String>) -> Self {
        self.api_key = Some(key.into());
        self
    }

    pub fn with_database(mut self, db: impl Into<String>) -> Self {
        self.database = db.into();
        self
    }

    /// Returns `Authorization: Bearer <key>` header value, if set.
    pub fn auth_header(&self) -> Option<String> {
        self.api_key.as_ref().map(|k| format!("Bearer {k}"))
    }

    /// Returns a config pointed at `http://localhost:19530` with the default database.
    pub fn local() -> Self {
        Self::new("http://localhost:19530")
    }
}

// ---- URL builders --------------------------------------------------------

pub fn collections_url(base: &str) -> String {
    format!("{base}/v2/vectordb/collections")
}
pub fn collection_load_url(base: &str) -> String {
    format!("{base}/v2/vectordb/collections/load")
}
pub fn collection_release_url(base: &str) -> String {
    format!("{base}/v2/vectordb/collections/release")
}
pub fn collection_describe_url(base: &str) -> String {
    format!("{base}/v2/vectordb/collections/describe")
}
pub fn collection_stats_url(base: &str) -> String {
    format!("{base}/v2/vectordb/collections/get_stats")
}
pub fn collection_drop_url(base: &str) -> String {
    format!("{base}/v2/vectordb/collections/drop")
}
pub fn entities_insert_url(base: &str) -> String {
    format!("{base}/v2/vectordb/entities/insert")
}
pub fn entities_upsert_url(base: &str) -> String {
    format!("{base}/v2/vectordb/entities/upsert")
}
pub fn entities_delete_url(base: &str) -> String {
    format!("{base}/v2/vectordb/entities/delete")
}
pub fn entities_query_url(base: &str) -> String {
    format!("{base}/v2/vectordb/entities/query")
}
pub fn entities_search_url(base: &str) -> String {
    format!("{base}/v2/vectordb/entities/search")
}
pub fn entities_hybrid_url(base: &str) -> String {
    format!("{base}/v2/vectordb/entities/hybrid_search")
}
pub fn partitions_url(base: &str) -> String {
    format!("{base}/v2/vectordb/partitions")
}
pub fn partition_load_url(base: &str) -> String {
    format!("{base}/v2/vectordb/partitions/load")
}
pub fn partition_release_url(base: &str) -> String {
    format!("{base}/v2/vectordb/partitions/release")
}
pub fn indexes_url(base: &str) -> String {
    format!("{base}/v2/vectordb/indexes")
}
pub fn index_drop_url(base: &str) -> String {
    format!("{base}/v2/vectordb/indexes/drop")
}
pub fn aliases_url(base: &str) -> String {
    format!("{base}/v2/vectordb/aliases")
}
pub fn alias_drop_url(base: &str) -> String {
    format!("{base}/v2/vectordb/aliases/drop")
}
pub fn health_url(base: &str) -> String {
    format!("{base}/healthz")
}

// ---- field-type constants ------------------------------------------------

pub mod field_type {
    pub const INT64: &str = "Int64";
    pub const VARCHAR: &str = "VarChar";
    pub const FLOAT_VECTOR: &str = "FloatVector";
    pub const BOOL: &str = "Bool";
    pub const FLOAT: &str = "Float";
    pub const DOUBLE: &str = "Double";
    pub const INT32: &str = "Int32";
}

// ---- metric-type constants -----------------------------------------------

pub mod metric_type {
    pub const COSINE: &str = "COSINE";
    pub const IP: &str = "IP";
    pub const L2: &str = "L2";
}

// ---- index-type constants ------------------------------------------------

pub mod index_type {
    pub const HNSW: &str = "HNSW";
    pub const IVF_FLAT: &str = "IVF_FLAT";
    pub const IVF_SQ8: &str = "IVF_SQ8";
    pub const FLAT: &str = "FLAT";
    pub const DISKANN: &str = "DISKANN";
    pub const AUTOINDEX: &str = "AUTOINDEX";
}

// ---- consistency level ---------------------------------------------------

pub mod consistency {
    pub const STRONG: &str = "Strong";
    pub const BOUNDED: &str = "Bounded";
    pub const SESSION: &str = "Session";
    pub const EVENTUALLY: &str = "Eventually";
}

// ---- collection schema builders ------------------------------------------

/// Build a simple collection schema with a primary key, an embedding field, and optional metadata.
pub fn collection_schema(
    collection: &str,
    dims: usize,
    metric: &str,
    extra_fields: &[(&str, &str)],
) -> Value {
    let mut fields = vec![
        json!({ "fieldName": "id", "dataType": field_type::INT64, "isPrimary": true, "autoID": true }),
        json!({ "fieldName": "embedding", "dataType": field_type::FLOAT_VECTOR, "elementTypeParams": { "dim": dims } }),
        json!({ "fieldName": "payload", "dataType": field_type::VARCHAR, "elementTypeParams": { "max_length": 65535 } }),
    ];
    for (name, dtype) in extra_fields {
        fields.push(json!({ "fieldName": name, "dataType": dtype }));
    }
    json!({
        "collectionName": collection,
        "schema": {
            "autoID": true,
            "enableDynamicField": true,
            "fields": fields,
        },
        "indexParams": [
            { "metricType": metric, "fieldName": "embedding", "indexName": "embedding_idx" }
        ]
    })
}

/// Build a create-collection request body.
pub fn create_collection_body(collection: &str, dims: usize, metric: &str) -> Value {
    collection_schema(collection, dims, metric, &[])
}

/// Build a create-collection body with HNSW index parameters.
pub fn create_collection_hnsw_body(
    collection: &str,
    dims: usize,
    metric: &str,
    m: u16,
    ef_construction: u16,
) -> Value {
    json!({
        "collectionName": collection,
        "schema": {
            "autoID": true,
            "enableDynamicField": true,
            "fields": [
                { "fieldName": "id", "dataType": field_type::INT64, "isPrimary": true, "autoID": true },
                { "fieldName": "embedding", "dataType": field_type::FLOAT_VECTOR, "elementTypeParams": { "dim": dims } },
                { "fieldName": "payload", "dataType": field_type::VARCHAR, "elementTypeParams": { "max_length": 65535 } },
            ]
        },
        "indexParams": [{
            "metricType": metric,
            "fieldName": "embedding",
            "indexName": "embedding_hnsw",
            "indexType": index_type::HNSW,
            "params": { "M": m, "efConstruction": ef_construction }
        }]
    })
}

/// Build a create-collection body with IVF_FLAT index parameters.
pub fn create_collection_ivf_body(
    collection: &str,
    dims: usize,
    metric: &str,
    nlist: u32,
) -> Value {
    json!({
        "collectionName": collection,
        "schema": {
            "autoID": true,
            "enableDynamicField": true,
            "fields": [
                { "fieldName": "id", "dataType": field_type::INT64, "isPrimary": true, "autoID": true },
                { "fieldName": "embedding", "dataType": field_type::FLOAT_VECTOR, "elementTypeParams": { "dim": dims } },
                { "fieldName": "payload", "dataType": field_type::VARCHAR, "elementTypeParams": { "max_length": 65535 } },
            ]
        },
        "indexParams": [{
            "metricType": metric,
            "fieldName": "embedding",
            "indexName": "embedding_ivf",
            "indexType": index_type::IVF_FLAT,
            "params": { "nlist": nlist }
        }]
    })
}

// ---- load / release -----------------------------------------------------

pub fn load_collection_body(collection: &str) -> Value {
    json!({ "collectionName": collection })
}

pub fn release_collection_body(collection: &str) -> Value {
    json!({ "collectionName": collection })
}

pub fn drop_collection_body(collection: &str) -> Value {
    json!({ "collectionName": collection })
}

// ---- insert / upsert -----------------------------------------------------

/// Build an insert request body from a list of `(embedding, payload_json)` pairs.
pub fn insert_entities_body(collection: &str, entities: &[(Vec<f32>, Value)]) -> Value {
    let data: Vec<Value> = entities
        .iter()
        .map(|(embedding, payload)| json!({ "embedding": embedding, "payload": payload.to_string() }))
        .collect();
    json!({ "collectionName": collection, "data": data })
}

/// Build an insert body that targets a specific partition.
pub fn insert_into_partition_body(
    collection: &str,
    partition: &str,
    entities: &[(Vec<f32>, Value)],
) -> Value {
    let data: Vec<Value> = entities
        .iter()
        .map(|(embedding, payload)| json!({ "embedding": embedding, "payload": payload.to_string() }))
        .collect();
    json!({ "collectionName": collection, "partitionName": partition, "data": data })
}

/// Build a batch insert body accepting a flat list of entity objects.
pub fn batch_insert_body(collection: &str, entities: Vec<Value>) -> Value {
    json!({ "collectionName": collection, "data": entities })
}

// ---- search --------------------------------------------------------------

/// Build a vector search request body.
pub fn search_body(
    collection: &str,
    vector: &[f32],
    top_k: usize,
    metric: &str,
    output_fields: &[&str],
) -> Value {
    json!({
        "collectionName": collection,
        "data": [vector],
        "annsField": "embedding",
        "limit": top_k,
        "outputFields": output_fields,
        "searchParams": { "metric_type": metric, "params": {} }
    })
}

/// Build a vector search body with a boolean expression filter.
pub fn search_with_filter_body(
    collection: &str,
    vector: &[f32],
    top_k: usize,
    metric: &str,
    filter_expr: &str,
    output_fields: &[&str],
) -> Value {
    json!({
        "collectionName": collection,
        "data": [vector],
        "annsField": "embedding",
        "limit": top_k,
        "filter": filter_expr,
        "outputFields": output_fields,
        "searchParams": { "metric_type": metric, "params": {} }
    })
}

/// Build a search body with HNSW ef search param.
pub fn search_hnsw_body(
    collection: &str,
    vector: &[f32],
    top_k: usize,
    metric: &str,
    ef: u32,
) -> Value {
    json!({
        "collectionName": collection,
        "data": [vector],
        "annsField": "embedding",
        "limit": top_k,
        "outputFields": ["payload"],
        "searchParams": { "metric_type": metric, "params": { "ef": ef } }
    })
}

/// Build a search body targeting a specific partition.
pub fn search_partition_body(
    collection: &str,
    partition: &str,
    vector: &[f32],
    top_k: usize,
    metric: &str,
) -> Value {
    json!({
        "collectionName": collection,
        "partitionNames": [partition],
        "data": [vector],
        "annsField": "embedding",
        "limit": top_k,
        "outputFields": ["payload"],
        "searchParams": { "metric_type": metric, "params": {} }
    })
}

/// Build a search body with a consistency-level override.
pub fn search_with_consistency_body(
    collection: &str,
    vector: &[f32],
    top_k: usize,
    metric: &str,
    consistency: &str,
) -> Value {
    json!({
        "collectionName": collection,
        "data": [vector],
        "annsField": "embedding",
        "limit": top_k,
        "outputFields": ["payload"],
        "searchParams": { "metric_type": metric, "params": {} },
        "consistencyLevel": consistency,
    })
}

// ---- query (scalar/filter-only) ------------------------------------------

/// Build a query body with a boolean expression filter (no vector).
pub fn query_body(collection: &str, filter_expr: &str, output_fields: &[&str]) -> Value {
    json!({
        "collectionName": collection,
        "filter": filter_expr,
        "outputFields": output_fields,
    })
}

/// Build a query body that returns up to `limit` rows from a partition.
pub fn query_partition_body(
    collection: &str,
    partition: &str,
    filter_expr: &str,
    limit: usize,
) -> Value {
    json!({
        "collectionName": collection,
        "partitionNames": [partition],
        "filter": filter_expr,
        "limit": limit,
        "outputFields": ["id", "payload"],
    })
}

// ---- delete --------------------------------------------------------------

pub fn delete_by_expr_body(collection: &str, filter_expr: &str) -> Value {
    json!({ "collectionName": collection, "filter": filter_expr })
}

pub fn delete_by_ids_body(collection: &str, ids: &[i64]) -> Value {
    let expr = format!(
        "id in [{}]",
        ids.iter()
            .map(|id| id.to_string())
            .collect::<Vec<_>>()
            .join(",")
    );
    json!({ "collectionName": collection, "filter": expr })
}

// ---- hybrid search -------------------------------------------------------

/// Build a hybrid search body that fuses two ANN requests with RRF.
pub fn hybrid_search_body(
    collection: &str,
    dense_vector: &[f32],
    sparse_field: &str,
    sparse_vector: &[(u32, f32)],
    top_k: usize,
    metric: &str,
) -> Value {
    let sparse_indices: Vec<u32> = sparse_vector.iter().map(|(i, _)| *i).collect();
    let sparse_values: Vec<f32> = sparse_vector.iter().map(|(_, v)| *v).collect();
    json!({
        "collectionName": collection,
        "search": [
            {
                "data": [dense_vector],
                "annsField": "embedding",
                "limit": top_k,
                "searchParams": { "metric_type": metric, "params": {} }
            },
            {
                "data": [{ "indices": sparse_indices, "values": sparse_values }],
                "annsField": sparse_field,
                "limit": top_k,
                "searchParams": { "metric_type": "IP", "params": {} }
            }
        ],
        "rerank": { "strategy": "rrf", "params": { "k": 60 } },
        "limit": top_k,
        "outputFields": ["payload"],
    })
}

// ---- partition management ------------------------------------------------

pub fn create_partition_body(collection: &str, partition: &str) -> Value {
    json!({ "collectionName": collection, "partitionName": partition })
}

pub fn drop_partition_body(collection: &str, partition: &str) -> Value {
    json!({ "collectionName": collection, "partitionName": partition })
}

pub fn load_partition_body(collection: &str, partitions: &[&str]) -> Value {
    json!({ "collectionName": collection, "partitionNames": partitions })
}

pub fn release_partition_body(collection: &str, partitions: &[&str]) -> Value {
    json!({ "collectionName": collection, "partitionNames": partitions })
}

// ---- index management ----------------------------------------------------

pub fn create_index_body(
    collection: &str,
    field: &str,
    index_name: &str,
    index_kind: &str,
    metric: &str,
) -> Value {
    json!({
        "collectionName": collection,
        "indexParams": [{
            "fieldName": field,
            "indexName": index_name,
            "indexType": index_kind,
            "metricType": metric,
        }]
    })
}

pub fn drop_index_body(collection: &str, index_name: &str) -> Value {
    json!({ "collectionName": collection, "indexName": index_name })
}

// ---- alias management ----------------------------------------------------

pub fn create_alias_body(collection: &str, alias: &str) -> Value {
    json!({ "collectionName": collection, "aliasName": alias })
}

pub fn drop_alias_body(alias: &str) -> Value {
    json!({ "aliasName": alias })
}

// ---- boolean expression builder ------------------------------------------

/// Escape a string value for use in a Milvus boolean expression.
pub fn escape_string(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

/// `field == "value"` expression.
pub fn expr_eq_str(field: &str, value: &str) -> String {
    format!(r#"{field} == "{}""#, escape_string(value))
}

/// `field == value` expression for integers.
pub fn expr_eq_int(field: &str, value: i64) -> String {
    format!("{field} == {value}")
}

/// `field > value` expression.
pub fn expr_gt(field: &str, value: i64) -> String {
    format!("{field} > {value}")
}

/// `field < value` expression.
pub fn expr_lt(field: &str, value: i64) -> String {
    format!("{field} < {value}")
}

/// `field in [v1, v2, ...]` expression for integers.
pub fn expr_in_ints(field: &str, values: &[i64]) -> String {
    let list = values
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<_>>()
        .join(", ");
    format!("{field} in [{list}]")
}

/// `(expr_a) and (expr_b)` combinator.
pub fn expr_and(a: &str, b: &str) -> String {
    format!("({a}) and ({b})")
}

/// `(expr_a) or (expr_b)` combinator.
pub fn expr_or(a: &str, b: &str) -> String {
    format!("({a}) or ({b})")
}

// ---- range and not-equal expression helpers -----------------------------

/// `field >= lo and field <= hi` range expression.
pub fn expr_range(field: &str, lo: i64, hi: i64) -> String {
    format!("{field} >= {lo} and {field} <= {hi}")
}

/// `field != value` not-equal expression for integers.
pub fn expr_ne_int(field: &str, value: i64) -> String {
    format!("{field} != {value}")
}

/// `field != "value"` not-equal expression for strings.
pub fn expr_ne_str(field: &str, value: &str) -> String {
    format!(r#"{field} != "{}""#, escape_string(value))
}

/// `not (expr)` negation wrapper.
pub fn expr_not(inner: &str) -> String {
    format!("not ({inner})")
}

// ---- response parsing ----------------------------------------------------

/// Extract result rows from a search or query response.
pub fn parse_search_results(body: &Value) -> Vec<(i64, f32, Value)> {
    body["data"]
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .map(|hit| {
            let id = hit["id"].as_i64().unwrap_or(0);
            let score = hit["distance"].as_f64().unwrap_or(0.0) as f32;
            let payload = hit["payload"]
                .as_str()
                .and_then(|s| serde_json::from_str(s).ok())
                .unwrap_or(json!({}));
            (id, score, payload)
        })
        .collect()
}

/// Extract query result rows (no score).
pub fn parse_query_results(body: &Value) -> Vec<(i64, Value)> {
    body["data"]
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .map(|row| {
            let id = row["id"].as_i64().unwrap_or(0);
            let payload = row["payload"]
                .as_str()
                .and_then(|s| serde_json::from_str(s).ok())
                .unwrap_or(json!({}));
            (id, payload)
        })
        .collect()
}

/// Extract the row count from a `get_stats` response.
pub fn parse_collection_stats(body: &Value) -> u64 {
    body["data"]["rowCount"].as_u64().unwrap_or(0)
}

/// Extract error message from a Milvus REST error response body.
pub fn parse_error_message(body: &Value) -> Option<String> {
    body["message"].as_str().map(|s| s.to_owned())
}

/// Extract the delete count from a delete response.
pub fn parse_delete_count(body: &Value) -> u64 {
    body["data"]["deleteCount"].as_u64().unwrap_or(0)
}

/// Extract alias names from a list-aliases response.
pub fn parse_alias_names(body: &Value) -> Vec<String> {
    body["data"]
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .filter_map(|a| a["aliasName"].as_str().map(|s| s.to_owned()))
        .collect()
}

/// Extract inserted primary-key IDs from an insert response.
pub fn parse_insert_ids(body: &Value) -> Vec<i64> {
    body["data"]["insertIds"]
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .filter_map(|v| v.as_i64())
        .collect()
}

/// Extract partition names from a list-partitions response.
pub fn parse_partition_names(body: &Value) -> Vec<String> {
    body["data"]
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .filter_map(|p| p["partitionName"].as_str().map(|s| s.to_owned()))
        .collect()
}

// ---- error handling ------------------------------------------------------

#[derive(Debug, PartialEq)]
pub enum MilvusError {
    NotFound(String),
    AlreadyExists(String),
    BadRequest(String),
    Unauthorized,
    Overloaded,
    InternalError(String),
    Unknown(u16, String),
}

impl MilvusError {
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
            429 | 503 => Self::Overloaded,
            500..=599 => Self::InternalError(msg),
            _ => Self::Unknown(status, msg),
        }
    }

    pub fn is_transient(&self) -> bool {
        matches!(self, Self::InternalError(_) | Self::Overloaded)
    }
}

// ---- retry policy --------------------------------------------------------

pub const MAX_RETRIES: u32 = 4;

/// Exponential backoff: 150 ms * 2^n, capped at 12 s.
pub fn milvus_retry_delay_ms(attempt: u32) -> u64 {
    let base: u64 = 150u64.saturating_mul(1u64 << attempt.min(6));
    base.min(12_000)
}

pub fn should_retry_status(status: u16) -> bool {
    matches!(status, 429 | 500 | 502 | 503 | 504)
}

// ---- collection alias helpers --------------------------------------------

/// Build a rename-alias request body (swap alias to a new collection).
pub fn rename_alias_body(old_alias: &str, new_alias: &str) -> Value {
    json!({ "oldAliasName": old_alias, "newAliasName": new_alias })
}

/// Build an alter-alias request body (point existing alias to a new collection).
pub fn alter_alias_body(collection: &str, alias: &str) -> Value {
    json!({ "collectionName": collection, "aliasName": alias })
}

// ---- collection sizing guidance ------------------------------------------

/// Returns a rough nlist recommendation for IVF_FLAT given the expected row count.
/// Milvus recommends nlist in the range [sqrt(N), 4*sqrt(N)].
pub fn recommended_nlist(expected_rows: u64) -> u32 {
    let sqrt = (expected_rows as f64).sqrt() as u32;
    sqrt.clamp(1, 65536)
}

/// Returns a rough capacity guideline string for a collection.
pub fn sizing_guidance(dims: usize, expected_rows: u64) -> String {
    let bytes_per_vector = dims * 4; // f32
    let raw_mb = (expected_rows as usize * bytes_per_vector) / (1024 * 1024);
    let nlist = recommended_nlist(expected_rows);
    format!("dims={dims} rows={expected_rows} raw_vector_mb={raw_mb} recommended_nlist={nlist}")
}

// ---- upsert request body ------------------------------------------------

/// Build an upsert request body (insert-or-replace semantics).
pub fn upsert_entities_body(collection: &str, entities: &[(i64, Vec<f32>, Value)]) -> Value {
    let data: Vec<Value> = entities
        .iter()
        .map(|(id, embedding, payload)| {
            json!({ "id": id, "embedding": embedding, "payload": payload.to_string() })
        })
        .collect();
    json!({ "collectionName": collection, "data": data })
}

// ---- collection schema extra helpers ------------------------------------

/// Build a schema body that enables a specific consistency level at creation.
pub fn create_collection_with_consistency_body(
    collection: &str,
    dims: usize,
    metric: &str,
    consistency_level: &str,
) -> Value {
    let mut body = create_collection_body(collection, dims, metric);
    body["consistencyLevel"] = json!(consistency_level);
    body
}

/// Build a describe-collection request body.
pub fn describe_collection_body(collection: &str) -> Value {
    json!({ "collectionName": collection })
}

/// Build a get-stats request body.
pub fn get_stats_body(collection: &str) -> Value {
    json!({ "collectionName": collection })
}

// ---- unit tests ----------------------------------------------------------

#[cfg(test)]
mod milvus_tests {
    use super::*;

    #[test]
    fn url_builders_produce_expected_paths() {
        let base = "http://localhost:19530";
        assert_eq!(
            collections_url(base),
            "http://localhost:19530/v2/vectordb/collections"
        );
        assert_eq!(
            entities_search_url(base),
            "http://localhost:19530/v2/vectordb/entities/search"
        );
        assert_eq!(health_url(base), "http://localhost:19530/healthz");
    }

    #[test]
    fn create_collection_body_has_required_fields() {
        let body = create_collection_body("docs", 128, metric_type::COSINE);
        assert_eq!(body["collectionName"], "docs");
        let fields = body["schema"]["fields"].as_array().unwrap();
        let has_vector = fields.iter().any(|f| f["fieldName"] == "embedding");
        assert!(has_vector, "embedding field must be present");
    }

    #[test]
    fn create_collection_hnsw_sets_params() {
        let body = create_collection_hnsw_body("docs", 128, metric_type::COSINE, 16, 200);
        let idx = &body["indexParams"][0];
        assert_eq!(idx["indexType"], index_type::HNSW);
        assert_eq!(idx["params"]["M"], 16);
        assert_eq!(idx["params"]["efConstruction"], 200);
    }

    #[test]
    fn create_collection_ivf_sets_nlist() {
        let body = create_collection_ivf_body("docs", 128, metric_type::L2, 512);
        let idx = &body["indexParams"][0];
        assert_eq!(idx["indexType"], index_type::IVF_FLAT);
        assert_eq!(idx["params"]["nlist"], 512);
    }

    #[test]
    fn insert_entities_body_wraps_each_entity() {
        let entities = vec![
            (vec![0.1f32, 0.2], json!({"tag": "a"})),
            (vec![0.3f32, 0.4], json!({"tag": "b"})),
        ];
        let body = insert_entities_body("docs", &entities);
        assert_eq!(body["collectionName"], "docs");
        assert_eq!(body["data"].as_array().unwrap().len(), 2);
    }

    #[test]
    fn batch_insert_body_passes_entities_through() {
        let data = vec![
            json!({"embedding": [0.1f32]}),
            json!({"embedding": [0.2f32]}),
        ];
        let body = batch_insert_body("docs", data);
        assert_eq!(body["data"].as_array().unwrap().len(), 2);
    }

    #[test]
    fn search_body_includes_anns_field() {
        let body = search_body("docs", &[0.1f32, 0.2], 5, metric_type::COSINE, &["payload"]);
        assert_eq!(body["annsField"], "embedding");
        assert_eq!(body["limit"], 5);
    }

    #[test]
    fn search_with_filter_body_sets_filter() {
        let body = search_with_filter_body(
            "docs",
            &[0.1f32],
            3,
            metric_type::L2,
            "score > 5",
            &["payload"],
        );
        assert_eq!(body["filter"], "score > 5");
    }

    #[test]
    fn search_hnsw_body_sets_ef_param() {
        let body = search_hnsw_body("docs", &[0.1f32], 5, metric_type::COSINE, 64);
        assert_eq!(body["searchParams"]["params"]["ef"], 64);
    }

    #[test]
    fn search_partition_body_names_partition() {
        let body = search_partition_body("docs", "tenant_a", &[0.1f32], 10, metric_type::COSINE);
        let parts = body["partitionNames"].as_array().unwrap();
        assert!(
            parts.iter().any(|p| p == "tenant_a"),
            "partitionNames must include tenant_a"
        );
    }

    #[test]
    fn search_with_consistency_sets_level() {
        let body = search_with_consistency_body(
            "docs",
            &[0.1f32],
            5,
            metric_type::COSINE,
            consistency::STRONG,
        );
        assert_eq!(body["consistencyLevel"], consistency::STRONG);
    }

    #[test]
    fn query_body_sets_filter_and_output_fields() {
        let body = query_body("docs", "tag == \"a\"", &["id", "payload"]);
        assert_eq!(body["collectionName"], "docs");
        assert!(body["filter"].as_str().unwrap().contains("tag"));
    }

    #[test]
    fn delete_by_ids_body_builds_in_expr() {
        let body = delete_by_ids_body("docs", &[1, 2, 3]);
        let filter = body["filter"].as_str().unwrap();
        assert!(filter.starts_with("id in ["), "filter: {filter}");
    }

    #[test]
    fn hybrid_search_body_has_two_search_requests() {
        let dense = vec![0.1f32, 0.2];
        let sparse = vec![(0u32, 0.8f32), (5u32, 0.3f32)];
        let body = hybrid_search_body("docs", &dense, "sparse", &sparse, 10, metric_type::COSINE);
        let searches = body["search"].as_array().unwrap();
        assert_eq!(searches.len(), 2);
        assert_eq!(body["rerank"]["strategy"], "rrf");
    }

    #[test]
    fn boolean_expr_eq_str_escapes_quotes() {
        let expr = expr_eq_str("title", r#"he said "hi""#);
        assert!(expr.contains(r#"\""#), "expr: {expr}");
    }

    #[test]
    fn boolean_expr_and_wraps_operands() {
        let e = expr_and("a == 1", "b > 2");
        assert_eq!(e, "(a == 1) and (b > 2)");
    }

    #[test]
    fn boolean_expr_in_ints_produces_valid_list() {
        let e = expr_in_ints("id", &[10, 20, 30]);
        assert_eq!(e, "id in [10, 20, 30]");
    }

    #[test]
    fn parse_search_results_extracts_rows() {
        let body = json!({ "data": [
            { "id": 1, "distance": 0.9, "payload": r#"{"k":"v"}"# },
            { "id": 2, "distance": 0.7, "payload": r#"{"k":"w"}"# },
        ]});
        let results = parse_search_results(&body);
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].0, 1);
        assert!((results[0].1 - 0.9f32).abs() < 1e-4);
    }

    #[test]
    fn parse_collection_stats_reads_row_count() {
        let body = json!({ "data": { "rowCount": 42 } });
        assert_eq!(parse_collection_stats(&body), 42);
    }

    #[test]
    fn parse_insert_ids_extracts_list() {
        let body = json!({ "data": { "insertIds": [100, 101, 102] } });
        assert_eq!(parse_insert_ids(&body), vec![100, 101, 102]);
    }

    #[test]
    fn parse_partition_names_extracts_names() {
        let body = json!({ "data": [
            { "partitionName": "part_a" },
            { "partitionName": "part_b" },
        ]});
        let names = parse_partition_names(&body);
        assert_eq!(names, vec!["part_a", "part_b"]);
    }

    #[test]
    fn milvus_error_404_is_not_found() {
        let err = MilvusError::from_response(404, r#"{"message":"collection not found"}"#);
        assert!(matches!(err, MilvusError::NotFound(_)));
    }

    #[test]
    fn milvus_error_409_is_already_exists() {
        let err = MilvusError::from_response(409, r#"{"message":"collection already exists"}"#);
        assert!(matches!(err, MilvusError::AlreadyExists(_)));
    }

    #[test]
    fn milvus_error_503_is_transient() {
        let err = MilvusError::from_response(503, "overloaded");
        assert!(err.is_transient(), "503 must be transient");
    }

    #[test]
    fn milvus_retry_delay_caps_at_12s() {
        let d = milvus_retry_delay_ms(20);
        assert!(d <= 12_000, "delay must not exceed 12 s, got {d}");
    }

    #[test]
    fn recommended_nlist_scales_with_row_count() {
        assert!(recommended_nlist(1_000_000) > recommended_nlist(1_000));
    }

    #[test]
    fn sizing_guidance_includes_raw_mb_and_nlist() {
        let s = sizing_guidance(128, 1_000_000);
        assert!(s.contains("raw_vector_mb="), "guidance: {s}");
        assert!(s.contains("recommended_nlist="), "guidance: {s}");
    }
}
