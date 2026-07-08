/// Redis Vector (RediSearch / Redis Stack) backend for the `VectorStore` trait.
///
/// Redis Stack includes RediSearch, which adds vector similarity search via the
/// HNSW and FLAT indexes. This module generates Redis command descriptors and
/// URL strings for the Redis REST (RedisInsight / Redis Enterprise REST) API
/// without requiring a live server.
///
/// Requires the `redis-vector` feature: `ancora-memory = { features = ["redis-vector"] }`.
use serde_json::{json, Value};

// ---- connection config ---------------------------------------------------

#[derive(Debug, Clone)]
pub struct RedisVectorConfig {
    /// Redis host, e.g. `localhost` or `redis-12345.c1.us-east-1.ec2.cloud.redislabs.com`.
    pub host: String,
    pub port: u16,
    pub password: Option<String>,
    pub tls: bool,
    pub timeout_secs: u64,
}

impl RedisVectorConfig {
    pub fn new(host: impl Into<String>, port: u16) -> Self {
        Self {
            host: host.into(),
            port,
            password: None,
            tls: false,
            timeout_secs: 30,
        }
    }

    pub fn with_password(mut self, pwd: impl Into<String>) -> Self {
        self.password = Some(pwd.into());
        self
    }

    pub fn with_tls(mut self) -> Self {
        self.tls = true;
        self
    }

    pub fn local() -> Self {
        Self::new("localhost", 6379)
    }

    /// Returns the connection URL string.
    pub fn url(&self) -> String {
        let scheme = if self.tls { "rediss" } else { "redis" };
        if let Some(pwd) = &self.password {
            format!("{scheme}://:{pwd}@{}:{}", self.host, self.port)
        } else {
            format!("{scheme}://{}:{}", self.host, self.port)
        }
    }
}

// ---- field type constants ------------------------------------------------

pub mod field_type {
    pub const VECTOR: &str = "VECTOR";
    pub const TEXT: &str = "TEXT";
    pub const TAG: &str = "TAG";
    pub const NUMERIC: &str = "NUMERIC";
}

// ---- index algorithm constants ------------------------------------------

pub mod algorithm {
    pub const HNSW: &str = "HNSW";
    pub const FLAT: &str = "FLAT";
}

// ---- distance constants -------------------------------------------------

pub mod distance {
    pub const COSINE: &str = "COSINE";
    pub const IP: &str = "IP";
    pub const L2: &str = "L2";
}

// ---- FT.CREATE command descriptor ----------------------------------------

/// Describes an FT.CREATE command for a vector index.
#[derive(Debug, Clone)]
pub struct CreateIndexArgs {
    pub index_name: String,
    pub prefix: String,
    pub vector_field: String,
    pub dims: usize,
    pub algorithm: String,
    pub distance: String,
    pub ef_construction: Option<u16>,
    pub m: Option<u16>,
    pub extra_fields: Vec<(String, String)>,
}

impl CreateIndexArgs {
    pub fn new(index_name: impl Into<String>, prefix: impl Into<String>, dims: usize) -> Self {
        Self {
            index_name: index_name.into(),
            prefix: prefix.into(),
            vector_field: "embedding".to_owned(),
            dims,
            algorithm: algorithm::HNSW.to_owned(),
            distance: distance::COSINE.to_owned(),
            ef_construction: Some(200),
            m: Some(16),
            extra_fields: vec![],
        }
    }

    pub fn flat(mut self) -> Self {
        self.algorithm = algorithm::FLAT.to_owned();
        self.ef_construction = None;
        self.m = None;
        self
    }

    pub fn distance(mut self, d: impl Into<String>) -> Self {
        self.distance = d.into();
        self
    }

    pub fn hnsw_params(mut self, ef: u16, m: u16) -> Self {
        self.ef_construction = Some(ef);
        self.m = Some(m);
        self
    }

    pub fn add_field(mut self, name: impl Into<String>, field_type: impl Into<String>) -> Self {
        self.extra_fields.push((name.into(), field_type.into()));
        self
    }

    /// Returns the FT.CREATE command as a JSON descriptor (for documentation / codegen).
    pub fn to_json(&self) -> Value {
        let mut params = vec![
            json!("TYPE"),
            json!("FLOAT32"),
            json!("DIM"),
            json!(self.dims),
            json!("DISTANCE_METRIC"),
            json!(self.distance),
        ];
        if self.algorithm == algorithm::HNSW {
            if let Some(ef) = self.ef_construction {
                params.push(json!("EF_CONSTRUCTION"));
                params.push(json!(ef));
            }
            if let Some(m) = self.m {
                params.push(json!("M"));
                params.push(json!(m));
            }
        }

        let mut fields = vec![
            json!(self.vector_field),
            json!(field_type::VECTOR),
            json!(self.algorithm),
            json!(params.len()),
            json!(params),
        ];
        for (name, ftype) in &self.extra_fields {
            fields.push(json!(name));
            fields.push(json!(ftype));
        }

        json!({
            "command": "FT.CREATE",
            "index": self.index_name,
            "on": "HASH",
            "prefix": [self.prefix],
            "schema": fields,
        })
    }
}

// ---- FT.SEARCH command descriptor ----------------------------------------

/// Describes an FT.SEARCH vector query.
#[derive(Debug, Clone)]
pub struct SearchArgs {
    pub index_name: String,
    pub query_filter: String,
    pub vector_field: String,
    pub top_k: usize,
    pub return_fields: Vec<String>,
    pub dialect: u8,
}

impl SearchArgs {
    /// ANN-only search (no scalar pre-filter).
    pub fn ann(
        index_name: impl Into<String>,
        vector_field: impl Into<String>,
        top_k: usize,
    ) -> Self {
        let vf = vector_field.into();
        let query_filter = format!("(*)=>[KNN {top_k} @{vf} $query_vec AS score]");
        Self {
            index_name: index_name.into(),
            query_filter,
            vector_field: vf,
            top_k,
            return_fields: vec!["score".to_owned(), "payload".to_owned()],
            dialect: 2,
        }
    }

    /// Pre-filtered ANN: `(@tag:{value}|...) => [KNN ...]`.
    pub fn filtered_ann(
        index_name: impl Into<String>,
        pre_filter: impl Into<String>,
        vector_field: impl Into<String>,
        top_k: usize,
    ) -> Self {
        let vf = vector_field.into();
        let query_filter = format!(
            "({})=>[KNN {top_k} @{vf} $query_vec AS score]",
            pre_filter.into()
        );
        Self {
            index_name: index_name.into(),
            query_filter,
            vector_field: vf,
            top_k,
            return_fields: vec!["score".to_owned(), "payload".to_owned()],
            dialect: 2,
        }
    }

    pub fn returns(mut self, fields: &[&str]) -> Self {
        self.return_fields = fields.iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn to_json(&self) -> Value {
        json!({
            "command": "FT.SEARCH",
            "index": self.index_name,
            "query": self.query_filter,
            "params": { "query_vec": "<binary_vector_bytes>" },
            "return": self.return_fields,
            "SORTBY": "score",
            "LIMIT": [0, self.top_k],
            "DIALECT": self.dialect,
        })
    }
}

// ---- HSET / HMSET helpers -----------------------------------------------

/// Build a HSET descriptor for inserting a vector document.
pub fn hset_descriptor(key: &str, embedding: &[f32], payload: Value) -> Value {
    json!({
        "command": "HSET",
        "key": key,
        "fields": {
            "embedding": "<binary_float32_le>",
            "embedding_dims": embedding.len(),
            "payload": payload.to_string(),
        }
    })
}

/// Build a key pattern for a document by prefix and ID.
pub fn document_key(prefix: &str, id: u64) -> String {
    format!("{prefix}:{id}")
}

// ---- filter expression helpers ------------------------------------------

/// Tag filter: `@field:{value}`.
pub fn tag_filter(field: &str, value: &str) -> String {
    format!("@{field}:{{{value}}}")
}

/// Numeric range filter: `@field:[lo hi]`.
pub fn numeric_range(field: &str, lo: f64, hi: f64) -> String {
    format!("@{field}:[{lo} {hi}]")
}

/// Text match: `@field:(word)`.
pub fn text_match(field: &str, word: &str) -> String {
    format!("@{field}:({word})")
}

// ---- response parsing ---------------------------------------------------

/// Parse FT.SEARCH response (simplified JSON form; real responses come as flat arrays).
pub fn parse_search_results(body: &Value) -> Vec<(String, f32, Value)> {
    body["results"]
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .map(|r| {
            let key = r["key"].as_str().unwrap_or("").to_owned();
            let score = r["score"].as_f64().unwrap_or(0.0) as f32;
            let payload = r["payload"]
                .as_str()
                .and_then(|s| serde_json::from_str(s).ok())
                .unwrap_or(json!({}));
            (key, score, payload)
        })
        .collect()
}

pub fn parse_index_info(body: &Value) -> Option<String> {
    body["index_name"].as_str().map(|s| s.to_owned())
}

// ---- error handling -----------------------------------------------------

#[derive(Debug, PartialEq)]
pub enum RedisVectorError {
    IndexNotFound(String),
    IndexAlreadyExists(String),
    WrongType(String),
    OutOfMemory,
    Unknown(String),
}

impl RedisVectorError {
    pub fn from_redis_err(msg: &str) -> Self {
        if msg.contains("Index already exists") {
            Self::IndexAlreadyExists(msg.to_owned())
        } else if msg.contains("Unknown Index name") {
            Self::IndexNotFound(msg.to_owned())
        } else if msg.contains("WRONGTYPE") {
            Self::WrongType(msg.to_owned())
        } else if msg.contains("OOM") {
            Self::OutOfMemory
        } else {
            Self::Unknown(msg.to_owned())
        }
    }

    pub fn is_transient(&self) -> bool {
        matches!(self, Self::OutOfMemory)
    }
}

pub const MAX_RETRIES: u32 = 3;

pub fn redis_retry_delay_ms(attempt: u32) -> u64 {
    let base: u64 = 50u64.saturating_mul(1u64 << attempt.min(6));
    base.min(2_000)
}

// ---- unit tests ---------------------------------------------------------

#[cfg(test)]
mod redis_vector_tests {
    use super::*;

    #[test]
    fn config_local_url() {
        let cfg = RedisVectorConfig::local();
        assert_eq!(cfg.url(), "redis://localhost:6379");
    }

    #[test]
    fn config_tls_url_uses_rediss() {
        let cfg = RedisVectorConfig::new("host", 6380).with_tls();
        assert!(cfg.url().starts_with("rediss://"), "url: {}", cfg.url());
    }

    #[test]
    fn config_password_in_url() {
        let cfg = RedisVectorConfig::new("host", 6379).with_password("secret");
        assert!(cfg.url().contains(":secret@"), "url: {}", cfg.url());
    }

    #[test]
    fn create_index_json_has_command() {
        let idx = CreateIndexArgs::new("docs", "doc:", 128);
        let j = idx.to_json();
        assert_eq!(j["command"], "FT.CREATE");
    }

    #[test]
    fn create_index_flat_has_no_hnsw_params() {
        let idx = CreateIndexArgs::new("docs", "doc:", 128).flat();
        assert_eq!(idx.algorithm, algorithm::FLAT);
        assert!(idx.ef_construction.is_none());
    }

    #[test]
    fn create_index_hnsw_stores_params() {
        let idx = CreateIndexArgs::new("docs", "doc:", 128).hnsw_params(128, 32);
        assert_eq!(idx.ef_construction, Some(128));
        assert_eq!(idx.m, Some(32));
    }

    #[test]
    fn search_ann_query_contains_knn() {
        let s = SearchArgs::ann("docs", "embedding", 10);
        let j = s.to_json();
        let q = j["query"].as_str().unwrap();
        assert!(q.contains("KNN"), "query: {q}");
    }

    #[test]
    fn search_filtered_ann_includes_pre_filter() {
        let s = SearchArgs::filtered_ann("docs", "@tag:{rust}", "embedding", 5);
        let j = s.to_json();
        let q = j["query"].as_str().unwrap();
        assert!(q.contains("@tag:{rust}"), "query: {q}");
        assert!(q.contains("KNN"), "query: {q}");
    }

    #[test]
    fn tag_filter_format() {
        let f = tag_filter("lang", "en");
        assert_eq!(f, "@lang:{en}");
    }

    #[test]
    fn numeric_range_format() {
        let f = numeric_range("score", 0.5, 1.0);
        assert_eq!(f, "@score:[0.5 1]");
    }

    #[test]
    fn document_key_format() {
        assert_eq!(document_key("doc", 42), "doc:42");
    }

    #[test]
    fn redis_error_index_exists_classification() {
        let err = RedisVectorError::from_redis_err("Index already exists");
        assert!(matches!(err, RedisVectorError::IndexAlreadyExists(_)));
    }

    #[test]
    fn redis_error_oom_is_transient() {
        let err = RedisVectorError::from_redis_err("OOM command not allowed");
        assert!(err.is_transient());
    }

    #[test]
    fn retry_delay_caps_at_2s() {
        assert!(redis_retry_delay_ms(20) <= 2_000);
    }
}
