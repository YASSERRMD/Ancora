/// pgvector backend for the `VectorStore` trait.
///
/// Requires the `pgvector` feature: `ancora-memory = { features = ["pgvector"] }`.
///
/// In tests, SQL generation is verified offline. Integration tests that need a
/// live Postgres are marked `#[ignore]` and require `TEST_DATABASE_URL` in the
/// environment.

// ---- connection config ---------------------------------------------------

/// Configuration for a pgvector connection.
#[derive(Debug, Clone)]
pub struct PgConfig {
    /// PostgreSQL connection string (e.g. `"postgres://user:pass@localhost/db"`).
    pub url: String,
    /// Maximum number of connections in the pool.
    pub pool_size: u32,
}

impl PgConfig {
    pub fn new(url: impl Into<String>) -> Self {
        Self { url: url.into(), pool_size: 5 }
    }
    pub fn with_pool_size(mut self, n: u32) -> Self {
        self.pool_size = n; self
    }
}

// ---- SQL DDL generation --------------------------------------------------

/// Generate a `CREATE TABLE` statement for a pgvector collection.
///
/// The table uses `vector(N)` as the embedding column type. A jsonb column
/// holds arbitrary payload.
pub fn create_table_sql(table: &str, dimensions: usize) -> String {
    format!(
        "CREATE TABLE IF NOT EXISTS {table} (\
         id BIGINT PRIMARY KEY, \
         embedding vector({dimensions}), \
         payload JSONB NOT NULL DEFAULT '{{}}'::jsonb\
         );"
    )
}

/// Validated HNSW index parameters.
#[derive(Debug, Clone, Copy)]
pub struct HnswParams {
    /// Number of bi-directional links per layer (range 2-100, default 16).
    pub m: u16,
    /// Size of the dynamic candidate list during construction (range 4-1000, default 100).
    pub ef_construct: u16,
}

impl HnswParams {
    pub fn new(m: u16, ef_construct: u16) -> Result<Self, String> {
        if !(2..=100).contains(&m) {
            return Err(format!("m={m} out of range [2, 100]"));
        }
        if !(4..=1000).contains(&ef_construct) {
            return Err(format!("ef_construct={ef_construct} out of range [4, 1000]"));
        }
        Ok(Self { m, ef_construct })
    }
}

impl Default for HnswParams {
    fn default() -> Self { Self { m: 16, ef_construct: 100 } }
}

/// Generate an HNSW index creation statement for cosine similarity.
pub fn create_hnsw_index_sql(table: &str, m: u16, ef_construct: u16) -> String {
    format!(
        "CREATE INDEX IF NOT EXISTS {table}_embedding_idx \
         ON {table} USING hnsw (embedding vector_cosine_ops) \
         WITH (m = {m}, ef_construction = {ef_construct});"
    )
}

/// Generate an HNSW index for a specific distance operator class.
pub fn create_hnsw_index_with_ops_sql(table: &str, params: &HnswParams, ops: &str) -> String {
    format!(
        "CREATE INDEX IF NOT EXISTS {table}_embedding_idx \
         ON {table} USING hnsw (embedding {ops}) \
         WITH (m = {m}, ef_construction = {ef_construct});",
        m = params.m, ef_construct = params.ef_construct
    )
}

/// Validated IVF-Flat index parameters.
#[derive(Debug, Clone, Copy)]
pub struct IvfFlatParams {
    /// Number of inverted file lists (clusters). Typical range 1-1000.
    pub lists: u32,
    /// Number of probes during search. Higher = better recall, slower query.
    pub probes: u32,
}

impl IvfFlatParams {
    pub fn new(lists: u32, probes: u32) -> Result<Self, String> {
        if lists == 0 { return Err("lists must be >= 1".to_owned()); }
        if probes == 0 { return Err("probes must be >= 1".to_owned()); }
        if probes > lists { return Err(format!("probes({probes}) > lists({lists}) is wasteful")); }
        Ok(Self { lists, probes })
    }

    /// Recommended default: sqrt(row_count) for lists, 10% of lists for probes.
    pub fn for_table_size(row_count: u64) -> Self {
        let lists = ((row_count as f64).sqrt() as u32).max(1);
        let probes = (lists / 10).max(1);
        Self { lists, probes }
    }
}

impl Default for IvfFlatParams {
    fn default() -> Self { Self { lists: 100, probes: 10 } }
}

/// Generate an IVF-Flat index creation statement.
pub fn create_ivfflat_index_sql(table: &str, lists: u32) -> String {
    format!(
        "CREATE INDEX IF NOT EXISTS {table}_embedding_ivf_idx \
         ON {table} USING ivfflat (embedding vector_cosine_ops) \
         WITH (lists = {lists});"
    )
}

/// Generate a `SET ivfflat.probes` session setting SQL.
pub fn set_ivfflat_probes_sql(probes: u32) -> String {
    format!("SET ivfflat.probes = {probes};")
}

/// Generate a `SET hnsw.ef_search` session setting SQL.
pub fn set_hnsw_ef_search_sql(ef_search: u16) -> String {
    format!("SET hnsw.ef_search = {ef_search};")
}

/// Generate a `DROP TABLE` statement.
pub fn drop_table_sql(table: &str) -> String {
    format!("DROP TABLE IF EXISTS {table};")
}

/// Generate a `SELECT COUNT(*) FROM table` for describe_collection.
pub fn count_sql(table: &str) -> String {
    format!("SELECT COUNT(*) FROM {table};")
}

/// Generate a `SELECT` to retrieve dimension information from the table.
pub fn dimension_query_sql(table: &str) -> String {
    format!(
        "SELECT vector_dims(embedding) AS dimensions \
         FROM {table} LIMIT 1;"
    )
}

/// Sanitize an identifier to prevent SQL injection in table names.
///
/// Only allows alphanumeric characters and underscores.
pub fn sanitize_identifier(name: &str) -> Result<&str, String> {
    if name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
        Ok(name)
    } else {
        Err(format!("invalid identifier `{name}`: only [a-zA-Z0-9_] allowed"))
    }
}

/// Return the canonical pgvector operator for a given distance metric name.
pub fn distance_operator(metric: &str) -> &'static str {
    match metric {
        "cosine" => "<=>",
        "dot" => "<#>",
        "l2" => "<->",
        _ => "<=>",
    }
}

// ---- DML generation ------------------------------------------------------

/// Generate an upsert statement using `ON CONFLICT (id) DO UPDATE`.
pub fn upsert_sql(table: &str) -> String {
    format!(
        "INSERT INTO {table} (id, embedding, payload) VALUES ($1, $2, $3) \
         ON CONFLICT (id) DO UPDATE SET embedding = EXCLUDED.embedding, payload = EXCLUDED.payload;"
    )
}

/// Generate a cosine similarity query with optional LIMIT and OFFSET.
pub fn cosine_query_sql(table: &str, limit: usize, offset: usize) -> String {
    format!(
        "SELECT id, payload, 1 - (embedding <=> $1) AS score \
         FROM {table} \
         ORDER BY embedding <=> $1 \
         LIMIT {limit} OFFSET {offset};"
    )
}

/// Generate a dot-product similarity query.
pub fn dot_query_sql(table: &str, limit: usize) -> String {
    format!(
        "SELECT id, payload, (embedding <#> $1) * -1 AS score \
         FROM {table} \
         ORDER BY embedding <#> $1 \
         LIMIT {limit};"
    )
}

/// Generate an L2 distance query.
pub fn l2_query_sql(table: &str, limit: usize) -> String {
    format!(
        "SELECT id, payload, 1 / (1 + (embedding <-> $1)) AS score \
         FROM {table} \
         ORDER BY embedding <-> $1 \
         LIMIT {limit};"
    )
}

/// Generate a DELETE statement for explicit IDs.
pub fn delete_by_ids_sql(table: &str, count: usize) -> String {
    let params: Vec<String> = (1..=count).map(|i| format!("${i}")).collect();
    format!("DELETE FROM {table} WHERE id IN ({});", params.join(", "))
}

// ---- filter-to-SQL mapping -----------------------------------------------

use crate::vector_store::{Filter, PayloadValue};

/// Translate a `Filter` into a SQL WHERE clause fragment.
///
/// Uses jsonb operators for payload access. Returns `("", vec![])` for no filter.
pub fn filter_to_sql(filter: &Filter, param_offset: usize) -> (String, Vec<FilterParam>) {
    filter_to_sql_inner(filter, param_offset)
}

/// A bound parameter value for the SQL WHERE clause.
#[derive(Debug, Clone)]
pub enum FilterParam {
    Text(String),
    Int(i64),
    Float(f64),
    Bool(bool),
}

fn filter_to_sql_inner(filter: &Filter, offset: usize) -> (String, Vec<FilterParam>) {
    match filter {
        Filter::Eq(key, val) => {
            let (sql, params) = payload_op(key, "=", val, offset);
            (sql, params)
        }
        Filter::Ne(key, val) => {
            let (sql, params) = payload_op(key, "!=", val, offset);
            (sql, params)
        }
        Filter::Gt(key, val) => {
            let (sql, params) = payload_numeric_op(key, ">", val, offset);
            (sql, params)
        }
        Filter::Lt(key, val) => {
            let (sql, params) = payload_numeric_op(key, "<", val, offset);
            (sql, params)
        }
        Filter::And(a, b) => {
            let (sql_a, params_a) = filter_to_sql_inner(a, offset);
            let (sql_b, params_b) = filter_to_sql_inner(b, offset + params_a.len());
            let mut params = params_a;
            params.extend(params_b);
            (format!("({sql_a} AND {sql_b})"), params)
        }
        Filter::Or(a, b) => {
            let (sql_a, params_a) = filter_to_sql_inner(a, offset);
            let (sql_b, params_b) = filter_to_sql_inner(b, offset + params_a.len());
            let mut params = params_a;
            params.extend(params_b);
            (format!("({sql_a} OR {sql_b})"), params)
        }
    }
}

fn payload_op(key: &str, op: &str, val: &PayloadValue, offset: usize) -> (String, Vec<FilterParam>) {
    let idx = offset + 1;
    match val {
        PayloadValue::String(s) => (
            format!("payload->>'{key}' {op} ${idx}"),
            vec![FilterParam::Text(s.clone())],
        ),
        PayloadValue::Integer(n) => (
            format!("(payload->>'{key}')::bigint {op} ${idx}"),
            vec![FilterParam::Int(*n)],
        ),
        PayloadValue::Float(f) => (
            format!("(payload->>'{key}')::float {op} ${idx}"),
            vec![FilterParam::Float(*f)],
        ),
        PayloadValue::Bool(b) => (
            format!("(payload->>'{key}')::boolean {op} ${idx}"),
            vec![FilterParam::Bool(*b)],
        ),
        PayloadValue::Null => (format!("payload->>'{key}' IS NULL"), vec![]),
    }
}

fn payload_numeric_op(key: &str, op: &str, val: &PayloadValue, offset: usize) -> (String, Vec<FilterParam>) {
    payload_op(key, op, val, offset)
}

// ---- hybrid: tsvector keyword search ------------------------------------

/// Generate a hybrid search SQL using both cosine similarity and `tsvector` keyword ranking.
///
/// The `alpha` parameter blends the scores. Result columns: id, payload, score.
pub fn hybrid_query_sql(table: &str, limit: usize, alpha: f32) -> String {
    let beta = 1.0 - alpha;
    format!(
        "SELECT id, payload, \
         ({alpha} * (1 - (embedding <=> $1)) + {beta} * ts_rank(to_tsvector(payload->>'text'), plainto_tsquery($2))) AS score \
         FROM {table} \
         ORDER BY score DESC \
         LIMIT {limit};"
    )
}

// ---- tests (all offline) ------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vector_store::{Filter, PayloadValue};

    #[test]
    fn create_table_sql_contains_vector_type() {
        let sql = create_table_sql("docs", 1536);
        assert!(sql.contains("vector(1536)"), "SQL: {sql}");
        assert!(sql.contains("BIGINT PRIMARY KEY"));
    }

    #[test]
    fn create_hnsw_index_sql_params() {
        let sql = create_hnsw_index_sql("docs", 16, 100);
        assert!(sql.contains("m = 16") || sql.contains("m=16"), "SQL: {sql}");
    }

    #[test]
    fn hnsw_params_validation_rejects_out_of_range_m() {
        assert!(HnswParams::new(1, 100).is_err(), "m=1 should fail");
        assert!(HnswParams::new(101, 100).is_err(), "m=101 should fail");
        assert!(HnswParams::new(16, 100).is_ok());
    }

    #[test]
    fn hnsw_params_validation_rejects_out_of_range_ef() {
        assert!(HnswParams::new(16, 3).is_err(), "ef_construct=3 should fail");
        assert!(HnswParams::new(16, 1001).is_err(), "ef_construct=1001 should fail");
        assert!(HnswParams::new(16, 200).is_ok());
    }

    #[test]
    fn hnsw_index_with_l2_ops_contains_correct_class() {
        let params = HnswParams::default();
        let sql = create_hnsw_index_with_ops_sql("docs", &params, "vector_l2_ops");
        assert!(sql.contains("vector_l2_ops"), "SQL: {sql}");
        assert!(sql.contains("USING hnsw"));
    }

    #[test]
    fn sanitize_identifier_allows_alphanumeric_underscore() {
        assert!(sanitize_identifier("my_table_123").is_ok());
        assert!(sanitize_identifier("bad-table").is_err());
        assert!(sanitize_identifier("drop; table").is_err());
    }

    #[test]
    fn distance_operator_returns_correct_symbol() {
        assert_eq!(distance_operator("cosine"), "<=>");
        assert_eq!(distance_operator("dot"), "<#>");
        assert_eq!(distance_operator("l2"), "<->");
        assert_eq!(distance_operator("unknown"), "<=>"); // default to cosine
    }

    #[test]
    fn drop_table_sql_format() {
        assert_eq!(drop_table_sql("docs"), "DROP TABLE IF EXISTS docs;");
    }

    #[test]
    fn ivfflat_params_rejects_zero_lists() {
        assert!(IvfFlatParams::new(0, 5).is_err());
    }

    #[test]
    fn ivfflat_params_rejects_probes_greater_than_lists() {
        assert!(IvfFlatParams::new(10, 15).is_err());
        assert!(IvfFlatParams::new(10, 10).is_ok());
    }

    #[test]
    fn ivfflat_params_for_table_size() {
        let p = IvfFlatParams::for_table_size(10_000);
        assert_eq!(p.lists, 100); // sqrt(10_000)
        assert!(p.probes <= p.lists);
    }

    #[test]
    fn set_ivfflat_probes_sql_correct() {
        assert_eq!(set_ivfflat_probes_sql(5), "SET ivfflat.probes = 5;");
    }

    #[test]
    fn set_hnsw_ef_search_sql_correct() {
        assert_eq!(set_hnsw_ef_search_sql(64), "SET hnsw.ef_search = 64;");
    }

    #[test]
    fn upsert_sql_has_on_conflict() {
        let sql = upsert_sql("docs");
        assert!(sql.contains("ON CONFLICT"), "SQL: {sql}");
    }

    #[test]
    fn cosine_query_sql_has_cosine_op() {
        let sql = cosine_query_sql("docs", 10, 0);
        assert!(sql.contains("<=>"), "SQL: {sql}");
        assert!(sql.contains("LIMIT 10"));
    }

    #[test]
    fn delete_by_ids_sql_placeholders() {
        let sql = delete_by_ids_sql("docs", 3);
        assert!(sql.contains("$1") && sql.contains("$2") && sql.contains("$3"), "SQL: {sql}");
    }

    #[test]
    fn filter_to_sql_eq_string() {
        let f = Filter::Eq("source".to_owned(), PayloadValue::String("wiki".to_owned()));
        let (sql, params) = filter_to_sql(&f, 0);
        assert!(sql.contains("payload->>'source'"));
        assert!(matches!(params[0], FilterParam::Text(ref s) if s == "wiki"));
    }

    #[test]
    fn filter_to_sql_and_compound() {
        let f = Filter::Eq("a".to_owned(), PayloadValue::String("x".to_owned()))
            .and(Filter::Gt("n".to_owned(), PayloadValue::Integer(5)));
        let (sql, params) = filter_to_sql(&f, 0);
        assert!(sql.contains("AND"), "SQL: {sql}");
        assert_eq!(params.len(), 2);
    }
}
