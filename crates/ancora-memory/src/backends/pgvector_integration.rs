//! Integration tests for the live `PgVectorStore`.
//!
//! These require an actual Postgres server with the `vector` extension
//! available. Set `TEST_DATABASE_URL` before running. Every test that uses
//! the shared `conformance::suite` functions operates on a collection
//! hardcoded to `"col"` inside those functions (matching the in-memory
//! `MemStore` conformance tests), so run with `--test-threads=1` to avoid
//! concurrent tests racing on the same table:
//!
//!   TEST_DATABASE_URL=postgres://localhost/ancora_test \
//!     cargo test -p ancora-memory --features pgvector -- --ignored --test-threads=1 pgvector
//!
//! All tests are `#[ignore]` so CI stays green without a Postgres service.
#![cfg(all(test, feature = "pgvector"))]

use crate::backends::pgvector_store::PgVectorStore;
use crate::conformance::suite;
use crate::vector_store::{CollectionSpec, Distance, VectorStore};

fn test_url() -> Option<String> {
    std::env::var("TEST_DATABASE_URL").ok()
}

/// Connect and (re)create the `"col"` collection the shared conformance
/// suite functions operate on.
fn fresh_suite_store(dims: usize) -> Option<PgVectorStore> {
    let url = test_url()?;
    let store = PgVectorStore::connect(&url).expect("connect to postgres");
    let _ = store.drop_collection("col");
    store
        .create_collection(CollectionSpec::new("col", dims, Distance::Cosine))
        .expect("create_collection");
    Some(store)
}

/// Connect and (re)create a custom-named collection for tests that don't
/// use the shared conformance suite.
fn fresh_store(collection: &str, dims: usize) -> Option<PgVectorStore> {
    let url = test_url()?;
    let store = PgVectorStore::connect(&url).expect("connect to postgres");
    let _ = store.drop_collection(collection);
    store
        .create_collection(CollectionSpec::new(collection, dims, Distance::Cosine))
        .expect("create_collection");
    Some(store)
}

#[test]
#[ignore]
fn integration_upsert_then_query_returns_nearest() {
    let Some(store) = fresh_suite_store(3) else {
        return;
    };
    suite::conformance_upsert_then_query_returns_nearest(&store);
}

#[test]
#[ignore]
fn integration_metadata_filter_narrows_results() {
    let Some(store) = fresh_suite_store(2) else {
        return;
    };
    suite::conformance_metadata_filter_narrows_results(&store);
}

#[test]
#[ignore]
fn integration_delete_removes_a_point() {
    let Some(store) = fresh_suite_store(2) else {
        return;
    };
    suite::conformance_delete_removes_a_point(&store);
}

#[test]
#[ignore]
fn integration_batch_upsert_ordering() {
    let Some(store) = fresh_suite_store(3) else {
        return;
    };
    suite::conformance_batch_upsert_ordering(&store);
}

#[test]
#[ignore]
fn integration_pagination_stable() {
    let Some(store) = fresh_suite_store(3) else {
        return;
    };
    suite::conformance_pagination_stable(&store);
}

#[test]
#[ignore]
fn integration_score_threshold_filters() {
    let Some(store) = fresh_suite_store(3) else {
        return;
    };
    suite::conformance_score_threshold_filters(&store);
}

#[test]
#[ignore]
fn integration_hybrid_search_merges_dense_and_keyword() {
    let Some(store) = fresh_suite_store(2) else {
        return;
    };
    suite::conformance_hybrid_search_merges_dense_and_keyword(&store);
}

#[test]
#[ignore]
fn integration_describe_collection_reports_point_count() {
    let Some(store) = fresh_store("it_describe", 3) else {
        return;
    };
    use crate::vector_store::Point;
    store
        .upsert(
            "it_describe",
            vec![
                Point::new(1u64, vec![1.0, 0.0, 0.0]),
                Point::new(2u64, vec![0.0, 1.0, 0.0]),
            ],
        )
        .unwrap();
    let info = store.describe_collection("it_describe").unwrap();
    assert_eq!(info.point_count, 2);
    assert_eq!(info.dimensions, 3);
}

#[test]
#[ignore]
fn integration_delete_by_filter_removes_matching_points() {
    use crate::vector_store::{Filter, Payload, PayloadValue, Point};
    let Some(store) = fresh_store("it_delete_by_filter", 2) else {
        return;
    };
    let mut tagged = Payload::new();
    tagged.insert("tag".to_owned(), PayloadValue::String("a".to_owned()));
    store
        .upsert(
            "it_delete_by_filter",
            vec![
                Point::new(1u64, vec![1.0, 0.0]).with_payload("tag", "a"),
                Point::new(2u64, vec![0.0, 1.0]).with_payload("tag", "b"),
            ],
        )
        .unwrap();
    let deleted = store
        .delete_by_filter(
            "it_delete_by_filter",
            Filter::Eq("tag".to_owned(), PayloadValue::String("a".to_owned())),
        )
        .unwrap();
    assert_eq!(deleted, 1);
    let info = store.describe_collection("it_delete_by_filter").unwrap();
    assert_eq!(info.point_count, 1);
}

#[test]
#[ignore]
fn integration_upsert_rejects_uuid_point_id() {
    let Some(store) = fresh_store("it_uuid_reject", 2) else {
        return;
    };
    use crate::vector_store::Point;
    let err = store
        .upsert(
            "it_uuid_reject",
            vec![Point::new("not-a-number", vec![1.0, 0.0])],
        )
        .unwrap_err();
    assert!(matches!(
        err,
        crate::vector_store::VectorStoreError::InvalidFilter(_)
    ));
}
