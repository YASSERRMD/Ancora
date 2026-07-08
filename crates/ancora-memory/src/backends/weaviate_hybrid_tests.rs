//! Hybrid search query tests for the Weaviate GraphQL layer.
//!
//! All offline -- no live server required.

#![cfg(test)]

use crate::backends::weaviate::*;

#[test]
fn hybrid_alpha_zero_is_pure_keyword() {
    let body = graphql_hybrid_query("Document", "rust lang", None, 0.0, 5, &["title"]);
    let q = body["query"].as_str().unwrap();
    assert!(q.contains("alpha: 0"), "query: {q}");
}

#[test]
fn hybrid_alpha_one_is_pure_vector() {
    let body = graphql_hybrid_query("Document", "rust lang", None, 1.0, 5, &["title"]);
    let q = body["query"].as_str().unwrap();
    assert!(q.contains("alpha: 1"), "query: {q}");
}

#[test]
fn hybrid_alpha_clamped_to_0_1() {
    let body = graphql_hybrid_query("Document", "test", None, 1.5, 5, &["title"]);
    let q = body["query"].as_str().unwrap();
    // alpha 1.5 clamped to 1.0
    assert!(q.contains("alpha: 1"), "query: {q}");
    assert!(
        !q.contains("1.5"),
        "should not contain unclamped value, query: {q}"
    );
}

#[test]
fn hybrid_with_explicit_vector_includes_vector_in_query() {
    let body = graphql_hybrid_query("Document", "test", Some(&[0.1f32, 0.2]), 0.5, 5, &["title"]);
    let q = body["query"].as_str().unwrap();
    assert!(q.contains("vector:"), "query: {q}");
    assert!(q.contains("0.1"), "query: {q}");
}

#[test]
fn hybrid_without_explicit_vector_no_vector_field() {
    let body = graphql_hybrid_query("Document", "test", None, 0.5, 5, &["title"]);
    let q = body["query"].as_str().unwrap();
    assert!(
        !q.contains("vector:"),
        "should have no vector field, query: {q}"
    );
}

#[test]
fn hybrid_query_limit_in_graphql() {
    let body = graphql_hybrid_query("Document", "test", None, 0.5, 42, &["title"]);
    let q = body["query"].as_str().unwrap();
    assert!(q.contains("limit: 42"), "query: {q}");
}

#[test]
fn bm25_query_has_correct_class_and_text() {
    let body = graphql_bm25_query("Article", "quantum computing", 10, &["body"]);
    let q = body["query"].as_str().unwrap();
    assert!(q.contains("Article"), "query: {q}");
    assert!(q.contains("quantum computing"), "query: {q}");
}

#[test]
fn near_vector_with_certainty_contains_threshold() {
    let body = graphql_near_vector_with_certainty_query("Document", &[0.1f32], 5, 0.8, &["title"]);
    let q = body["query"].as_str().unwrap();
    assert!(q.contains("certainty: 0.8"), "query: {q}");
}
