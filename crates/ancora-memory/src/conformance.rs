/// Conformance test suite for the `VectorStore` trait.
///
/// Each public function exercises one behavioral contract. Backends call these
/// functions from their own test modules to verify they satisfy the contract
/// without duplicating test logic.
#[cfg(test)]
pub mod suite {
    use crate::mem_store::MemStore;
    use crate::vector_store::*;

    fn fresh(name: &str, dims: usize) -> MemStore {
        let s = MemStore::new();
        s.create_collection(CollectionSpec::new(name, dims, Distance::Cosine))
            .unwrap();
        s
    }

    fn pt(id: u64, v: Vec<f32>) -> Point {
        Point::new(id, v)
    }

    // ---- upsert + query --------------------------------------------------

    pub fn conformance_upsert_then_query_returns_nearest<S: VectorStore>(store: &S) {
        store
            .upsert(
                "col",
                vec![
                    pt(1, vec![1.0, 0.0, 0.0]),
                    pt(2, vec![0.0, 1.0, 0.0]),
                    pt(3, vec![0.0, 0.0, 1.0]),
                ],
            )
            .unwrap();
        let r = store
            .query("col", QueryRequest::new(vec![1.0, 0.0, 0.0], 1))
            .unwrap();
        assert_eq!(r.len(), 1);
        assert_eq!(r[0].id, PointId::Num(1));
    }

    #[test]
    fn mem_upsert_query() {
        conformance_upsert_then_query_returns_nearest(&fresh("col", 3));
    }

    // ---- metadata filter -------------------------------------------------

    pub fn conformance_metadata_filter_narrows_results<S: VectorStore>(store: &S) {
        store
            .upsert(
                "col",
                vec![
                    Point::new(1u64, vec![1.0, 0.0]).with_payload("tag", "a"),
                    Point::new(2u64, vec![0.9, 0.1]).with_payload("tag", "b"),
                    Point::new(3u64, vec![0.8, 0.2]).with_payload("tag", "a"),
                ],
            )
            .unwrap();
        let req = QueryRequest::new(vec![1.0, 0.0], 10).with_filter(Filter::Eq(
            "tag".to_owned(),
            PayloadValue::String("a".to_owned()),
        ));
        let r = store.query("col", req).unwrap();
        assert!(r
            .iter()
            .all(|p| p.payload.get("tag") == Some(&PayloadValue::String("a".to_owned()))));
        assert_eq!(r.len(), 2);
    }

    #[test]
    fn mem_metadata_filter() {
        conformance_metadata_filter_narrows_results(&fresh("col", 2));
    }

    // ---- delete ----------------------------------------------------------

    pub fn conformance_delete_removes_a_point<S: VectorStore>(store: &S) {
        store
            .upsert("col", vec![pt(1, vec![1.0, 0.0]), pt(2, vec![0.0, 1.0])])
            .unwrap();
        store.delete("col", vec![PointId::Num(1)]).unwrap();
        let r = store
            .query("col", QueryRequest::new(vec![1.0, 0.0], 10))
            .unwrap();
        assert!(!r.iter().any(|p| p.id == PointId::Num(1)));
        assert_eq!(r.len(), 1);
    }

    #[test]
    fn mem_delete() {
        conformance_delete_removes_a_point(&fresh("col", 2));
    }

    // ---- batch upsert ----------------------------------------------------

    pub fn conformance_batch_upsert_ordering<S: VectorStore>(store: &S) {
        let batch1 = vec![pt(1, vec![1.0, 0.0, 0.0]), pt(2, vec![0.0, 1.0, 0.0])];
        let batch2 = vec![pt(3, vec![0.0, 0.0, 1.0])];
        store.batch_upsert("col", vec![batch1, batch2]).unwrap();
        let info = store.describe_collection("col").unwrap();
        assert_eq!(info.point_count, 3);
    }

    #[test]
    fn mem_batch_upsert() {
        conformance_batch_upsert_ordering(&fresh("col", 3));
    }

    // ---- distance metrics ------------------------------------------------

    pub fn conformance_distance_metrics_behave<S: VectorStore>(store: &S) {
        store
            .upsert("col", vec![pt(1, vec![1.0, 0.0]), pt(2, vec![0.0, 1.0])])
            .unwrap();
        let r = store
            .query("col", QueryRequest::new(vec![1.0, 0.0], 2))
            .unwrap();
        assert_eq!(r[0].id, PointId::Num(1), "most similar should be id=1");
        assert!(r[0].score > r[1].score, "score ordering must be descending");
    }

    #[test]
    fn mem_distance_metrics() {
        conformance_distance_metrics_behave(&fresh("col", 2));
    }

    // ---- pagination ------------------------------------------------------

    pub fn conformance_pagination_stable<S: VectorStore>(store: &S) {
        store
            .upsert(
                "col",
                vec![
                    pt(1, vec![1.0, 0.0, 0.0]),
                    pt(2, vec![0.9, 0.1, 0.0]),
                    pt(3, vec![0.8, 0.2, 0.0]),
                    pt(4, vec![0.7, 0.3, 0.0]),
                ],
            )
            .unwrap();
        let page0 = store
            .query(
                "col",
                QueryRequest::new(vec![1.0, 0.0, 0.0], 2).with_offset(0),
            )
            .unwrap();
        let page1 = store
            .query(
                "col",
                QueryRequest::new(vec![1.0, 0.0, 0.0], 2).with_offset(2),
            )
            .unwrap();
        assert_eq!(page0.len(), 2);
        assert_eq!(page1.len(), 2);
        let ids0: Vec<_> = page0.iter().map(|p| p.id.clone()).collect();
        let ids1: Vec<_> = page1.iter().map(|p| p.id.clone()).collect();
        assert!(
            ids0.iter().all(|id| !ids1.contains(id)),
            "pages must not overlap"
        );
    }

    #[test]
    fn mem_pagination() {
        conformance_pagination_stable(&fresh("col", 3));
    }

    // ---- hybrid search ---------------------------------------------------

    pub fn conformance_hybrid_search_merges_dense_and_keyword<S: VectorStore>(store: &S) {
        store
            .upsert(
                "col",
                vec![
                    Point::new(1u64, vec![1.0, 0.0]).with_payload("text", "apple fruit"),
                    Point::new(2u64, vec![0.5, 0.5]).with_payload("text", "banana fruit"),
                    Point::new(3u64, vec![0.0, 1.0]).with_payload("text", "carrot vegetable"),
                ],
            )
            .unwrap();
        let req = HybridQueryRequest::new(vec![1.0, 0.0], "apple", 3).with_alpha(0.5);
        let r = store.hybrid_query("col", req).unwrap();
        assert!(!r.is_empty(), "hybrid query must return results");
        assert_eq!(
            r[0].id,
            PointId::Num(1),
            "apple+dense should rank id=1 first"
        );
    }

    #[test]
    fn mem_hybrid_search() {
        conformance_hybrid_search_merges_dense_and_keyword(&fresh("col", 2));
    }

    // ---- score threshold -------------------------------------------------

    pub fn conformance_score_threshold_filters<S: VectorStore>(store: &S) {
        store
            .upsert(
                "col",
                vec![pt(1, vec![1.0, 0.0, 0.0]), pt(2, vec![0.0, 1.0, 0.0])],
            )
            .unwrap();
        let req = QueryRequest::new(vec![1.0, 0.0, 0.0], 10).with_score_threshold(0.9);
        let r = store.query("col", req).unwrap();
        assert!(
            r.iter().all(|p| p.score >= 0.9),
            "all results must be above threshold"
        );
    }

    #[test]
    fn mem_score_threshold() {
        conformance_score_threshold_filters(&fresh("col", 3));
    }
}
