//! Hybrid dense+sparse search tests for the Milvus backend.
//! All offline.

#![cfg(test)]

use crate::backends::milvus::*;

#[test]
fn hybrid_search_body_has_two_search_sub_requests() {
    let body = hybrid_search_body(
        "docs",
        &[0.1f32; 4],
        "sparse",
        &[(0, 0.9f32), (5, 0.3f32)],
        10,
        metric_type::COSINE,
    );
    let searches = body["search"].as_array().unwrap();
    assert_eq!(searches.len(), 2, "must fuse two ANN requests");
}

#[test]
fn hybrid_search_body_uses_rrf_reranker() {
    let body = hybrid_search_body("docs", &[0.1f32; 4], "sparse", &[], 5, metric_type::L2);
    assert_eq!(body["rerank"]["strategy"], "rrf");
}

#[test]
fn hybrid_search_body_respects_top_k() {
    let body = hybrid_search_body("docs", &[0.1f32; 4], "sparse", &[], 42, metric_type::COSINE);
    assert_eq!(body["limit"], 42);
}

#[test]
fn hybrid_search_dense_sub_request_uses_embedding_field() {
    let body = hybrid_search_body(
        "docs",
        &[0.1f32; 4],
        "sparse",
        &[(0, 0.9f32)],
        5,
        metric_type::COSINE,
    );
    let dense = &body["search"][0];
    assert_eq!(dense["annsField"], "embedding");
}

#[test]
fn hybrid_search_sparse_sub_request_uses_named_field() {
    let body = hybrid_search_body(
        "docs",
        &[0.1f32; 4],
        "my_sparse",
        &[(0, 0.9f32)],
        5,
        metric_type::COSINE,
    );
    let sparse = &body["search"][1];
    assert_eq!(sparse["annsField"], "my_sparse");
}

#[test]
fn hybrid_search_sparse_indices_and_values_are_embedded() {
    let sparse = vec![(3u32, 0.7f32), (99u32, 0.2f32)];
    let body = hybrid_search_body("docs", &[0.1f32; 4], "sp", &sparse, 5, metric_type::COSINE);
    let sparse_data = &body["search"][1]["data"][0];
    let indices = sparse_data["indices"].as_array().unwrap();
    assert!(indices.iter().any(|v| v == 3), "indices: {indices:?}");
}

#[test]
fn hybrid_search_rrf_k_defaults_to_60() {
    let body = hybrid_search_body("docs", &[0.1f32; 4], "sp", &[], 5, metric_type::COSINE);
    assert_eq!(body["rerank"]["params"]["k"], 60);
}

#[test]
fn hybrid_search_outputs_payload_field() {
    let body = hybrid_search_body("docs", &[0.1f32; 4], "sp", &[], 5, metric_type::COSINE);
    let fields = body["outputFields"].as_array().unwrap();
    assert!(
        fields.iter().any(|f| f == "payload"),
        "outputFields: {fields:?}"
    );
}
