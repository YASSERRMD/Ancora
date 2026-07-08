//! ANN index configuration tests for the LanceDB backend.
//! All offline.

#![cfg(test)]

use crate::backends::lancedb::*;

#[test]
fn ann_index_json_has_ivf_pq_type() {
    let idx = AnnIndex::new(256, 16);
    assert_eq!(idx.to_json()["index_type"], "IVF_PQ");
}

#[test]
fn ann_index_num_sub_vectors_propagates() {
    let idx = AnnIndex::new(128, 32);
    assert_eq!(idx.to_json()["num_sub_vectors"], 32);
}

#[test]
fn ann_index_num_partitions_propagates() {
    let idx = AnnIndex::new(512, 16);
    assert_eq!(idx.to_json()["num_partitions"], 512);
}

#[test]
fn ann_index_default_metric_is_cosine() {
    let idx = AnnIndex::new(256, 16);
    assert_eq!(idx.to_json()["metric"], "cosine");
}

#[test]
fn ann_index_metric_override_propagates() {
    let idx = AnnIndex::new(256, 16).metric("l2");
    assert_eq!(idx.to_json()["metric"], "l2");
}

#[test]
fn ann_index_dot_metric() {
    let idx = AnnIndex::new(256, 8).metric("dot");
    assert_eq!(idx.to_json()["metric"], "dot");
}

#[test]
fn fts_index_column_name_preserved() {
    let idx = FullTextIndex::new("abstract");
    assert_eq!(idx.column, "abstract");
}

#[test]
fn fts_index_json_has_fts_type() {
    let idx = FullTextIndex::new("body");
    assert_eq!(idx.to_json()["index_type"], "FTS");
}
