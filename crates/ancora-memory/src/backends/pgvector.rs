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

/// Generate an HNSW index creation statement for cosine similarity.
pub fn create_hnsw_index_sql(table: &str, m: u16, ef_construct: u16) -> String {
    format!(
        "CREATE INDEX IF NOT EXISTS {table}_embedding_idx \
         ON {table} USING hnsw (embedding vector_cosine_ops) \
         WITH (m = {m}, ef_construction = {ef_construct});"
    )
}

/// Generate an IVF-Flat index creation statement.
pub fn create_ivfflat_index_sql(table: &str, lists: u32) -> String {
    format!(
        "CREATE INDEX IF NOT EXISTS {table}_embedding_ivf_idx \
         ON {table} USING ivfflat (embedding vector_cosine_ops) \
         WITH (lists = {lists});"
    )
}

/// Generate a `DROP TABLE` statement.
pub fn drop_table_sql(table: &str) -> String {
    format!("DROP TABLE IF EXISTS {table};")
}

/// Generate a `SELECT COUNT(*) FROM table` for describe_collection.
pub fn count_sql(table: &str) -> String {
    format!("SELECT COUNT(*) FROM {table};")
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
    fn drop_table_sql_format() {
        assert_eq!(drop_table_sql("docs"), "DROP TABLE IF EXISTS docs;");
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
