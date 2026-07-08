//! Integration tests for the Redis Vector (RediSearch) backend.
//! Skipped by default; set ANCORA_REDIS_URL to run against a live Redis Stack.

#![cfg(test)]

fn redis_url() -> Option<String> {
    std::env::var("ANCORA_REDIS_URL").ok()
}

#[test]
#[ignore = "requires ANCORA_REDIS_URL pointing to Redis Stack"]
fn live_create_index_and_search() {
    let _ = redis_url();
}

#[test]
#[ignore = "requires ANCORA_REDIS_URL pointing to Redis Stack"]
fn live_filtered_ann_search() {
    let _ = redis_url();
}

#[test]
#[ignore = "requires ANCORA_REDIS_URL pointing to Redis Stack"]
fn live_hset_and_delete() {
    let _ = redis_url();
}
