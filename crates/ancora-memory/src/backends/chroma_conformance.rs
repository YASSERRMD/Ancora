//! Conformance tests for the Chroma backend.
//! All offline.

#![cfg(test)]

use crate::backends::chroma::*;

#[test]
fn create_collection_body_has_name_field() {
    let body = create_collection_body("my_collection", None);
    assert_eq!(body["name"], "my_collection");
}

#[test]
fn add_body_ids_count_matches_embeddings() {
    let body = add_body(
        &["a", "b"],
        &[vec![0.1f32], vec![0.2f32]],
        &[serde_json::json!({}), serde_json::json!({})],
        None,
    );
    assert_eq!(body["ids"].as_array().unwrap().len(), 2);
    assert_eq!(body["embeddings"].as_array().unwrap().len(), 2);
}

#[test]
fn query_body_has_n_results() {
    let body = query_body(&[vec![0.1f32]], 7, None, &["distances"]);
    assert_eq!(body["n_results"], 7);
}

#[test]
fn delete_body_with_filter_sets_where() {
    let f = where_eq("tag", serde_json::json!("ml"));
    let body = delete_body(&[], Some(f));
    assert!(body["where"].is_object());
}

#[test]
fn get_body_limit_propagates() {
    let body = get_body(None, None, Some(20), &["ids"]);
    assert_eq!(body["limit"], 20);
}

#[test]
fn where_gt_filter_structure() {
    let f = where_gt("score", serde_json::json!(5));
    assert!(f["score"]["$gt"] == 5);
}

#[test]
fn where_or_produces_or_key() {
    let f = where_or(
        where_eq("a", serde_json::json!(1)),
        where_eq("b", serde_json::json!(2)),
    );
    assert!(f["$or"].is_array());
}

#[test]
fn parse_query_distances_extracts_values() {
    let body = serde_json::json!({ "distances": [[0.1, 0.2]] });
    let dists = parse_query_distances(&body);
    assert_eq!(dists[0].len(), 2);
}

#[test]
fn heartbeat_url_ends_with_heartbeat() {
    let url = heartbeat_url("http://localhost:8000");
    assert!(url.ends_with("/heartbeat"), "url: {url}");
}

#[test]
fn chroma_error_500_is_transient() {
    let err = ChromaError::from_response(500, "err");
    assert!(err.is_transient(), "InternalError (500) must be transient");
}
