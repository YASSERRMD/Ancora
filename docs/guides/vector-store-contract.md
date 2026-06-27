# Vector Store Contract

Ancora's `VectorStore` trait is the single interface every vector backend must
satisfy. Implementing it gives you automatic compatibility with the agent
memory system, the conformance test suite, and any future backend that honors
the same contract.

## The trait

```rust
pub trait VectorStore: Send + Sync {
    // Collection lifecycle
    fn create_collection(&self, spec: CollectionSpec) -> Result<(), VectorStoreError>;
    fn drop_collection(&self, name: &str) -> Result<(), VectorStoreError>;
    fn describe_collection(&self, name: &str) -> Result<CollectionInfo, VectorStoreError>;

    // Write
    fn upsert(&self, collection: &str, points: Vec<Point>) -> Result<(), VectorStoreError>;
    fn batch_upsert(&self, collection: &str, batches: Vec<Vec<Point>>) -> Result<(), VectorStoreError>;

    // Read
    fn query(&self, collection: &str, req: QueryRequest) -> Result<Vec<ScoredPoint>, VectorStoreError>;
    fn hybrid_query(&self, collection: &str, req: HybridQueryRequest) -> Result<Vec<ScoredPoint>, VectorStoreError>;

    // Delete
    fn delete(&self, collection: &str, ids: Vec<PointId>) -> Result<(), VectorStoreError>;
    fn delete_by_filter(&self, collection: &str, filter: Filter) -> Result<u64, VectorStoreError>;
}
```

## Key types

### `CollectionSpec`

```rust
let spec = CollectionSpec::new("docs", 1536, Distance::Cosine)
    .with_schema(
        PayloadSchema::new()
            .field("source", FieldType::Keyword)
            .field("year", FieldType::Integer)
    )
    .with_index(IndexConfig::Hnsw(HnswConfig { m: 16, ef_construct: 100 }));
```

### `Point`

```rust
let point = Point::new(42u64, vec![0.1, 0.2, ...])
    .with_payload("source", "wikipedia")
    .with_payload("year", 2024i64)
    .with_named_vector("image", image_embedding);
```

### `QueryRequest`

```rust
let req = QueryRequest::new(query_vec, 10)
    .with_filter(Filter::Eq("source".to_owned(), PayloadValue::String("wikipedia".to_owned())))
    .with_score_threshold(0.75)
    .with_offset(0)
    .with_vector_name("text"); // multi-vector: select which vector to search
```

### `HybridQueryRequest`

```rust
let req = HybridQueryRequest::new(dense_vec, "machine learning", 10)
    .with_alpha(0.7)  // 0.0 = pure keyword, 1.0 = pure vector
    .with_filter(Filter::Gt("year".to_owned(), PayloadValue::Integer(2020)));
```

## Distance metrics

| Metric | When to use |
|--------|-------------|
| `Distance::Cosine` | Normalized embeddings, text/semantic search |
| `Distance::Dot` | Dot-product-maximization tasks, some reranking models |
| `Distance::L2` | Euclidean geometry, image embeddings |

The `Distance::score` method always returns a higher value for more-similar
vectors, regardless of the underlying metric.

## Metadata filters

```rust
// Single condition
Filter::Eq("tag".to_owned(), PayloadValue::String("news".to_owned()))

// Compound
Filter::Gt("year".to_owned(), PayloadValue::Integer(2020))
    .and(Filter::Eq("lang".to_owned(), PayloadValue::String("en".to_owned())))

// Filter evaluator (for in-memory / pre-filter backends)
let matches: bool = filter_matches(&payload, &filter);
```

## Batch upsert

```rust
let batches = make_batches(all_points, BatchConfig::default().batch_size);
store.batch_upsert("docs", batches)?;
```

## Conformance suite

Every backend should run the conformance suite against its implementation:

```rust
use ancora_memory::conformance::suite;

// In your backend's test module:
#[test]
fn my_backend_passes_conformance() {
    let store = MyBackend::connect(...);
    store.create_collection(CollectionSpec::new("col", 3, Distance::Cosine)).unwrap();
    suite::conformance_upsert_then_query_returns_nearest(&store);
    suite::conformance_metadata_filter_narrows_results(&store);
    suite::conformance_delete_removes_a_point(&store);
    suite::conformance_batch_upsert_ordering(&store);
    suite::conformance_distance_metrics_behave(&store);
    suite::conformance_pagination_stable(&store);
    suite::conformance_hybrid_search_merges_dense_and_keyword(&store);
    suite::conformance_score_threshold_filters(&store);
}
```

## In-memory reference implementation

`MemStore` in `ancora_memory::mem_store` is the zero-dependency reference
implementation. It passes the full conformance suite and is suitable for unit
tests and prototyping. It uses a flat linear scan -- not suitable for
production with large collections.

```rust
use ancora_memory::mem_store::MemStore;
use ancora_memory::vector_store::*;

let store = MemStore::new();
store.create_collection(CollectionSpec::new("docs", 384, Distance::Cosine))?;
store.upsert("docs", points)?;
let results = store.query("docs", QueryRequest::new(query_vec, 5))?;
```
