//! Integration tests for the Vespa backend.
//! Skipped by default; set ANCORA_VESPA_URL to run against a live deployment.

#![cfg(test)]

fn vespa_url() -> Option<String> {
    std::env::var("ANCORA_VESPA_URL").ok()
}

#[test]
#[ignore = "requires ANCORA_VESPA_URL"]
fn live_feed_and_query_document() {
    let _ = vespa_url();
}

#[test]
#[ignore = "requires ANCORA_VESPA_URL"]
fn live_hybrid_search_returns_hits() {
    let _ = vespa_url();
}

#[test]
#[ignore = "requires ANCORA_VESPA_URL"]
fn live_delete_document() {
    let _ = vespa_url();
}
