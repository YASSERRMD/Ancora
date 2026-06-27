use std::collections::HashMap;
use std::sync::Mutex;

use crate::vector_store::{
    apply_score_threshold, filter_matches, keyword_score_naive, hybrid_score,
    CollectionInfo, CollectionSpec, Distance, Filter, HybridQueryRequest, Page,
    Payload, Point, PointId, QueryRequest, ScoredPoint, VectorStore, VectorStoreError,
};

struct Collection {
    spec: CollectionSpec,
    points: HashMap<PointId, (Vec<f32>, Payload)>,
    /// Stores the raw text payload value for keyword search (keyed by PointId).
    texts: HashMap<PointId, String>,
}

/// In-memory reference implementation of `VectorStore`.
///
/// Fully correct, zero-dependency, and used by the conformance test suite.
/// Not suitable for production (no persistence, no real HNSW index), but
/// useful for tests and as a template for real backends.
pub struct MemStore {
    collections: Mutex<HashMap<String, Collection>>,
}

impl MemStore {
    pub fn new() -> Self {
        Self { collections: Mutex::new(HashMap::new()) }
    }
}

impl Default for MemStore {
    fn default() -> Self { Self::new() }
}

impl VectorStore for MemStore {
    fn create_collection(&self, spec: CollectionSpec) -> Result<(), VectorStoreError> {
        let mut guard = self.collections.lock().unwrap();
        if guard.contains_key(&spec.name) {
            return Err(VectorStoreError::AlreadyExists(spec.name));
        }
        guard.insert(spec.name.clone(), Collection {
            spec,
            points: HashMap::new(),
            texts: HashMap::new(),
        });
        Ok(())
    }

    fn drop_collection(&self, name: &str) -> Result<(), VectorStoreError> {
        let mut guard = self.collections.lock().unwrap();
        guard.remove(name).ok_or_else(|| VectorStoreError::NotFound(name.to_owned()))?;
        Ok(())
    }

    fn describe_collection(&self, name: &str) -> Result<CollectionInfo, VectorStoreError> {
        let guard = self.collections.lock().unwrap();
        let col = guard.get(name).ok_or_else(|| VectorStoreError::NotFound(name.to_owned()))?;
        Ok(CollectionInfo {
            name: col.spec.name.clone(),
            dimensions: col.spec.dimensions,
            point_count: col.points.len() as u64,
            distance: col.spec.distance,
        })
    }

    fn upsert(&self, collection: &str, points: Vec<Point>) -> Result<(), VectorStoreError> {
        let mut guard = self.collections.lock().unwrap();
        let col = guard.get_mut(collection)
            .ok_or_else(|| VectorStoreError::NotFound(collection.to_owned()))?;
        for p in points {
            let dims = col.spec.dimensions;
            if p.vector.len() != dims {
                return Err(VectorStoreError::DimensionMismatch { expected: dims, got: p.vector.len() });
            }
            if let Some(crate::vector_store::PayloadValue::String(s)) = p.payload.get("text") {
                col.texts.insert(p.id.clone(), s.clone());
            }
            col.points.insert(p.id, (p.vector, p.payload));
        }
        Ok(())
    }

    fn query(&self, collection: &str, req: QueryRequest) -> Result<Vec<ScoredPoint>, VectorStoreError> {
        let guard = self.collections.lock().unwrap();
        let col = guard.get(collection)
            .ok_or_else(|| VectorStoreError::NotFound(collection.to_owned()))?;
        let metric = col.spec.distance;

        let mut scored: Vec<ScoredPoint> = col.points.iter()
            .filter(|(_, (_, payload))| {
                req.filter.as_ref().map(|f| filter_matches(payload, f)).unwrap_or(true)
            })
            .map(|(id, (vec, payload))| ScoredPoint {
                id: id.clone(),
                score: metric.score(vec, &req.vector),
                payload: if req.with_payload { payload.clone() } else { Payload::new() },
            })
            .collect();

        scored.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));

        let mut results = scored;
        if let Some(threshold) = req.score_threshold {
            results = apply_score_threshold(results, threshold);
        }

        let results = results.into_iter().skip(req.offset).take(req.top_k).collect();
        Ok(results)
    }

    fn hybrid_query(&self, collection: &str, req: HybridQueryRequest) -> Result<Vec<ScoredPoint>, VectorStoreError> {
        let guard = self.collections.lock().unwrap();
        let col = guard.get(collection)
            .ok_or_else(|| VectorStoreError::NotFound(collection.to_owned()))?;
        let metric = col.spec.distance;

        let mut scored: Vec<ScoredPoint> = col.points.iter()
            .filter(|(_, (_, payload))| {
                req.filter.as_ref().map(|f| filter_matches(payload, f)).unwrap_or(true)
            })
            .map(|(id, (vec, payload))| {
                let vec_score = metric.score(vec, &req.dense_vector);
                let text = col.texts.get(id).map(|s| s.as_str()).unwrap_or("");
                let kw_score = keyword_score_naive(text, &req.keyword);
                let score = hybrid_score(vec_score, kw_score, req.alpha);
                ScoredPoint { id: id.clone(), score, payload: payload.clone() }
            })
            .collect();

        scored.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));

        let mut results = scored;
        if let Some(threshold) = req.score_threshold {
            results = apply_score_threshold(results, threshold);
        }
        Ok(results.into_iter().take(req.top_k).collect())
    }

    fn delete(&self, collection: &str, ids: Vec<PointId>) -> Result<(), VectorStoreError> {
        let mut guard = self.collections.lock().unwrap();
        let col = guard.get_mut(collection)
            .ok_or_else(|| VectorStoreError::NotFound(collection.to_owned()))?;
        for id in ids {
            col.points.remove(&id);
            col.texts.remove(&id);
        }
        Ok(())
    }

    fn delete_by_filter(&self, collection: &str, filter: Filter) -> Result<u64, VectorStoreError> {
        let mut guard = self.collections.lock().unwrap();
        let col = guard.get_mut(collection)
            .ok_or_else(|| VectorStoreError::NotFound(collection.to_owned()))?;
        let before = col.points.len();
        col.points.retain(|_, (_, payload)| !filter_matches(payload, &filter));
        let after = col.points.len();
        Ok((before - after) as u64)
    }
}

// ---- conformance test helper ----------------------------------------------

/// Create a new `MemStore` with a test collection already created.
#[cfg(test)]
pub fn test_store(name: &str, dims: usize) -> MemStore {
    let store = MemStore::new();
    store.create_collection(CollectionSpec::new(name, dims, Distance::Cosine)).unwrap();
    store
}

#[cfg(test)]
pub fn make_point(id: u64, vec: Vec<f32>) -> Point {
    Point::new(id, vec)
}

#[cfg(test)]
pub fn make_text_point(id: u64, vec: Vec<f32>, text: &str) -> Point {
    Point::new(id, vec).with_payload("text", text)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::conformance::suite;
    use crate::vector_store::*;

    fn fresh(dims: usize) -> MemStore {
        let s = MemStore::new();
        s.create_collection(CollectionSpec::new("col", dims, Distance::Cosine)).unwrap();
        s
    }

    #[test]
    fn mem_store_create_and_describe() {
        let store = MemStore::new();
        store.create_collection(CollectionSpec::new("docs", 4, Distance::Cosine)).unwrap();
        let info = store.describe_collection("docs").unwrap();
        assert_eq!(info.dimensions, 4);
        assert_eq!(info.point_count, 0);
    }

    #[test]
    fn mem_store_create_duplicate_is_error() {
        let store = MemStore::new();
        store.create_collection(CollectionSpec::new("docs", 4, Distance::Cosine)).unwrap();
        let err = store.create_collection(CollectionSpec::new("docs", 4, Distance::Cosine));
        assert!(matches!(err, Err(VectorStoreError::AlreadyExists(_))));
    }

    #[test]
    fn mem_store_upsert_and_query_nearest() {
        let store = test_store("docs", 3);
        store.upsert("docs", vec![
            Point::new(1u64, vec![1.0, 0.0, 0.0]),
            Point::new(2u64, vec![0.0, 1.0, 0.0]),
            Point::new(3u64, vec![0.0, 0.0, 1.0]),
        ]).unwrap();
        let results = store.query("docs", QueryRequest::new(vec![1.0, 0.0, 0.0], 1)).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, PointId::Num(1));
    }

    #[test]
    fn mem_store_drop_collection_removes_it() {
        let store = MemStore::new();
        store.create_collection(CollectionSpec::new("tmp", 2, Distance::Cosine)).unwrap();
        store.drop_collection("tmp").unwrap();
        let err = store.describe_collection("tmp");
        assert!(matches!(err, Err(VectorStoreError::NotFound(_))));
    }

    #[test]
    fn mem_store_dimension_mismatch_on_upsert() {
        let store = test_store("col", 3);
        let err = store.upsert("col", vec![Point::new(1u64, vec![1.0, 0.0])]);
        assert!(matches!(err, Err(VectorStoreError::DimensionMismatch { .. })));
    }

    #[test]
    fn mem_store_delete_by_filter() {
        let store = fresh(2);
        store.upsert("col", vec![
            Point::new(1u64, vec![1.0, 0.0]).with_payload("keep", "yes"),
            Point::new(2u64, vec![0.0, 1.0]).with_payload("keep", "no"),
        ]).unwrap();
        let count = store.delete_by_filter("col", Filter::Eq("keep".to_owned(), PayloadValue::String("no".to_owned()))).unwrap();
        assert_eq!(count, 1);
        let info = store.describe_collection("col").unwrap();
        assert_eq!(info.point_count, 1);
    }

    // Full conformance suite runs against MemStore
    #[test]
    fn conformance_full_upsert_query() {
        let s = fresh(3);
        suite::conformance_upsert_then_query_returns_nearest(&s);
    }
    #[test]
    fn conformance_full_filter() {
        let s = fresh(2);
        suite::conformance_metadata_filter_narrows_results(&s);
    }
    #[test]
    fn conformance_full_delete() {
        let s = fresh(2);
        suite::conformance_delete_removes_a_point(&s);
    }
    #[test]
    fn conformance_full_batch() {
        let s = fresh(3);
        suite::conformance_batch_upsert_ordering(&s);
    }
    #[test]
    fn conformance_full_distance() {
        let s = fresh(2);
        suite::conformance_distance_metrics_behave(&s);
    }
    #[test]
    fn conformance_full_pagination() {
        let s = fresh(3);
        suite::conformance_pagination_stable(&s);
    }
    #[test]
    fn conformance_full_hybrid() {
        let s = fresh(2);
        suite::conformance_hybrid_search_merges_dense_and_keyword(&s);
    }
    #[test]
    fn conformance_full_threshold() {
        let s = fresh(3);
        suite::conformance_score_threshold_filters(&s);
    }

    #[test]
    fn mem_store_empty_collection_query_returns_empty() {
        let store = fresh(3);
        let results = store.query("col", QueryRequest::new(vec![1.0, 0.0, 0.0], 10)).unwrap();
        assert!(results.is_empty());
    }

    #[test]
    fn mem_store_upsert_replaces_existing_point() {
        let store = fresh(2);
        store.upsert("col", vec![Point::new(1u64, vec![1.0, 0.0]).with_payload("v", "first")]).unwrap();
        store.upsert("col", vec![Point::new(1u64, vec![1.0, 0.0]).with_payload("v", "second")]).unwrap();
        let info = store.describe_collection("col").unwrap();
        assert_eq!(info.point_count, 1, "upsert must replace, not append");
    }

    #[test]
    fn mem_store_query_respects_top_k() {
        let store = fresh(2);
        for i in 0u64..10 {
            store.upsert("col", vec![Point::new(i, vec![i as f32 / 10.0, 0.5])]).unwrap();
        }
        let results = store.query("col", QueryRequest::new(vec![1.0, 0.0], 3)).unwrap();
        assert_eq!(results.len(), 3);
    }
}
