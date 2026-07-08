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
        Self {
            url: url.into(),
            pool_size: 5,
        }
    }
    pub fn with_pool_size(mut self, n: u32) -> Self {
        self.pool_size = n;
        self
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
            return Err(format!(
                "ef_construct={ef_construct} out of range [4, 1000]"
            ));
        }
        Ok(Self { m, ef_construct })
    }
}

impl Default for HnswParams {
    fn default() -> Self {
        Self {
            m: 16,
            ef_construct: 100,
        }
    }
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
        m = params.m,
        ef_construct = params.ef_construct
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
        if lists == 0 {
            return Err("lists must be >= 1".to_owned());
        }
        if probes == 0 {
            return Err("probes must be >= 1".to_owned());
        }
        if probes > lists {
            return Err(format!("probes({probes}) > lists({lists}) is wasteful"));
        }
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
    fn default() -> Self {
        Self {
            lists: 100,
            probes: 10,
        }
    }
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
        Err(format!(
            "invalid identifier `{name}`: only [a-zA-Z0-9_] allowed"
        ))
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

/// Serialize a `Payload` map to a JSON string suitable for the JSONB column.
pub fn serialize_payload(payload: &crate::vector_store::Payload) -> Result<String, String> {
    use crate::vector_store::PayloadValue;
    let mut obj = serde_json::Map::new();
    for (k, v) in payload {
        let jv = match v {
            PayloadValue::String(s) => serde_json::Value::String(s.clone()),
            PayloadValue::Integer(n) => serde_json::Value::Number((*n).into()),
            PayloadValue::Float(f) => serde_json::Value::Number(
                serde_json::Number::from_f64(*f)
                    .ok_or_else(|| format!("non-finite float for key `{k}`"))?,
            ),
            PayloadValue::Bool(b) => serde_json::Value::Bool(*b),
            PayloadValue::Null => serde_json::Value::Null,
        };
        obj.insert(k.clone(), jv);
    }
    serde_json::to_string(&obj).map_err(|e| e.to_string())
}

/// Deserialize a JSON string from the JSONB column back to a `Payload` map.
pub fn deserialize_payload(json: &str) -> Result<crate::vector_store::Payload, String> {
    use crate::vector_store::PayloadValue;
    let obj: serde_json::Map<String, serde_json::Value> =
        serde_json::from_str(json).map_err(|e| e.to_string())?;
    let mut payload = crate::vector_store::Payload::new();
    for (k, v) in obj {
        let pv = match v {
            serde_json::Value::String(s) => PayloadValue::String(s),
            serde_json::Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    PayloadValue::Integer(i)
                } else {
                    PayloadValue::Float(n.as_f64().unwrap_or(0.0))
                }
            }
            serde_json::Value::Bool(b) => PayloadValue::Bool(b),
            serde_json::Value::Null => PayloadValue::Null,
            _ => PayloadValue::Null,
        };
        payload.insert(k, pv);
    }
    Ok(payload)
}

/// Encode an embedding as a pgvector literal string: `[0.1,0.2,0.3]`.
pub fn encode_vector(embedding: &[f32]) -> String {
    let inner: Vec<String> = embedding.iter().map(|v| format!("{v}")).collect();
    format!("[{}]", inner.join(","))
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

/// Generate a cosine similarity query with a score threshold applied in SQL.
///
/// Only rows where `1 - cosine_distance >= threshold` are returned. This
/// lets Postgres prune results before transferring them over the wire.
pub fn cosine_query_with_threshold_sql(
    table: &str,
    limit: usize,
    offset: usize,
    threshold: f32,
) -> String {
    format!(
        "SELECT id, payload, 1 - (embedding <=> $1) AS score \
         FROM {table} \
         WHERE 1 - (embedding <=> $1) >= {threshold} \
         ORDER BY embedding <=> $1 \
         LIMIT {limit} OFFSET {offset};"
    )
}

/// Generate a filtered cosine query (WHERE clause inserted before ORDER BY).
///
/// `filter_sql` is a fragment like `payload->>'lang' = $2`. The embedding
/// vector is always `$1`; filter params start at `param_offset + 1`.
pub fn cosine_query_with_filter_sql(
    table: &str,
    limit: usize,
    offset: usize,
    filter_sql: &str,
) -> String {
    format!(
        "SELECT id, payload, 1 - (embedding <=> $1) AS score \
         FROM {table} \
         WHERE {filter_sql} \
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

/// Generate a dot-product query with score threshold.
///
/// Note: pgvector's `<#>` returns the negative dot product, so
/// `(embedding <#> $1) * -1 >= threshold` is the WHERE condition.
pub fn dot_query_with_threshold_sql(table: &str, limit: usize, threshold: f32) -> String {
    format!(
        "SELECT id, payload, (embedding <#> $1) * -1 AS score \
         FROM {table} \
         WHERE (embedding <#> $1) * -1 >= {threshold} \
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

/// Generate an L2 distance query with score threshold.
///
/// Score is `1 / (1 + distance)`, so threshold applies to that derived value.
pub fn l2_query_with_threshold_sql(table: &str, limit: usize, threshold: f32) -> String {
    format!(
        "SELECT id, payload, 1 / (1 + (embedding <-> $1)) AS score \
         FROM {table} \
         WHERE 1 / (1 + (embedding <-> $1)) >= {threshold} \
         ORDER BY embedding <-> $1 \
         LIMIT {limit};"
    )
}

/// Generate a query dispatched on distance metric name.
pub fn metric_query_sql(metric: &str, table: &str, limit: usize, offset: usize) -> String {
    match metric {
        "dot" => dot_query_sql(table, limit),
        "l2" => l2_query_sql(table, limit),
        _ => cosine_query_sql(table, limit, offset),
    }
}

// ---- batch upsert via COPY -----------------------------------------------

/// Generate a `COPY ... FROM STDIN` SQL for bulk inserts.
///
/// The COPY format expects tab-separated: `id\tembedding_literal\tpayload_json`.
/// After the COPY completes, a MERGE or INSERT ON CONFLICT must reconcile
/// the staging table with the main table.
pub fn copy_into_staging_sql(staging: &str) -> String {
    format!(
        "COPY {staging} (id, embedding, payload) FROM STDIN WITH (FORMAT text, DELIMITER E'\\t');"
    )
}

/// Generate the staging table DDL that mirrors a collection's schema.
pub fn create_staging_table_sql(staging: &str, main: &str) -> String {
    format!("CREATE TEMP TABLE IF NOT EXISTS {staging} (LIKE {main} INCLUDING ALL);")
}

/// Generate the MERGE from staging -> main to perform the upsert.
pub fn merge_from_staging_sql(main: &str, staging: &str) -> String {
    format!(
        "INSERT INTO {main} (id, embedding, payload) \
         SELECT id, embedding, payload FROM {staging} \
         ON CONFLICT (id) DO UPDATE SET \
         embedding = EXCLUDED.embedding, payload = EXCLUDED.payload;"
    )
}

/// Encode a single row for tab-separated COPY input.
///
/// Returns `"id\t[x,y,...]\t{json}"`.
pub fn encode_copy_row(id: i64, embedding: &[f32], payload_json: &str) -> String {
    format!("{id}\t{}\t{payload_json}", encode_vector(embedding))
}

/// Split a large upsert payload into batches of at most `batch_size` points.
pub fn split_into_batches<T: Clone>(items: Vec<T>, batch_size: usize) -> Vec<Vec<T>> {
    if batch_size == 0 {
        return vec![items];
    }
    items.chunks(batch_size).map(|c| c.to_vec()).collect()
}

/// Generate a DELETE statement for explicit IDs.
pub fn delete_by_ids_sql(table: &str, count: usize) -> String {
    let params: Vec<String> = (1..=count).map(|i| format!("${i}")).collect();
    format!("DELETE FROM {table} WHERE id IN ({});", params.join(", "))
}

/// Generate a DELETE-with-RETURNING statement for filter-based deletes.
///
/// Returns the SQL and the bound params from the filter. The RETURNING clause
/// lets the caller count deleted rows without an extra SELECT.
pub fn delete_by_filter_sql(table: &str, filter: &Filter) -> (String, Vec<FilterParam>) {
    let (where_fragment, params) = filter_to_sql(filter, 0);
    let sql = if where_fragment.is_empty() {
        format!("DELETE FROM {table} RETURNING id;")
    } else {
        format!("DELETE FROM {table} WHERE {where_fragment} RETURNING id;")
    };
    (sql, params)
}

/// Generate a SELECT COUNT(*) query filtered by a WHERE clause fragment.
///
/// Used to preview how many rows a filter would delete before committing.
pub fn count_by_filter_sql(table: &str, filter: &Filter) -> (String, Vec<FilterParam>) {
    let (where_fragment, params) = filter_to_sql(filter, 0);
    let sql = if where_fragment.is_empty() {
        format!("SELECT COUNT(*) FROM {table};")
    } else {
        format!("SELECT COUNT(*) FROM {table} WHERE {where_fragment};")
    };
    (sql, params)
}

// ---- connection error classification and reconnect policy ----------------

/// Classify a postgres error code string into a reconnect decision.
#[derive(Debug, PartialEq)]
pub enum ConnectAction {
    /// The error is transient; retry after backing off.
    Retry,
    /// The operation should be retried in a new transaction.
    RetryTx,
    /// The error is permanent; do not retry.
    Abort,
}

/// Classify a Postgres SQLSTATE error code for retry decisions.
///
/// SQLSTATE classes are standardized; pgvector does not add new ones.
pub fn classify_pg_error(sqlstate: &str) -> ConnectAction {
    match sqlstate {
        // connection-level transients
        "08000" | "08003" | "08006" | "08001" | "08004" => ConnectAction::Retry,
        // serialization failures and deadlocks
        "40001" | "40P01" => ConnectAction::RetryTx,
        // permanent errors: syntax, constraint, no table, etc.
        "42000" | "42P01" | "42601" | "23000" | "23503" | "23505" => ConnectAction::Abort,
        // undefined -- treat as transient with caution
        _ => ConnectAction::Retry,
    }
}

/// Exponential-backoff delay in milliseconds for retry attempt `n` (0-indexed).
///
/// Caps at 30 seconds. Offline tests can assert the schedule without sleeping.
pub fn retry_delay_ms(attempt: u32) -> u64 {
    let base: u64 = 100;
    let cap: u64 = 30_000;
    let delay = base.saturating_mul(2u64.saturating_pow(attempt));
    delay.min(cap)
}

/// Maximum number of connection retries before returning an error.
pub const MAX_CONNECT_RETRIES: u32 = 5;

// ---- transactional journal table ----------------------------------------

/// Generate DDL for the upsert journal table.
///
/// The journal records every upsert attempt with a deduplication key so
/// retried writes are idempotent. A cleanup query can prune rows older than
/// a given timestamp.
pub fn create_journal_table_sql(journal: &str) -> String {
    format!(
        "CREATE TABLE IF NOT EXISTS {journal} (\
         idempotency_key TEXT PRIMARY KEY, \
         collection TEXT NOT NULL, \
         committed_at TIMESTAMPTZ NOT NULL DEFAULT NOW(), \
         row_count INT NOT NULL\
         );"
    )
}

/// Generate the SQL that records a completed upsert batch in the journal.
pub fn insert_journal_sql(journal: &str) -> String {
    format!(
        "INSERT INTO {journal} (idempotency_key, collection, row_count) \
         VALUES ($1, $2, $3) \
         ON CONFLICT (idempotency_key) DO NOTHING;"
    )
}

/// Generate the SQL that checks if an idempotency key has already been processed.
pub fn check_journal_sql(journal: &str) -> String {
    format!("SELECT row_count FROM {journal} WHERE idempotency_key = $1;")
}

/// Generate a cleanup statement that removes journal entries older than an interval.
pub fn purge_journal_sql(journal: &str) -> String {
    format!("DELETE FROM {journal} WHERE committed_at < NOW() - $1::interval;")
}

// ---- filter-to-SQL mapping -----------------------------------------------

use crate::vector_store::{Filter, PayloadValue};

/// Validate that a filter does not exceed maximum nesting depth.
///
/// Deep nesting can produce very long SQL WHERE clauses; 16 levels is enough
/// for any realistic agent memory query.
pub fn validate_filter_depth(filter: &Filter) -> Result<(), String> {
    fn depth(f: &Filter, n: u8) -> Result<u8, String> {
        if n > 16 {
            return Err("filter exceeds maximum nesting depth (16)".to_owned());
        }
        match f {
            Filter::And(a, b) | Filter::Or(a, b) => {
                let da = depth(a, n + 1)?;
                let db = depth(b, n + 1)?;
                Ok(da.max(db))
            }
            _ => Ok(n),
        }
    }
    depth(filter, 0).map(|_| ())
}

/// Build a complete WHERE clause from a filter, including the keyword.
///
/// Returns `None` when the filter list is empty (no WHERE needed).
pub fn build_where_clause(filter: &Filter, param_offset: usize) -> (String, Vec<FilterParam>) {
    let (fragment, params) = filter_to_sql(filter, param_offset);
    if fragment.is_empty() {
        (String::new(), params)
    } else {
        (format!("WHERE {fragment}"), params)
    }
}

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

fn payload_op(
    key: &str,
    op: &str,
    val: &PayloadValue,
    offset: usize,
) -> (String, Vec<FilterParam>) {
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

fn payload_numeric_op(
    key: &str,
    op: &str,
    val: &PayloadValue,
    offset: usize,
) -> (String, Vec<FilterParam>) {
    payload_op(key, op, val, offset)
}

// ---- hybrid: tsvector keyword search ------------------------------------

/// Clamp alpha to [0, 1] for hybrid score blending.
fn clamp_alpha(alpha: f32) -> f32 {
    alpha.clamp(0.0, 1.0)
}

/// Generate a hybrid search SQL using both cosine similarity and `tsvector` keyword ranking.
///
/// The `alpha` parameter blends the scores. Result columns: id, payload, score.
pub fn hybrid_query_sql(table: &str, limit: usize, alpha: f32) -> String {
    let alpha = clamp_alpha(alpha);
    let beta = 1.0 - alpha;
    format!(
        "SELECT id, payload, \
         ({alpha} * (1 - (embedding <=> $1)) + {beta} * ts_rank(to_tsvector(payload->>'text'), plainto_tsquery($2))) AS score \
         FROM {table} \
         ORDER BY score DESC \
         LIMIT {limit};"
    )
}

/// Generate a hybrid query that searches a specific text column name.
///
/// Use this when the text is stored under a key other than `text` in the payload.
pub fn hybrid_query_column_sql(table: &str, text_key: &str, limit: usize, alpha: f32) -> String {
    let alpha = clamp_alpha(alpha);
    let beta = 1.0 - alpha;
    format!(
        "SELECT id, payload, \
         ({alpha} * (1 - (embedding <=> $1)) + {beta} * ts_rank(to_tsvector(payload->>'{text_key}'), plainto_tsquery($2))) AS score \
         FROM {table} \
         ORDER BY score DESC \
         LIMIT {limit};"
    )
}

/// Generate a hybrid query with an additional filter clause.
pub fn hybrid_query_with_filter_sql(
    table: &str,
    limit: usize,
    alpha: f32,
    filter_sql: &str,
) -> String {
    let alpha = clamp_alpha(alpha);
    let beta = 1.0 - alpha;
    format!(
        "SELECT id, payload, \
         ({alpha} * (1 - (embedding <=> $1)) + {beta} * ts_rank(to_tsvector(payload->>'text'), plainto_tsquery($2))) AS score \
         FROM {table} \
         WHERE {filter_sql} \
         ORDER BY score DESC \
         LIMIT {limit};"
    )
}

/// Validate that an alpha value is in the expected [0,1] range.
pub fn validate_alpha(alpha: f32) -> Result<f32, String> {
    if (0.0..=1.0).contains(&alpha) {
        Ok(alpha)
    } else {
        Err(format!("alpha={alpha} outside [0.0, 1.0]"))
    }
}

// ---- score threshold helpers --------------------------------------------

/// A per-metric score threshold validator.
///
/// Cosine/dot scores are in (-1, 1); L2 scores are in (0, 1] because of
/// the `1/(1+dist)` transform. Rejects values outside metric-specific range.
pub struct ThresholdValidator;

impl ThresholdValidator {
    pub fn validate_cosine(threshold: f32) -> Result<(), String> {
        if (-1.0..=1.0).contains(&threshold) {
            Ok(())
        } else {
            Err(format!("cosine threshold={threshold} outside [-1.0, 1.0]"))
        }
    }

    pub fn validate_dot(threshold: f32) -> Result<(), String> {
        // dot product is theoretically unbounded, but in practice [0,1] for normalized vecs
        if threshold.is_finite() {
            Ok(())
        } else {
            Err("dot threshold must be finite".to_owned())
        }
    }

    pub fn validate_l2(threshold: f32) -> Result<(), String> {
        if (0.0..=1.0).contains(&threshold) {
            Ok(())
        } else {
            Err(format!("l2 threshold={threshold} outside [0.0, 1.0]"))
        }
    }

    pub fn validate(metric: &str, threshold: f32) -> Result<(), String> {
        match metric {
            "dot" => Self::validate_dot(threshold),
            "l2" => Self::validate_l2(threshold),
            _ => Self::validate_cosine(threshold),
        }
    }
}

/// Apply an in-memory score threshold filter to a result set.
///
/// Used when the backend cannot push threshold into SQL (e.g. for the
/// dot/L2 metrics where the WHERE expression is verbose).
pub fn apply_threshold(results: Vec<(i64, f32)>, threshold: f32) -> Vec<(i64, f32)> {
    results
        .into_iter()
        .filter(|(_, score)| *score >= threshold)
        .collect()
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
        assert!(
            HnswParams::new(16, 3).is_err(),
            "ef_construct=3 should fail"
        );
        assert!(
            HnswParams::new(16, 1001).is_err(),
            "ef_construct=1001 should fail"
        );
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
    fn serialize_payload_round_trips() {
        use crate::vector_store::{Payload, PayloadValue};
        let mut p = Payload::new();
        p.insert("name".to_owned(), PayloadValue::String("test".to_owned()));
        p.insert("count".to_owned(), PayloadValue::Integer(42));
        p.insert("flag".to_owned(), PayloadValue::Bool(true));
        let json = serialize_payload(&p).unwrap();
        let back = deserialize_payload(&json).unwrap();
        assert_eq!(
            back.get("name"),
            Some(&PayloadValue::String("test".to_owned()))
        );
        assert_eq!(back.get("count"), Some(&PayloadValue::Integer(42)));
        assert_eq!(back.get("flag"), Some(&PayloadValue::Bool(true)));
    }

    #[test]
    fn encode_vector_format() {
        let v = encode_vector(&[0.1, 0.2, 0.3]);
        assert!(v.starts_with('[') && v.ends_with(']'));
        assert!(v.contains("0.1"));
    }

    #[test]
    fn cosine_query_sql_has_cosine_op() {
        let sql = cosine_query_sql("docs", 10, 0);
        assert!(sql.contains("<=>"), "SQL: {sql}");
        assert!(sql.contains("LIMIT 10"));
    }

    #[test]
    fn cosine_query_with_threshold_has_where() {
        let sql = cosine_query_with_threshold_sql("docs", 5, 0, 0.75);
        assert!(sql.contains("WHERE"), "SQL: {sql}");
        assert!(sql.contains("0.75") || sql.contains(">="), "SQL: {sql}");
    }

    #[test]
    fn cosine_query_with_filter_contains_filter_fragment() {
        let sql = cosine_query_with_filter_sql("docs", 5, 0, "payload->>'lang' = $2");
        assert!(sql.contains("payload->>'lang'"), "SQL: {sql}");
        assert!(sql.contains("ORDER BY"), "SQL: {sql}");
    }

    #[test]
    fn delete_by_ids_sql_placeholders() {
        let sql = delete_by_ids_sql("docs", 3);
        assert!(
            sql.contains("$1") && sql.contains("$2") && sql.contains("$3"),
            "SQL: {sql}"
        );
    }

    #[test]
    fn delete_by_filter_sql_has_returning() {
        let f = Filter::Eq("tag".to_owned(), PayloadValue::String("old".to_owned()));
        let (sql, params) = delete_by_filter_sql("docs", &f);
        assert!(sql.contains("DELETE FROM docs"), "SQL: {sql}");
        assert!(sql.contains("RETURNING id"), "SQL: {sql}");
        assert!(!params.is_empty());
    }

    #[test]
    fn count_by_filter_sql_has_count_star() {
        let f = Filter::Gt("year".to_owned(), PayloadValue::Integer(2020));
        let (sql, params) = count_by_filter_sql("docs", &f);
        assert!(sql.contains("COUNT(*)"), "SQL: {sql}");
        assert!(!params.is_empty());
    }

    #[test]
    fn copy_into_staging_sql_has_from_stdin() {
        let sql = copy_into_staging_sql("docs_staging");
        assert!(sql.contains("FROM STDIN"), "SQL: {sql}");
    }

    #[test]
    fn merge_from_staging_sql_has_on_conflict() {
        let sql = merge_from_staging_sql("docs", "docs_staging");
        assert!(sql.contains("ON CONFLICT"), "SQL: {sql}");
        assert!(sql.contains("EXCLUDED.embedding"));
    }

    #[test]
    fn encode_copy_row_tab_separated() {
        let row = encode_copy_row(42, &[0.1, 0.2], r#"{"k":"v"}"#);
        let parts: Vec<&str> = row.splitn(3, '\t').collect();
        assert_eq!(parts.len(), 3);
        assert_eq!(parts[0], "42");
        assert!(parts[1].starts_with('['));
    }

    #[test]
    fn split_into_batches_groups_correctly() {
        let batches = split_into_batches(vec![1, 2, 3, 4, 5], 2);
        assert_eq!(batches.len(), 3);
        assert_eq!(batches[0], vec![1, 2]);
        assert_eq!(batches[2], vec![5]);
    }

    #[test]
    fn dot_query_sql_uses_inner_product_op() {
        let sql = dot_query_sql("docs", 5);
        assert!(sql.contains("<#>"), "SQL: {sql}");
    }

    #[test]
    fn l2_query_sql_uses_euclidean_op() {
        let sql = l2_query_sql("docs", 5);
        assert!(sql.contains("<->"), "SQL: {sql}");
    }

    #[test]
    fn metric_query_sql_dispatches_correctly() {
        let cosine = metric_query_sql("cosine", "docs", 5, 0);
        let dot = metric_query_sql("dot", "docs", 5, 0);
        let l2 = metric_query_sql("l2", "docs", 5, 0);
        assert!(cosine.contains("<=>"));
        assert!(dot.contains("<#>"));
        assert!(l2.contains("<->"));
    }

    #[test]
    fn dot_query_with_threshold_has_where_clause() {
        let sql = dot_query_with_threshold_sql("docs", 5, 0.5);
        assert!(sql.contains("WHERE"), "SQL: {sql}");
        assert!(sql.contains("<#>"), "SQL: {sql}");
    }

    #[test]
    fn filter_to_sql_eq_string() {
        let f = Filter::Eq("source".to_owned(), PayloadValue::String("wiki".to_owned()));
        let (sql, params) = filter_to_sql(&f, 0);
        assert!(sql.contains("payload->>'source'"));
        assert!(matches!(params[0], FilterParam::Text(ref s) if s == "wiki"));
    }

    #[test]
    fn hybrid_query_sql_contains_tsvector() {
        let sql = hybrid_query_sql("docs", 5, 0.7);
        assert!(sql.contains("to_tsvector"), "SQL: {sql}");
        assert!(sql.contains("plainto_tsquery"), "SQL: {sql}");
        assert!(sql.contains("ORDER BY score DESC"));
    }

    #[test]
    fn hybrid_query_column_sql_uses_custom_key() {
        let sql = hybrid_query_column_sql("docs", "body", 5, 0.5);
        assert!(sql.contains("'body'"), "SQL: {sql}");
    }

    #[test]
    fn hybrid_alpha_clamping() {
        let sql1 = hybrid_query_sql("docs", 5, 1.5); // clamped to 1.0
        let sql2 = hybrid_query_sql("docs", 5, 1.0);
        assert_eq!(sql1, sql2, "alpha should be clamped to 1.0");
    }

    #[test]
    fn validate_alpha_rejects_out_of_range() {
        assert!(validate_alpha(1.1).is_err());
        assert!(validate_alpha(-0.1).is_err());
        assert!(validate_alpha(0.5).is_ok());
    }

    #[test]
    fn threshold_validator_cosine_range() {
        assert!(ThresholdValidator::validate_cosine(0.75).is_ok());
        assert!(ThresholdValidator::validate_cosine(1.5).is_err());
    }

    #[test]
    fn threshold_validator_l2_range() {
        assert!(ThresholdValidator::validate_l2(0.5).is_ok());
        assert!(ThresholdValidator::validate_l2(-0.1).is_err());
    }

    #[test]
    fn threshold_validator_dispatch_by_metric() {
        assert!(ThresholdValidator::validate("cosine", 0.9).is_ok());
        assert!(ThresholdValidator::validate("l2", 1.5).is_err());
        assert!(ThresholdValidator::validate("dot", f32::INFINITY).is_err());
    }

    #[test]
    fn apply_threshold_filters_low_scores() {
        let results = vec![(1, 0.9f32), (2, 0.5), (3, 0.8)];
        let filtered = apply_threshold(results, 0.75);
        assert_eq!(filtered.len(), 2);
        assert!(filtered.iter().all(|(_, s)| *s >= 0.75));
    }

    #[test]
    fn classify_pg_error_connection_lost_is_retry() {
        assert_eq!(classify_pg_error("08000"), ConnectAction::Retry);
        assert_eq!(classify_pg_error("08006"), ConnectAction::Retry);
    }

    #[test]
    fn classify_pg_error_serialization_failure_is_retry_tx() {
        assert_eq!(classify_pg_error("40001"), ConnectAction::RetryTx);
        assert_eq!(classify_pg_error("40P01"), ConnectAction::RetryTx);
    }

    #[test]
    fn classify_pg_error_syntax_error_is_abort() {
        assert_eq!(classify_pg_error("42601"), ConnectAction::Abort);
    }

    #[test]
    fn retry_delay_ms_caps_at_30s() {
        let d = retry_delay_ms(100);
        assert_eq!(d, 30_000);
    }

    #[test]
    fn retry_delay_ms_is_exponential_for_small_n() {
        assert_eq!(retry_delay_ms(0), 100);
        assert_eq!(retry_delay_ms(1), 200);
        assert_eq!(retry_delay_ms(2), 400);
    }

    #[test]
    fn create_journal_table_has_idempotency_key_primary_key() {
        let sql = create_journal_table_sql("upsert_journal");
        assert!(
            sql.contains("idempotency_key TEXT PRIMARY KEY"),
            "SQL: {sql}"
        );
        assert!(sql.contains("committed_at TIMESTAMPTZ"), "SQL: {sql}");
    }

    #[test]
    fn insert_journal_sql_has_on_conflict_do_nothing() {
        let sql = insert_journal_sql("upsert_journal");
        assert!(sql.contains("ON CONFLICT"), "SQL: {sql}");
        assert!(sql.contains("DO NOTHING"), "SQL: {sql}");
    }

    #[test]
    fn check_journal_sql_selects_row_count() {
        let sql = check_journal_sql("upsert_journal");
        assert!(sql.contains("row_count"), "SQL: {sql}");
        assert!(sql.contains("idempotency_key = $1"), "SQL: {sql}");
    }

    #[test]
    fn purge_journal_sql_has_interval_param() {
        let sql = purge_journal_sql("upsert_journal");
        assert!(sql.contains("interval"), "SQL: {sql}");
        assert!(sql.contains("DELETE FROM"), "SQL: {sql}");
    }

    #[test]
    fn filter_to_sql_and_compound() {
        let f = Filter::Eq("a".to_owned(), PayloadValue::String("x".to_owned()))
            .and(Filter::Gt("n".to_owned(), PayloadValue::Integer(5)));
        let (sql, params) = filter_to_sql(&f, 0);
        assert!(sql.contains("AND"), "SQL: {sql}");
        assert_eq!(params.len(), 2);
    }

    #[test]
    fn filter_to_sql_or_compound() {
        let f = Filter::Eq("tag".to_owned(), PayloadValue::String("news".to_owned())).or(
            Filter::Eq("tag".to_owned(), PayloadValue::String("blog".to_owned())),
        );
        let (sql, params) = filter_to_sql(&f, 0);
        assert!(sql.contains("OR"), "SQL: {sql}");
        assert_eq!(params.len(), 2);
    }

    #[test]
    fn filter_to_sql_ne_integer() {
        let f = Filter::Ne("year".to_owned(), PayloadValue::Integer(2020));
        let (sql, _) = filter_to_sql(&f, 0);
        assert!(sql.contains("!="), "SQL: {sql}");
    }

    #[test]
    fn filter_to_sql_bool_value() {
        let f = Filter::Eq("active".to_owned(), PayloadValue::Bool(true));
        let (sql, params) = filter_to_sql(&f, 0);
        assert!(sql.contains("::boolean"), "SQL: {sql}");
        assert!(matches!(params[0], FilterParam::Bool(true)));
    }

    #[test]
    fn validate_filter_depth_rejects_deep_nesting() {
        let mut f = Filter::Eq("a".to_owned(), PayloadValue::Integer(1));
        for _ in 0..18 {
            f = f.and(Filter::Eq("a".to_owned(), PayloadValue::Integer(1)));
        }
        assert!(validate_filter_depth(&f).is_err());
    }

    #[test]
    fn build_where_clause_prepends_where_keyword() {
        let f = Filter::Eq("x".to_owned(), PayloadValue::String("y".to_owned()));
        let (clause, _) = build_where_clause(&f, 0);
        assert!(clause.starts_with("WHERE "), "clause: {clause}");
    }
}
