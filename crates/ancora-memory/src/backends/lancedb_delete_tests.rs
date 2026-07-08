//! Delete-by-predicate tests for the LanceDB backend.
//! All offline.

#![cfg(test)]

use crate::backends::lancedb::*;

#[test]
fn delete_predicate_sets_table() {
    let j = delete_predicate("docs", "score < 0.5");
    assert_eq!(j["table"], "docs");
}

#[test]
fn delete_predicate_sets_sql() {
    let j = delete_predicate("docs", "year < 2020");
    assert_eq!(j["predicate"], "year < 2020");
}

#[test]
fn delete_predicate_with_eq_str_filter() {
    let sql = sql_eq_str("status", "deleted");
    let j = delete_predicate("docs", &sql);
    assert!(j["predicate"].as_str().unwrap().contains("deleted"));
}

#[test]
fn delete_predicate_with_compound_filter() {
    let expr = sql_or(&sql_lt("score", 0), &sql_eq_str("status", "spam"));
    let j = delete_predicate("docs", &expr);
    assert!(
        j["predicate"].as_str().unwrap().contains("OR"),
        "expr: {}",
        j["predicate"]
    );
}

#[test]
fn delete_predicate_is_json_object() {
    let j = delete_predicate("docs", "x > 0");
    assert!(j.is_object());
}

#[test]
fn cleanup_removes_versions_older_than_zero() {
    let j = cleanup_old_versions_descriptor("docs", 0);
    assert_eq!(j["older_than_days"], 0);
}

#[test]
fn merge_insert_empty_batch_is_valid() {
    let j = merge_insert_descriptor("docs", "id", vec![]);
    assert!(j["data"].is_array());
    assert_eq!(j["data"].as_array().unwrap().len(), 0);
}

#[test]
fn schema_evolution_round_trip_preserves_names() {
    let col = ColumnDef::new("embedding_v2", column_type::FLOAT32);
    let add = add_column_descriptor("docs", &col, None);
    let drop = drop_column_descriptor("docs", "embedding");
    assert_eq!(add["column"], "embedding_v2");
    assert_eq!(drop["column"], "embedding");
}
