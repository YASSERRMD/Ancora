/// Schema migration helpers for pgvector collections.
///
/// Generates SQL for adding columns, resizing vector dimensions, and
/// managing schema version tracking -- all offline verifiable.

use crate::backends::pgvector::{sanitize_identifier, create_table_sql};

/// A schema version record stored in the `_ancora_schema_version` table.
pub struct SchemaVersion {
    pub table_name: String,
    pub version: u32,
}

/// Generate DDL for the schema-version tracking table.
pub fn create_schema_version_table_sql() -> String {
    "CREATE TABLE IF NOT EXISTS _ancora_schema_version (\
     table_name TEXT PRIMARY KEY, \
     version INT NOT NULL DEFAULT 0, \
     applied_at TIMESTAMPTZ NOT NULL DEFAULT NOW()\
     );".to_owned()
}

/// Generate SQL to read the current version for a table.
pub fn read_version_sql() -> String {
    "SELECT version FROM _ancora_schema_version WHERE table_name = $1;".to_owned()
}

/// Generate SQL to insert or bump the version.
pub fn upsert_version_sql() -> String {
    "INSERT INTO _ancora_schema_version (table_name, version) VALUES ($1, $2) \
     ON CONFLICT (table_name) DO UPDATE SET version = EXCLUDED.version, applied_at = NOW();".to_owned()
}

/// Generate SQL to add a JSONB metadata column to an existing table.
///
/// Safe to call if the column already exists because of the `IF NOT EXISTS` guard.
pub fn add_payload_column_sql(table: &str) -> Result<String, String> {
    sanitize_identifier(table)?;
    Ok(format!(
        "ALTER TABLE {table} ADD COLUMN IF NOT EXISTS payload JSONB NOT NULL DEFAULT '{{}}'::jsonb;"
    ))
}

/// Generate SQL to recreate the embedding column with a new dimension.
///
/// This is a destructive DDL operation: it drops and re-adds the column.
/// The caller must take a backup or run in a transaction with a rollback plan.
pub fn resize_embedding_column_sql(table: &str, new_dims: usize) -> Result<String, String> {
    sanitize_identifier(table)?;
    Ok(format!(
        "ALTER TABLE {table} DROP COLUMN IF EXISTS embedding; \
         ALTER TABLE {table} ADD COLUMN embedding vector({new_dims});"
    ))
}

/// Generate a full migration script: create table + HNSW index + journal table.
pub fn full_setup_sql(collection: &str, dimensions: usize, m: u16, ef_construct: u16) -> Result<String, String> {
    sanitize_identifier(collection)?;
    let table = create_table_sql(collection, dimensions);
    let idx = crate::backends::pgvector::create_hnsw_index_sql(collection, m, ef_construct);
    let version = create_schema_version_table_sql();
    Ok(format!("{table}\n{idx}\n{version}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_schema_version_table_has_primary_key() {
        let sql = create_schema_version_table_sql();
        assert!(sql.contains("table_name TEXT PRIMARY KEY"), "SQL: {sql}");
        assert!(sql.contains("applied_at TIMESTAMPTZ"), "SQL: {sql}");
    }

    #[test]
    fn upsert_version_sql_has_on_conflict() {
        let sql = upsert_version_sql();
        assert!(sql.contains("ON CONFLICT"), "SQL: {sql}");
        assert!(sql.contains("DO UPDATE"), "SQL: {sql}");
    }

    #[test]
    fn add_payload_column_sql_has_if_not_exists() {
        let sql = add_payload_column_sql("docs").unwrap();
        assert!(sql.contains("IF NOT EXISTS"), "SQL: {sql}");
        assert!(sql.contains("JSONB"), "SQL: {sql}");
    }

    #[test]
    fn add_payload_column_rejects_unsafe_identifier() {
        assert!(add_payload_column_sql("docs; DROP TABLE users").is_err());
    }

    #[test]
    fn resize_embedding_drops_then_adds() {
        let sql = resize_embedding_column_sql("docs", 1024).unwrap();
        assert!(sql.contains("DROP COLUMN"), "SQL: {sql}");
        assert!(sql.contains("vector(1024)"), "SQL: {sql}");
    }

    #[test]
    fn full_setup_sql_contains_all_parts() {
        let sql = full_setup_sql("agents", 512, 16, 100).unwrap();
        assert!(sql.contains("CREATE TABLE"), "SQL: {sql}");
        assert!(sql.contains("hnsw"), "SQL: {sql}");
        assert!(sql.contains("_ancora_schema_version"), "SQL: {sql}");
    }

    #[test]
    fn full_setup_rejects_unsafe_collection_name() {
        assert!(full_setup_sql("bad-name", 512, 16, 100).is_err());
    }
}
