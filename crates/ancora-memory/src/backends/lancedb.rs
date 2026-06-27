/// LanceDB backend for the `VectorStore` trait.
///
/// LanceDB is an embedded, serverless vector database backed by the Lance
/// columnar format. Data lives on the local filesystem or object storage
/// (S3, GCS, Azure Blob). No server process is required -- all operations
/// are in-process.
///
/// This module generates configuration structs, query descriptors, and
/// schema definitions that callers can use to drive a real LanceDB
/// connection (via the `lancedb` crate or REST API), plus a full offline
/// test suite that verifies the logic without any I/O.
///
/// Requires the `lancedb` feature: `ancora-memory = { features = ["lancedb"] }`.

use serde_json::{json, Value};

// ---- connection config ---------------------------------------------------

/// How to open a LanceDB table.
#[derive(Debug, Clone, PartialEq)]
pub enum LanceDbPath {
    /// Local directory, e.g. `/data/lancedb`.
    Local(String),
    /// S3 path, e.g. `s3://bucket/prefix`.
    S3(String),
    /// GCS path, e.g. `gs://bucket/prefix`.
    Gcs(String),
    /// Azure Blob path, e.g. `az://container/prefix`.
    Azure(String),
}

impl LanceDbPath {
    /// Returns the URI string for this path.
    pub fn uri(&self) -> &str {
        match self {
            Self::Local(p) | Self::S3(p) | Self::Gcs(p) | Self::Azure(p) => p.as_str(),
        }
    }

    /// Returns `true` when no external network is required.
    pub fn is_local(&self) -> bool {
        matches!(self, Self::Local(_))
    }

    /// Returns `true` when the path points to object storage.
    pub fn is_remote(&self) -> bool {
        !self.is_local()
    }

    pub fn local(path: impl Into<String>) -> Self { Self::Local(path.into()) }
    pub fn s3(path: impl Into<String>) -> Self { Self::S3(path.into()) }
    pub fn gcs(path: impl Into<String>) -> Self { Self::Gcs(path.into()) }
    pub fn azure(path: impl Into<String>) -> Self { Self::Azure(path.into()) }
}

/// Configuration for a LanceDB database directory.
#[derive(Debug, Clone)]
pub struct LanceDbConfig {
    /// Path to the database directory.
    pub path: LanceDbPath,
    /// AWS region (for S3 paths).
    pub aws_region: Option<String>,
    /// Read-only mode -- prevents accidental writes.
    pub read_only: bool,
}

impl LanceDbConfig {
    pub fn new(path: LanceDbPath) -> Self {
        Self { path, aws_region: None, read_only: false }
    }

    pub fn local(dir: impl Into<String>) -> Self {
        Self::new(LanceDbPath::local(dir))
    }

    pub fn s3(bucket_path: impl Into<String>, region: impl Into<String>) -> Self {
        Self { path: LanceDbPath::s3(bucket_path), aws_region: Some(region.into()), read_only: false }
    }

    pub fn read_only(mut self) -> Self {
        self.read_only = true; self
    }
}

// ---- schema / column types -----------------------------------------------

pub mod column_type {
    pub const FLOAT32: &str = "float32";
    pub const INT64: &str = "int64";
    pub const UTF8: &str = "utf8";
    pub const BOOL: &str = "bool";
    pub const DATE32: &str = "date32";
    pub const BINARY: &str = "binary";
}

// ---- table schema builder ------------------------------------------------

/// A column definition in a LanceDB table schema.
#[derive(Debug, Clone)]
pub struct ColumnDef {
    pub name: String,
    pub data_type: String,
    pub nullable: bool,
}

impl ColumnDef {
    pub fn new(name: impl Into<String>, data_type: impl Into<String>) -> Self {
        Self { name: name.into(), data_type: data_type.into(), nullable: true }
    }

    pub fn required(mut self) -> Self {
        self.nullable = false; self
    }
}

/// Build a table schema descriptor (for documentation / codegen, not direct Arrow use).
pub fn table_schema(
    vector_dims: usize,
    extra_columns: &[ColumnDef],
) -> Value {
    let mut columns = vec![
        json!({ "name": "id", "type": column_type::INT64, "nullable": false }),
        json!({ "name": "embedding", "type": format!("fixed_size_list<float32>[{vector_dims}]"), "nullable": false }),
        json!({ "name": "payload", "type": column_type::UTF8, "nullable": true }),
    ];
    for col in extra_columns {
        columns.push(json!({ "name": col.name, "type": col.data_type, "nullable": col.nullable }));
    }
    json!({ "columns": columns, "vector_dims": vector_dims })
}

// ---- row / add helpers ---------------------------------------------------

/// Build a single row descriptor for insertion.
pub fn row(id: i64, embedding: Vec<f32>, payload: Value) -> Value {
    json!({ "id": id, "embedding": embedding, "payload": payload.to_string() })
}

/// Build a batch of rows.
pub fn rows(data: &[(i64, Vec<f32>, Value)]) -> Vec<Value> {
    data.iter()
        .map(|(id, emb, payload)| row(*id, emb.clone(), payload.clone()))
        .collect()
}

/// Build a row with extra typed columns.
pub fn row_with_columns(id: i64, embedding: Vec<f32>, payload: Value, extra: Value) -> Value {
    let mut base = row(id, embedding, payload);
    if let (Some(obj), Some(extra_obj)) = (base.as_object_mut(), extra.as_object()) {
        for (k, v) in extra_obj {
            obj.insert(k.clone(), v.clone());
        }
    }
    base
}

// ---- vector search query descriptors ------------------------------------

/// Describe a vector search query (offline -- used as input to a real client).
#[derive(Debug, Clone)]
pub struct VectorQuery {
    pub table: String,
    pub vector: Vec<f32>,
    pub limit: usize,
    pub metric: String,
    pub filter: Option<String>,
    pub ef: Option<u32>,
    pub refine_factor: Option<u32>,
    pub select_columns: Vec<String>,
}

impl VectorQuery {
    pub fn new(table: impl Into<String>, vector: Vec<f32>, limit: usize) -> Self {
        Self {
            table: table.into(),
            vector,
            limit,
            metric: "cosine".to_owned(),
            filter: None,
            ef: None,
            refine_factor: None,
            select_columns: vec!["id".to_owned(), "payload".to_owned()],
        }
    }

    pub fn metric(mut self, m: impl Into<String>) -> Self {
        self.metric = m.into(); self
    }

    pub fn filter(mut self, sql: impl Into<String>) -> Self {
        self.filter = Some(sql.into()); self
    }

    pub fn ef(mut self, ef: u32) -> Self {
        self.ef = Some(ef); self
    }

    pub fn refine(mut self, factor: u32) -> Self {
        self.refine_factor = Some(factor); self
    }

    pub fn select(mut self, columns: &[&str]) -> Self {
        self.select_columns = columns.iter().map(|s| s.to_string()).collect(); self
    }

    pub fn to_json(&self) -> Value {
        let mut q = json!({
            "table": self.table,
            "vector": self.vector,
            "limit": self.limit,
            "metric": self.metric,
            "select": self.select_columns,
        });
        if let Some(f) = &self.filter {
            q["filter"] = json!(f);
        }
        if let Some(ef) = self.ef {
            q["ef"] = json!(ef);
        }
        if let Some(rf) = self.refine_factor {
            q["refine_factor"] = json!(rf);
        }
        q
    }
}

// ---- SQL filter helpers --------------------------------------------------

pub fn sql_eq_str(col: &str, val: &str) -> String {
    let escaped = val.replace('\'', "''");
    format!("{col} = '{escaped}'")
}

pub fn sql_eq_int(col: &str, val: i64) -> String {
    format!("{col} = {val}")
}

pub fn sql_gt(col: &str, val: i64) -> String {
    format!("{col} > {val}")
}

pub fn sql_lt(col: &str, val: i64) -> String {
    format!("{col} < {val}")
}

pub fn sql_is_null(col: &str) -> String {
    format!("{col} IS NULL")
}

pub fn sql_is_not_null(col: &str) -> String {
    format!("{col} IS NOT NULL")
}

pub fn sql_and(a: &str, b: &str) -> String {
    format!("({a}) AND ({b})")
}

pub fn sql_or(a: &str, b: &str) -> String {
    format!("({a}) OR ({b})")
}

pub fn sql_in_ints(col: &str, vals: &[i64]) -> String {
    let list = vals.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(", ");
    format!("{col} IN ({list})")
}

// ---- full-text index descriptor ------------------------------------------

/// Describes a full-text index configuration for a string column.
#[derive(Debug, Clone)]
pub struct FullTextIndex {
    pub column: String,
    pub with_position: bool,
}

impl FullTextIndex {
    pub fn new(column: impl Into<String>) -> Self {
        Self { column: column.into(), with_position: false }
    }

    pub fn with_position(mut self) -> Self {
        self.with_position = true; self
    }

    pub fn to_json(&self) -> Value {
        json!({
            "column": self.column,
            "index_type": "FTS",
            "with_position": self.with_position,
        })
    }
}

// ---- ANN index descriptor -----------------------------------------------

/// Describes an IVF_PQ ANN index configuration.
#[derive(Debug, Clone)]
pub struct AnnIndex {
    pub num_partitions: u32,
    pub num_sub_vectors: u32,
    pub metric: String,
}

impl AnnIndex {
    pub fn new(num_partitions: u32, num_sub_vectors: u32) -> Self {
        Self { num_partitions, num_sub_vectors, metric: "cosine".to_owned() }
    }

    pub fn metric(mut self, m: impl Into<String>) -> Self {
        self.metric = m.into(); self
    }

    pub fn to_json(&self) -> Value {
        json!({
            "index_type": "IVF_PQ",
            "num_partitions": self.num_partitions,
            "num_sub_vectors": self.num_sub_vectors,
            "metric": self.metric,
        })
    }
}

// ---- hybrid query descriptor --------------------------------------------

/// Describes a hybrid vector + full-text query.
#[derive(Debug, Clone)]
pub struct HybridQuery {
    pub table: String,
    pub vector: Vec<f32>,
    pub fts_query: String,
    pub limit: usize,
    pub reranker: String,
    pub filter: Option<String>,
}

impl HybridQuery {
    pub fn new(
        table: impl Into<String>,
        vector: Vec<f32>,
        fts_query: impl Into<String>,
        limit: usize,
    ) -> Self {
        Self {
            table: table.into(),
            vector,
            fts_query: fts_query.into(),
            limit,
            reranker: "rrf".to_owned(),
            filter: None,
        }
    }

    pub fn reranker(mut self, r: impl Into<String>) -> Self {
        self.reranker = r.into(); self
    }

    pub fn filter(mut self, sql: impl Into<String>) -> Self {
        self.filter = Some(sql.into()); self
    }

    pub fn to_json(&self) -> Value {
        let mut q = json!({
            "table": self.table,
            "vector": self.vector,
            "fts_query": self.fts_query,
            "limit": self.limit,
            "reranker": self.reranker,
        });
        if let Some(f) = &self.filter {
            q["filter"] = json!(f);
        }
        q
    }
}

// ---- versioning / checkout helpers --------------------------------------

/// Describes a version checkout operation.
#[derive(Debug, Clone)]
pub struct VersionCheckout {
    pub table: String,
    pub version: u64,
}

impl VersionCheckout {
    pub fn new(table: impl Into<String>, version: u64) -> Self {
        Self { table: table.into(), version }
    }

    pub fn to_json(&self) -> Value {
        json!({ "table": self.table, "version": self.version })
    }
}

/// Describes a time-travel checkout by timestamp (Unix seconds).
pub fn checkout_as_of(table: &str, unix_secs: u64) -> Value {
    json!({ "table": table, "as_of": unix_secs })
}

/// Describes a restore-to-version operation.
pub fn restore_version(table: &str, version: u64) -> Value {
    json!({ "table": table, "restore_to": version })
}

// ---- delete by predicate ------------------------------------------------

pub fn delete_predicate(table: &str, sql: &str) -> Value {
    json!({ "table": table, "predicate": sql })
}

// ---- multimodal column helpers ------------------------------------------

/// Build a multimodal row with both a text and an image embedding.
pub fn multimodal_row(
    id: i64,
    text_embedding: Vec<f32>,
    image_embedding: Vec<f32>,
    payload: Value,
) -> Value {
    json!({
        "id": id,
        "text_embedding": text_embedding,
        "image_embedding": image_embedding,
        "payload": payload.to_string(),
    })
}

// ---- response / result helpers ------------------------------------------

/// Parse result rows from a LanceDB search response (hypothetical JSON shape).
pub fn parse_results(body: &Value) -> Vec<(i64, f32, Value)> {
    body["rows"]
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .map(|row| {
            let id = row["id"].as_i64().unwrap_or(0);
            let score = row["_distance"].as_f64().unwrap_or(0.0) as f32;
            let payload = row["payload"]
                .as_str()
                .and_then(|s| serde_json::from_str(s).ok())
                .unwrap_or(json!({}));
            (id, score, payload)
        })
        .collect()
}

/// Parse the current version number from a table metadata response.
pub fn parse_version(body: &Value) -> u64 {
    body["version"].as_u64().unwrap_or(0)
}

/// Parse object-storage path detection from a config.
pub fn detect_storage_type(uri: &str) -> &'static str {
    if uri.starts_with("s3://") { "s3" }
    else if uri.starts_with("gs://") { "gcs" }
    else if uri.starts_with("az://") { "azure" }
    else { "local" }
}

// ---- edge single-binary default -----------------------------------------

/// Returns a sensible default LanceDB directory for edge / single-binary deployments.
pub fn edge_default_dir() -> String {
    std::env::var("ANCORA_LANCEDB_DIR").unwrap_or_else(|_| "./ancora_lancedb".to_owned())
}

/// Returns a `LanceDbConfig` suitable for an edge single-binary deployment.
/// Uses `ANCORA_LANCEDB_DIR` if set, falling back to `./ancora_lancedb`.
pub fn edge_config() -> LanceDbConfig {
    LanceDbConfig::local(edge_default_dir())
}

// ---- merge insert helpers -----------------------------------------------

/// Describes an upsert (merge-insert) operation -- update matching rows, insert new ones.
pub fn merge_insert_descriptor(table: &str, on_column: &str, batch: Vec<Value>) -> Value {
    json!({
        "table": table,
        "operation": "merge_insert",
        "on": on_column,
        "data": batch,
    })
}

// ---- table management helpers -------------------------------------------

/// Describes a table compaction operation (rewrites small files into larger ones).
pub fn compact_descriptor(table: &str) -> Value {
    json!({ "table": table, "operation": "compact_files" })
}

/// Describes a cleanup-old-versions operation.
pub fn cleanup_old_versions_descriptor(table: &str, older_than_days: u32) -> Value {
    json!({ "table": table, "operation": "cleanup_old_versions", "older_than_days": older_than_days })
}

/// Describes a table optimize operation (compaction + cleanup).
pub fn optimize_descriptor(table: &str) -> Value {
    json!({ "table": table, "operation": "optimize" })
}

// ---- unit tests ----------------------------------------------------------

#[cfg(test)]
mod lancedb_tests {
    use super::*;

    // ---- path tests -------------------------------------------------------

    #[test]
    fn local_path_is_local() {
        let p = LanceDbPath::local("/data/db");
        assert!(p.is_local());
        assert!(!p.is_remote());
    }

    #[test]
    fn s3_path_is_remote() {
        let p = LanceDbPath::s3("s3://bucket/prefix");
        assert!(p.is_remote());
        assert!(!p.is_local());
    }

    #[test]
    fn gcs_path_is_remote() {
        let p = LanceDbPath::gcs("gs://bucket/key");
        assert!(p.is_remote());
    }

    #[test]
    fn azure_path_is_remote() {
        let p = LanceDbPath::azure("az://container/blob");
        assert!(p.is_remote());
    }

    #[test]
    fn path_uri_returns_inner_string() {
        let p = LanceDbPath::local("/tmp/db");
        assert_eq!(p.uri(), "/tmp/db");
    }

    // ---- config tests -----------------------------------------------------

    #[test]
    fn config_local_sets_local_path() {
        let cfg = LanceDbConfig::local("/data");
        assert!(cfg.path.is_local());
    }

    #[test]
    fn config_s3_sets_aws_region() {
        let cfg = LanceDbConfig::s3("s3://b/p", "us-east-1");
        assert_eq!(cfg.aws_region.as_deref(), Some("us-east-1"));
    }

    #[test]
    fn config_read_only_sets_flag() {
        let cfg = LanceDbConfig::local("/data").read_only();
        assert!(cfg.read_only);
    }

    // ---- schema tests -----------------------------------------------------

    #[test]
    fn table_schema_includes_embedding_and_payload() {
        let schema = table_schema(128, &[]);
        let cols = schema["columns"].as_array().unwrap();
        assert!(cols.iter().any(|c| c["name"] == "embedding"), "missing embedding");
        assert!(cols.iter().any(|c| c["name"] == "payload"), "missing payload");
    }

    #[test]
    fn table_schema_extra_column_is_included() {
        let extra = ColumnDef::new("year", column_type::INT64).required();
        let schema = table_schema(128, &[extra]);
        let cols = schema["columns"].as_array().unwrap();
        assert!(cols.iter().any(|c| c["name"] == "year"), "missing year column");
    }

    #[test]
    fn column_def_nullable_default_is_true() {
        let col = ColumnDef::new("x", "utf8");
        assert!(col.nullable);
    }

    #[test]
    fn column_def_required_sets_nullable_false() {
        let col = ColumnDef::new("x", "utf8").required();
        assert!(!col.nullable);
    }

    // ---- row tests -------------------------------------------------------

    #[test]
    fn row_builder_sets_id_and_embedding() {
        let r = row(42, vec![0.1f32; 4], serde_json::json!({"k": "v"}));
        assert_eq!(r["id"], 42);
        assert_eq!(r["embedding"].as_array().unwrap().len(), 4);
    }

    #[test]
    fn rows_batch_has_correct_length() {
        let data = vec![
            (1i64, vec![0.1f32; 4], serde_json::json!({})),
            (2i64, vec![0.2f32; 4], serde_json::json!({})),
        ];
        assert_eq!(rows(&data).len(), 2);
    }

    #[test]
    fn row_with_columns_merges_extra_fields() {
        let r = row_with_columns(1, vec![0.1f32], serde_json::json!({}), serde_json::json!({"year": 2024}));
        assert_eq!(r["year"], 2024);
    }

    // ---- vector query tests ----------------------------------------------

    #[test]
    fn vector_query_to_json_includes_table_and_limit() {
        let q = VectorQuery::new("docs", vec![0.1f32; 4], 10);
        let j = q.to_json();
        assert_eq!(j["table"], "docs");
        assert_eq!(j["limit"], 10);
    }

    #[test]
    fn vector_query_filter_is_included_when_set() {
        let q = VectorQuery::new("docs", vec![0.1f32; 4], 5).filter("year > 2020");
        let j = q.to_json();
        assert_eq!(j["filter"], "year > 2020");
    }

    #[test]
    fn vector_query_ef_is_included_when_set() {
        let q = VectorQuery::new("docs", vec![0.1f32; 4], 5).ef(64);
        assert_eq!(q.to_json()["ef"], 64);
    }

    #[test]
    fn vector_query_metric_defaults_to_cosine() {
        let q = VectorQuery::new("docs", vec![0.1f32; 4], 5);
        assert_eq!(q.metric, "cosine");
    }

    // ---- SQL filter tests ------------------------------------------------

    #[test]
    fn sql_eq_str_produces_quoted_value() {
        assert_eq!(sql_eq_str("tag", "ml"), "tag = 'ml'");
    }

    #[test]
    fn sql_eq_str_escapes_single_quote() {
        let e = sql_eq_str("name", "it's");
        assert!(e.contains("it''s"), "e: {e}");
    }

    #[test]
    fn sql_and_wraps_both_clauses() {
        let e = sql_and("a = 1", "b = 2");
        assert_eq!(e, "(a = 1) AND (b = 2)");
    }

    #[test]
    fn sql_in_ints_formats_correctly() {
        assert_eq!(sql_in_ints("id", &[1, 2, 3]), "id IN (1, 2, 3)");
    }

    // ---- FTS index tests -------------------------------------------------

    #[test]
    fn full_text_index_json_has_index_type() {
        let idx = FullTextIndex::new("body").with_position();
        let j = idx.to_json();
        assert_eq!(j["index_type"], "FTS");
        assert_eq!(j["with_position"], true);
    }

    // ---- ANN index tests -------------------------------------------------

    #[test]
    fn ann_index_json_sets_num_partitions() {
        let idx = AnnIndex::new(256, 16).metric("l2");
        let j = idx.to_json();
        assert_eq!(j["num_partitions"], 256);
        assert_eq!(j["metric"], "l2");
    }

    // ---- hybrid query tests ----------------------------------------------

    #[test]
    fn hybrid_query_json_includes_fts_and_vector() {
        let q = HybridQuery::new("docs", vec![0.1f32; 4], "machine learning", 10);
        let j = q.to_json();
        assert_eq!(j["fts_query"], "machine learning");
        assert!(j["vector"].is_array());
    }

    #[test]
    fn hybrid_query_filter_propagates() {
        let q = HybridQuery::new("docs", vec![0.1f32], "rust", 5).filter("year > 2021");
        assert_eq!(q.to_json()["filter"], "year > 2021");
    }

    // ---- versioning tests ------------------------------------------------

    #[test]
    fn version_checkout_json_is_correct() {
        let vc = VersionCheckout::new("docs", 7);
        let j = vc.to_json();
        assert_eq!(j["version"], 7);
        assert_eq!(j["table"], "docs");
    }

    #[test]
    fn checkout_as_of_includes_timestamp() {
        let j = checkout_as_of("docs", 1700000000);
        assert_eq!(j["as_of"], 1700000000u64);
    }

    #[test]
    fn restore_version_json_is_correct() {
        let j = restore_version("docs", 3);
        assert_eq!(j["restore_to"], 3);
    }

    // ---- delete predicate tests ------------------------------------------

    #[test]
    fn delete_predicate_includes_table_and_sql() {
        let j = delete_predicate("docs", "score < 0.3");
        assert_eq!(j["predicate"], "score < 0.3");
        assert_eq!(j["table"], "docs");
    }

    // ---- multimodal tests ------------------------------------------------

    #[test]
    fn multimodal_row_has_both_embeddings() {
        let r = multimodal_row(1, vec![0.1f32], vec![0.2f32], serde_json::json!({}));
        assert!(r["text_embedding"].is_array());
        assert!(r["image_embedding"].is_array());
    }

    // ---- response parsing tests ------------------------------------------

    #[test]
    fn parse_results_extracts_rows() {
        let body = serde_json::json!({
            "rows": [
                { "id": 1, "_distance": 0.1, "payload": r#"{"k":"v"}"# },
                { "id": 2, "_distance": 0.9, "payload": r#"{"k":"w"}"# },
            ]
        });
        let results = parse_results(&body);
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].0, 1);
    }

    #[test]
    fn parse_version_extracts_number() {
        let body = serde_json::json!({ "version": 5 });
        assert_eq!(parse_version(&body), 5);
    }

    // ---- storage type detection ------------------------------------------

    #[test]
    fn detect_storage_type_local() {
        assert_eq!(detect_storage_type("/data/db"), "local");
    }

    #[test]
    fn detect_storage_type_s3() {
        assert_eq!(detect_storage_type("s3://my-bucket/key"), "s3");
    }

    #[test]
    fn detect_storage_type_gcs() {
        assert_eq!(detect_storage_type("gs://bucket/prefix"), "gcs");
    }

    #[test]
    fn detect_storage_type_azure() {
        assert_eq!(detect_storage_type("az://container/blob"), "azure");
    }
}
