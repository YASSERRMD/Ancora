//! Hybrid vector + full-text search tests for the LanceDB backend.
//! All offline.

#![cfg(test)]

use crate::backends::lancedb::*;

#[test]
fn hybrid_query_includes_fts_query_text() {
    let q = HybridQuery::new("docs", vec![0.1f32], "machine learning", 5);
    let j = q.to_json();
    assert_eq!(j["fts_query"], "machine learning");
}

#[test]
fn hybrid_query_includes_dense_vector() {
    let q = HybridQuery::new("docs", vec![0.1f32, 0.2f32], "query", 5);
    assert!(q.to_json()["vector"].is_array());
}

#[test]
fn hybrid_query_default_reranker_is_rrf() {
    let q = HybridQuery::new("docs", vec![0.1f32], "q", 5);
    assert_eq!(q.to_json()["reranker"], "rrf");
}

#[test]
fn hybrid_query_custom_reranker_propagates() {
    let q = HybridQuery::new("docs", vec![0.1f32], "q", 5).reranker("linear");
    assert_eq!(q.to_json()["reranker"], "linear");
}

#[test]
fn hybrid_query_limit_propagates() {
    let q = HybridQuery::new("docs", vec![0.1f32], "q", 42);
    assert_eq!(q.to_json()["limit"], 42);
}

#[test]
fn hybrid_query_filter_is_omitted_when_not_set() {
    let q = HybridQuery::new("docs", vec![0.1f32], "q", 5);
    assert!(q.to_json()["filter"].is_null());
}

#[test]
fn hybrid_query_filter_is_set_when_provided() {
    let q = HybridQuery::new("docs", vec![0.1f32], "q", 5).filter("year > 2021");
    assert_eq!(q.to_json()["filter"], "year > 2021");
}

#[test]
fn full_text_index_sets_column_name() {
    let idx = FullTextIndex::new("body_text");
    assert_eq!(idx.column, "body_text");
}

#[test]
fn full_text_index_without_position_is_false_by_default() {
    let idx = FullTextIndex::new("body");
    assert!(!idx.with_position);
}

#[test]
fn full_text_index_with_position_sets_flag() {
    let idx = FullTextIndex::new("body").with_position();
    assert_eq!(idx.to_json()["with_position"], true);
}
