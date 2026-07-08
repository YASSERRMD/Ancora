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
        Self {
            url: url.into(),
            api_key: None,
            openai_key: None,
            timeout_secs: 30,
        }
    }

    pub fn with_api_key(mut self, key: impl Into<String>) -> Self {
        self.api_key = Some(key.into());
        self
    }

    pub fn with_openai_key(mut self, key: impl Into<String>) -> Self {
        self.openai_key = Some(key.into());
        self
    }

    pub fn local() -> Self {
        Self::new("http://localhost:8080")
    }

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
pub fn create_class_body(class_name: &str, description: &str, vectorizer: &str) -> Value {
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
    let props: Vec<Value> = properties
        .iter()
        .map(|(name, dtype, desc)| {
            json!({
                "name": name,
                "description": desc,
                "dataType": [dtype]
            })
        })
        .collect();
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
    let objs: Vec<Value> = objects
        .iter()
        .map(|(class, props, vec)| {
            let mut obj = json!({ "class": class, "properties": props });
            if let Some(v) = vec {
                obj["vector"] = json!(v);
            }
            obj
        })
        .collect();
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
    let vec_str = format!(
        "[{}]",
        vector
            .iter()
            .map(|v| format!("{v}"))
            .collect::<Vec<_>>()
            .join(",")
    );
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
    let vec_str = format!(
        "[{}]",
        vector
            .iter()
            .map(|v| format!("{v}"))
            .collect::<Vec<_>>()
            .join(",")
    );
    let field_str = fields.join(" ");
    let query = format!(
        r#"{{ Get {{ {class}(nearVector: {{ vector: {vec_str} certainty: {certainty} }} limit: {limit}) {{ {field_str} _additional {{ id distance }} }} }} }}"#
    );
    json!({ "query": query })
}

// ---- vector index configuration ------------------------------------------

/// Weaviate vector index types.
pub mod vector_index_type {
    pub const HNSW: &str = "hnsw";
    pub const FLAT: &str = "flat";
}

/// Build an HNSW index config object for class creation.
pub fn hnsw_index_config(ef_construction: u16, max_connections: u8, ef: i32) -> Value {
    json!({
        "vectorIndexType": vector_index_type::HNSW,
        "vectorIndexConfig": {
            "efConstruction": ef_construction,
            "maxConnections": max_connections,
            "ef": ef
        }
    })
}

/// Build a flat index config (BQ-compressed, for very large collections).
pub fn flat_index_config(bq_compression: bool) -> Value {
    json!({
        "vectorIndexType": vector_index_type::FLAT,
        "vectorIndexConfig": {
            "bq": { "enabled": bq_compression }
        }
    })
}

/// Build a class creation body with explicit vector index config.
pub fn create_class_with_index_body(
    class_name: &str,
    vectorizer: &str,
    index_config: &Value,
) -> Value {
    let mut body = create_class_body(class_name, "", vectorizer);
    if let Some(vit) = index_config.get("vectorIndexType") {
        body["vectorIndexType"] = vit.clone();
    }
    if let Some(vic) = index_config.get("vectorIndexConfig") {
        body["vectorIndexConfig"] = vic.clone();
    }
    body
}

// ---- cross-reference links -----------------------------------------------

/// Build the URL for adding a cross-reference to an object.
pub fn add_reference_url(base: &str, class: &str, id: &str, property: &str) -> String {
    format!("{base}/v1/objects/{class}/{id}/references/{property}")
}

/// Build the body for a cross-reference to another object.
pub fn add_reference_body(to_class: &str, to_id: &str) -> Value {
    json!({ "beacon": format!("weaviate://localhost/{to_class}/{to_id}") })
}

/// Build a batch reference body for `POST /v1/batch/references`.
pub fn batch_references_body(refs: &[(&str, &str, &str, &str, &str)]) -> Vec<Value> {
    // (from_class, from_id, property, to_class, to_id)
    refs.iter()
        .map(|(from_class, from_id, property, to_class, to_id)| {
            json!({
                "from": format!("weaviate://localhost/{from_class}/{from_id}/{property}"),
                "to": format!("weaviate://localhost/{to_class}/{to_id}")
            })
        })
        .collect()
}

pub fn batch_references_url(base: &str) -> String {
    format!("{base}/v1/batch/references")
}

// ---- multi-tenancy support -----------------------------------------------

/// URL for managing tenants of a multi-tenant class.
pub fn tenants_url(base: &str, class: &str) -> String {
    format!("{base}/v1/schema/{class}/tenants")
}

/// Build the body to add tenants to a class.
pub fn add_tenants_body(tenants: &[&str]) -> Value {
    let ts: Vec<Value> = tenants.iter().map(|t| json!({ "name": t })).collect();
    json!(ts)
}

/// Build the multi-tenant version of an object URL.
pub fn tenant_object_url(base: &str, class: &str, id: &str, tenant: &str) -> String {
    format!("{base}/v1/objects/{class}/{id}?tenant={tenant}")
}

/// Build an object body with a tenant annotation.
pub fn create_tenant_object_body(
    class: &str,
    tenant: &str,
    properties: &Value,
    vector: Option<&[f32]>,
) -> Value {
    let mut body = create_object_body(class, properties, vector);
    body["tenant"] = json!(tenant);
    body
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
        let vs = format!(
            "[{}]",
            v.iter()
                .map(|x| format!("{x}"))
                .collect::<Vec<_>>()
                .join(",")
        );
        format!(", vector: {vs}")
    } else {
        String::new()
    };
    let q = format!(
        r#"{{ Get {{ {class}(hybrid: {{ query: "{query_text}" alpha: {alpha}{vector_part} }} limit: {limit}) {{ {field_str} _additional {{ id score }} }} }} }}"#
    );
    json!({ "query": q })
}

// ---- replication config --------------------------------------------------

/// Build the replication config block for a class creation body.
pub fn replication_config(factor: u8) -> Value {
    json!({ "replicationConfig": { "factor": factor } })
}

/// Build the sharding config block for a class creation body.
pub fn sharding_config(virtual_per_physical: u16) -> Value {
    json!({ "shardingConfig": { "virtualPerPhysical": virtual_per_physical } })
}

/// Merge replication and sharding config into a class body.
pub fn apply_cluster_config(
    mut class_body: Value,
    replication_factor: u8,
    virtual_shards: u16,
) -> Value {
    class_body["replicationConfig"] = json!({ "factor": replication_factor });
    class_body["shardingConfig"] = json!({ "virtualPerPhysical": virtual_shards });
    class_body
}

// ---- inverted index and property tokenization ----------------------------

/// Tokenization modes for text properties.
pub mod tokenization {
    pub const WORD: &str = "word";
    pub const LOWERCASE: &str = "lowercase";
    pub const WHITESPACE: &str = "whitespace";
    pub const FIELD: &str = "field";
    pub const TRIGRAM: &str = "trigram";
    pub const GSE: &str = "gse";
}

/// Build a property definition with a specific tokenization mode.
pub fn property_with_tokenization(name: &str, data_type: &str, tokenization: &str) -> Value {
    json!({
        "name": name,
        "dataType": [data_type],
        "tokenization": tokenization
    })
}

/// Build an inverted index config for a class.
pub fn inverted_index_config(
    bm25_b: f32,
    bm25_k1: f32,
    index_null_state: bool,
    index_property_length: bool,
) -> Value {
    json!({
        "bm25": { "b": bm25_b, "k1": bm25_k1 },
        "indexNullState": index_null_state,
        "indexPropertyLength": index_property_length
    })
}

// ---- backup and node management ------------------------------------------

/// URL for creating a backup.
pub fn backup_create_url(base: &str, backend: &str, backup_id: &str) -> String {
    format!("{base}/v1/backups/{backend}/{backup_id}")
}

/// URL for restoring a backup.
pub fn backup_restore_url(base: &str, backend: &str, backup_id: &str) -> String {
    format!("{base}/v1/backups/{backend}/{backup_id}/restore")
}

/// Build the body for creating a backup.
pub fn backup_create_body(include: &[&str], exclude: &[&str]) -> Value {
    json!({ "include": include, "exclude": exclude })
}

/// URL for cluster node status.
pub fn nodes_url(base: &str) -> String {
    format!("{base}/v1/nodes")
}

/// Parse node names from a Weaviate nodes response.
pub fn parse_node_names(body: &Value) -> Vec<String> {
    body["nodes"]
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .filter_map(|n| n["name"].as_str().map(|s| s.to_owned()))
        .collect()
}

// ---- aggregate queries ---------------------------------------------------

/// Build a GraphQL Aggregate query for total object count.
pub fn graphql_aggregate_count_query(class: &str) -> Value {
    let q = format!(r#"{{ Aggregate {{ {class} {{ meta {{ count }} }} }} }}"#);
    json!({ "query": q })
}

/// Build a GraphQL Aggregate query with a where filter.
pub fn graphql_aggregate_with_filter_query(class: &str, where_filter: &Value) -> Value {
    let filter_str = serde_json::to_string(where_filter).unwrap_or_default();
    let q = format!(r#"{{ Aggregate {{ {class}(where: {filter_str}) {{ meta {{ count }} }} }} }}"#);
    json!({ "query": q })
}

/// Parse the total count from an Aggregate response.
pub fn parse_aggregate_count(body: &Value, class: &str) -> Option<u64> {
    body["data"]["Aggregate"][class][0]["meta"]["count"].as_u64()
}

// ---- nearText and nearObject search queries ------------------------------

/// Build a GraphQL nearText search (text2vec vectorizer must be configured).
pub fn graphql_near_text_query(
    class: &str,
    concepts: &[&str],
    limit: usize,
    certainty: Option<f32>,
    fields: &[&str],
) -> Value {
    let concepts_str = concepts
        .iter()
        .map(|c| format!("\"{c}\""))
        .collect::<Vec<_>>()
        .join(", ");
    let certainty_str = certainty
        .map(|c| format!(", certainty: {c}"))
        .unwrap_or_default();
    let field_str = fields.join(" ");
    let q = format!(
        r#"{{ Get {{ {class}(nearText: {{ concepts: [{concepts_str}]{certainty_str} }} limit: {limit}) {{ {field_str} _additional {{ id distance }} }} }} }}"#
    );
    json!({ "query": q })
}

/// Build a GraphQL nearObject search (find objects similar to an existing one).
pub fn graphql_near_object_query(
    class: &str,
    object_id: &str,
    limit: usize,
    fields: &[&str],
) -> Value {
    let field_str = fields.join(" ");
    let q = format!(
        r#"{{ Get {{ {class}(nearObject: {{ id: "{object_id}" }} limit: {limit}) {{ {field_str} _additional {{ id distance }} }} }} }}"#
    );
    json!({ "query": q })
}

// ---- generative search (RAG) GraphQL -------------------------------------

/// Build a GraphQL generative search query (Weaviate's built-in RAG).
///
/// `prompt_template` is a Go-style template, e.g. `"Summarize {title}: {body}"`.
pub fn graphql_generative_query(
    class: &str,
    vector: &[f32],
    limit: usize,
    prompt_template: &str,
    fields: &[&str],
) -> Value {
    let vec_str = format!(
        "[{}]",
        vector
            .iter()
            .map(|v| format!("{v}"))
            .collect::<Vec<_>>()
            .join(",")
    );
    let field_str = fields.join(" ");
    let query = format!(
        r#"{{ Get {{ {class}(nearVector: {{ vector: {vec_str} }} limit: {limit}) {{ {field_str} _additional {{ generate(singleResult: {{ prompt: "{prompt_template}" }}) {{ singleResult error }} id }} }} }} }}"#
    );
    json!({ "query": query })
}

/// Build a grouped generative task query (all results -> one LLM call).
pub fn graphql_grouped_generative_query(
    class: &str,
    vector: &[f32],
    limit: usize,
    task_prompt: &str,
    fields: &[&str],
) -> Value {
    let vec_str = format!(
        "[{}]",
        vector
            .iter()
            .map(|v| format!("{v}"))
            .collect::<Vec<_>>()
            .join(",")
    );
    let field_str = fields.join(" ");
    let query = format!(
        r#"{{ Get {{ {class}(nearVector: {{ vector: {vec_str} }} limit: {limit}) {{ {field_str} _additional {{ generate(groupedResult: {{ task: "{task_prompt}" }}) {{ groupedResult error }} id }} }} }} }}"#
    );
    json!({ "query": query })
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

/// Parse match/delete counts from a `POST /v1/batch/objects` delete response.
pub fn parse_batch_delete_result(body: &Value) -> (u64, u64) {
    let matched = body["results"]["matches"].as_u64().unwrap_or(0);
    let deleted = body["results"]["successful"].as_u64().unwrap_or(0);
    (matched, deleted)
}

/// Extract GraphQL errors (if any) from a Weaviate GraphQL response.
pub fn parse_graphql_errors(body: &Value) -> Vec<String> {
    body["errors"]
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .filter_map(|e| e["message"].as_str().map(|m| m.to_owned()))
        .collect()
}

/// Extract GraphQL Get results from a Weaviate GraphQL response.
pub fn parse_graphql_get(body: &Value, class: &str) -> Vec<Value> {
    body["data"]["Get"][class]
        .as_array()
        .unwrap_or(&vec![])
        .to_vec()
}

// ---- schema inspection helpers -------------------------------------------

/// Extract class names from a `GET /v1/schema` response.
pub fn parse_class_names(body: &Value) -> Vec<String> {
    body["classes"]
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .filter_map(|c| c["class"].as_str().map(|s| s.to_owned()))
        .collect()
}

/// Extract property names from a class schema response.
pub fn parse_property_names(body: &Value) -> Vec<String> {
    body["properties"]
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .filter_map(|p| p["name"].as_str().map(|s| s.to_owned()))
        .collect()
}

/// Extract the vectorizer from a class schema response.
pub fn parse_vectorizer(body: &Value) -> Option<String> {
    body["vectorizer"].as_str().map(|s| s.to_owned())
}

// ---- additional-field helpers (GraphQL _additional) ----------------------

/// Extract the UUID from an object's `_additional.id` field.
pub fn parse_additional_id(obj: &Value) -> Option<String> {
    obj["_additional"]["id"].as_str().map(|s| s.to_owned())
}

/// Extract the certainty score (0.0 - 1.0) from `_additional.certainty`.
pub fn parse_additional_certainty(obj: &Value) -> Option<f32> {
    obj["_additional"]["certainty"].as_f64().map(|v| v as f32)
}

/// Extract the vector distance from `_additional.distance`.
pub fn parse_additional_distance(obj: &Value) -> Option<f32> {
    obj["_additional"]["distance"].as_f64().map(|v| v as f32)
}

/// Extract `(id, certainty)` tuples from a GraphQL Get response for `class`.
pub fn parse_get_with_certainty(body: &Value, class: &str) -> Vec<(String, f32, Value)> {
    parse_graphql_get(body, class)
        .into_iter()
        .filter_map(|mut obj| {
            let id = parse_additional_id(&obj)?;
            let certainty = parse_additional_certainty(&obj).unwrap_or(0.0);
            obj.as_object_mut()?.remove("_additional");
            Some((id, certainty, obj))
        })
        .collect()
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
            "Document",
            "test",
            "none",
            &[
                ("title", data_type::TEXT, "The title"),
                ("year", data_type::INT, "Year"),
            ],
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
            (
                "Document".to_owned(),
                json!({"title": "a"}),
                Some(vec![0.1f32]),
            ),
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
    fn parse_class_names_extracts_all() {
        let body = json!({ "classes": [{ "class": "Document" }, { "class": "Author" }] });
        let names = parse_class_names(&body);
        assert_eq!(names, vec!["Document", "Author"]);
    }

    #[test]
    fn parse_property_names_extracts_names() {
        let body = json!({ "properties": [{ "name": "title" }, { "name": "body" }] });
        let names = parse_property_names(&body);
        assert_eq!(names, vec!["title", "body"]);
    }

    #[test]
    fn parse_vectorizer_extracts_value() {
        let body = json!({ "class": "Document", "vectorizer": "text2vec-openai" });
        assert_eq!(parse_vectorizer(&body), Some("text2vec-openai".to_owned()));
    }

    #[test]
    fn parse_additional_id_extracts_uuid() {
        let obj = json!({ "_additional": { "id": "abc-123" }, "title": "doc" });
        assert_eq!(parse_additional_id(&obj), Some("abc-123".to_owned()));
    }

    #[test]
    fn parse_additional_certainty_extracts_score() {
        let obj = json!({ "_additional": { "certainty": 0.95 } });
        let c = parse_additional_certainty(&obj).unwrap();
        assert!((c - 0.95f32).abs() < 1e-4);
    }

    #[test]
    fn parse_additional_distance_extracts_value() {
        let obj = json!({ "_additional": { "distance": 0.12 } });
        let d = parse_additional_distance(&obj).unwrap();
        assert!((d - 0.12f32).abs() < 1e-4);
    }

    #[test]
    fn parse_get_with_certainty_extracts_triples() {
        let body = json!({
            "data": { "Get": { "Document": [
                { "_additional": { "id": "id-1", "certainty": 0.9 }, "title": "a" },
                { "_additional": { "id": "id-2", "certainty": 0.7 }, "title": "b" },
            ]}}
        });
        let results = parse_get_with_certainty(&body, "Document");
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].0, "id-1");
        assert!((results[0].1 - 0.9f32).abs() < 1e-4);
    }

    #[test]
    fn parse_batch_delete_result_extracts_counts() {
        let body = json!({ "results": { "matches": 5, "successful": 4 } });
        let (matched, deleted) = parse_batch_delete_result(&body);
        assert_eq!(matched, 5);
        assert_eq!(deleted, 4);
    }

    #[test]
    fn parse_batch_delete_result_zeros_on_missing() {
        let body = json!({});
        let (matched, deleted) = parse_batch_delete_result(&body);
        assert_eq!(matched, 0);
        assert_eq!(deleted, 0);
    }

    #[test]
    fn parse_graphql_errors_extracts_messages() {
        let body = json!({
            "errors": [
                { "message": "class not found" },
                { "message": "property unknown" },
            ]
        });
        let errs = parse_graphql_errors(&body);
        assert_eq!(errs, vec!["class not found", "property unknown"]);
    }

    #[test]
    fn parse_graphql_errors_empty_on_no_errors() {
        let body = json!({ "data": { "Get": {} } });
        assert!(parse_graphql_errors(&body).is_empty());
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
    fn replication_config_has_factor() {
        let cfg = replication_config(3);
        assert_eq!(cfg["replicationConfig"]["factor"], 3);
    }

    #[test]
    fn sharding_config_has_virtual_per_physical() {
        let cfg = sharding_config(128);
        assert_eq!(cfg["shardingConfig"]["virtualPerPhysical"], 128);
    }

    #[test]
    fn apply_cluster_config_merges_into_body() {
        let body = create_class_body("Document", "test", "none");
        let merged = apply_cluster_config(body, 3, 128);
        assert_eq!(merged["replicationConfig"]["factor"], 3);
        assert_eq!(merged["shardingConfig"]["virtualPerPhysical"], 128);
        assert_eq!(merged["class"], "Document");
    }

    #[test]
    fn property_with_tokenization_has_tokenization_field() {
        let p = property_with_tokenization("body", data_type::TEXT, tokenization::WORD);
        assert_eq!(p["tokenization"], "word");
        assert_eq!(p["dataType"][0], data_type::TEXT);
    }

    #[test]
    fn inverted_index_config_has_bm25_params() {
        let cfg = inverted_index_config(0.75, 1.2, true, false);
        assert!((cfg["bm25"]["b"].as_f64().unwrap() - 0.75).abs() < 0.001);
        assert!((cfg["bm25"]["k1"].as_f64().unwrap() - 1.2).abs() < 0.001);
        assert_eq!(cfg["indexNullState"], true);
    }

    #[test]
    fn backup_create_url_has_backend_and_id() {
        let url = backup_create_url("http://localhost:8080", "s3", "bk-001");
        assert!(url.contains("/backups/s3/bk-001"), "url: {url}");
    }

    #[test]
    fn backup_restore_url_ends_with_restore() {
        let url = backup_restore_url("http://localhost:8080", "s3", "bk-001");
        assert!(url.ends_with("/restore"), "url: {url}");
    }

    #[test]
    fn backup_create_body_has_include_and_exclude() {
        let body = backup_create_body(&["Document", "Author"], &["TempData"]);
        assert!(body["include"].is_array());
        assert!(body["exclude"].is_array());
    }

    #[test]
    fn parse_node_names_extracts_names() {
        let body = json!({ "nodes": [{ "name": "node1" }, { "name": "node2" }] });
        let names = parse_node_names(&body);
        assert_eq!(names, vec!["node1", "node2"]);
    }

    #[test]
    fn graphql_aggregate_count_query_has_meta_count() {
        let body = graphql_aggregate_count_query("Document");
        let q = body["query"].as_str().unwrap();
        assert!(q.contains("Aggregate"), "query: {q}");
        assert!(
            q.contains("meta { count }") || q.contains("meta{count}") || q.contains("count"),
            "query: {q}"
        );
    }

    #[test]
    fn parse_aggregate_count_extracts_value() {
        let body = json!({
            "data": { "Aggregate": { "Document": [{ "meta": { "count": 42 } }] } }
        });
        assert_eq!(parse_aggregate_count(&body, "Document"), Some(42));
    }

    #[test]
    fn graphql_near_text_query_contains_concepts() {
        let body =
            graphql_near_text_query("Document", &["machine learning", "AI"], 5, None, &["title"]);
        let q = body["query"].as_str().unwrap();
        assert!(q.contains("nearText"), "query: {q}");
        assert!(q.contains("machine learning"), "query: {q}");
    }

    #[test]
    fn graphql_near_text_with_certainty() {
        let body = graphql_near_text_query("Document", &["rust"], 5, Some(0.7), &["title"]);
        let q = body["query"].as_str().unwrap();
        assert!(q.contains("certainty: 0.7"), "query: {q}");
    }

    #[test]
    fn graphql_near_object_query_has_id() {
        let body = graphql_near_object_query("Document", "my-uuid", 5, &["title"]);
        let q = body["query"].as_str().unwrap();
        assert!(q.contains("nearObject"), "query: {q}");
        assert!(q.contains("my-uuid"), "query: {q}");
    }

    #[test]
    fn graphql_generative_query_contains_generate_keyword() {
        let body =
            graphql_generative_query("Document", &[0.1f32], 3, "Summarize {title}", &["title"]);
        let q = body["query"].as_str().unwrap();
        assert!(q.contains("generate"), "query: {q}");
        assert!(q.contains("singleResult"), "query: {q}");
        assert!(q.contains("Summarize"), "query: {q}");
    }

    #[test]
    fn graphql_grouped_generative_query_contains_grouped_result() {
        let body = graphql_grouped_generative_query(
            "Document",
            &[0.1f32],
            3,
            "Analyze all docs",
            &["title"],
        );
        let q = body["query"].as_str().unwrap();
        assert!(q.contains("groupedResult"), "query: {q}");
        assert!(q.contains("Analyze all docs"), "query: {q}");
    }

    #[test]
    fn hnsw_index_config_has_correct_structure() {
        let cfg = hnsw_index_config(128, 64, 100);
        assert_eq!(cfg["vectorIndexType"], "hnsw");
        assert_eq!(cfg["vectorIndexConfig"]["efConstruction"], 128);
        assert_eq!(cfg["vectorIndexConfig"]["maxConnections"], 64);
    }

    #[test]
    fn flat_index_config_has_bq_enabled() {
        let cfg = flat_index_config(true);
        assert_eq!(cfg["vectorIndexType"], "flat");
        assert_eq!(cfg["vectorIndexConfig"]["bq"]["enabled"], true);
    }

    #[test]
    fn create_class_with_index_body_merges_index_config() {
        let idx = hnsw_index_config(100, 32, 64);
        let body = create_class_with_index_body("Document", "none", &idx);
        assert_eq!(body["vectorIndexType"], "hnsw");
        assert_eq!(body["class"], "Document");
    }

    #[test]
    fn add_reference_body_has_beacon() {
        let body = add_reference_body("Author", "author-uuid");
        assert!(
            body["beacon"].as_str().unwrap().contains("Author"),
            "body: {body}"
        );
        assert!(
            body["beacon"].as_str().unwrap().contains("author-uuid"),
            "body: {body}"
        );
    }

    #[test]
    fn add_reference_url_includes_property() {
        let url = add_reference_url("http://localhost:8080", "Document", "uuid-1", "author");
        assert!(url.ends_with("/author"), "url: {url}");
    }

    #[test]
    fn batch_references_body_has_from_and_to() {
        let refs = vec![("Document", "doc-1", "author", "Author", "auth-1")];
        let bodies = batch_references_body(&refs);
        assert_eq!(bodies.len(), 1);
        assert!(bodies[0]["from"].as_str().unwrap().contains("Document"));
        assert!(bodies[0]["to"].as_str().unwrap().contains("Author"));
    }

    #[test]
    fn tenants_url_format() {
        let url = tenants_url("http://localhost:8080", "Document");
        assert!(url.ends_with("/Document/tenants"), "url: {url}");
    }

    #[test]
    fn add_tenants_body_is_array_of_name_objects() {
        let body = add_tenants_body(&["tenant_a", "tenant_b"]);
        assert_eq!(body[0]["name"], "tenant_a");
        assert_eq!(body[1]["name"], "tenant_b");
    }

    #[test]
    fn tenant_object_url_has_tenant_param() {
        let url = tenant_object_url("http://localhost:8080", "Document", "uuid-1", "org_a");
        assert!(url.contains("tenant=org_a"), "url: {url}");
    }

    #[test]
    fn create_tenant_object_body_has_tenant_field() {
        let props = json!({"title": "test"});
        let body = create_tenant_object_body("Document", "org_a", &props, None);
        assert_eq!(body["tenant"], "org_a");
    }

    #[test]
    fn should_retry_429_and_503() {
        assert!(should_retry_status(429));
        assert!(should_retry_status(503));
        assert!(!should_retry_status(400));
        assert!(!should_retry_status(404));
    }
}
