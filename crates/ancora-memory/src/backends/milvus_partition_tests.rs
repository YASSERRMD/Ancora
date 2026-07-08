//! Partition routing tests for the Milvus backend.
//! All offline.

#![cfg(test)]

use crate::backends::milvus::*;

#[test]
fn create_partition_body_sets_partition_name() {
    let body = create_partition_body("docs", "region_us");
    assert_eq!(body["partitionName"], "region_us");
}

#[test]
fn drop_partition_body_targets_correct_partition() {
    let body = drop_partition_body("docs", "region_us");
    assert_eq!(body["collectionName"], "docs");
    assert_eq!(body["partitionName"], "region_us");
}

#[test]
fn load_partition_body_includes_all_names() {
    let body = load_partition_body("docs", &["p0", "p1", "p2"]);
    let names = body["partitionNames"].as_array().unwrap();
    assert_eq!(names.len(), 3);
}

#[test]
fn release_partition_body_structure_matches_load() {
    let load = load_partition_body("docs", &["p0"]);
    let release = release_partition_body("docs", &["p0"]);
    assert_eq!(load["collectionName"], release["collectionName"]);
}

#[test]
fn insert_into_partition_sets_partition_name() {
    let body = insert_into_partition_body(
        "docs",
        "region_eu",
        &[(vec![0.1f32; 4], serde_json::json!({"tag": "x"}))],
    );
    assert_eq!(body["partitionName"], "region_eu");
}

#[test]
fn search_partition_body_restricts_to_partition() {
    let body = search_partition_body("docs", "region_eu", &[0.1f32; 4], 5, metric_type::COSINE);
    let parts = body["partitionNames"].as_array().unwrap();
    assert!(parts.iter().any(|p| p == "region_eu"));
}

#[test]
fn query_partition_body_limits_results() {
    let body = query_partition_body("docs", "region_eu", "score > 0", 100);
    assert_eq!(body["limit"], 100);
    assert_eq!(body["partitionNames"][0], "region_eu");
}

#[test]
fn parse_partition_names_handles_empty_response() {
    let body = serde_json::json!({ "data": [] });
    let names = parse_partition_names(&body);
    assert!(names.is_empty());
}

#[test]
fn parse_partition_names_handles_multiple() {
    let body = serde_json::json!({
        "data": [
            { "partitionName": "alpha" },
            { "partitionName": "beta" },
        ]
    });
    let names = parse_partition_names(&body);
    assert_eq!(names, vec!["alpha", "beta"]);
}
