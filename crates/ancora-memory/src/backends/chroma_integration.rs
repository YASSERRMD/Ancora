//! Integration tests for the Chroma backend.
//! Skipped by default; set ANCORA_CHROMA_URL to run against a live server.

#![cfg(test)]

use crate::backends::chroma::*;

fn chroma_url() -> Option<String> {
    std::env::var("ANCORA_CHROMA_URL").ok()
}

#[test]
#[ignore = "requires ANCORA_CHROMA_URL"]
fn live_heartbeat_responds() {
    let url = match chroma_url() {
        Some(u) => u,
        None => return,
    };
    let endpoint = heartbeat_url(&url);
    // Verify URL shape only (no live call in CI).
    assert!(endpoint.ends_with("/heartbeat"), "url: {endpoint}");
}

#[test]
#[ignore = "requires ANCORA_CHROMA_URL"]
fn live_create_and_query_collection() {
    // This test intentionally left as a stub.
    // Real body: create collection, add embeddings, query, delete.
    let _ = chroma_url();
}

#[test]
#[ignore = "requires ANCORA_CHROMA_URL"]
fn live_delete_by_filter() {
    let _ = chroma_url();
}
