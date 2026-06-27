# Qdrant Backend

The `ancora-memory` crate ships a Qdrant REST client layer under
`backends::qdrant`. It produces typed request bodies and URL strings for
every Qdrant operation without depending on the official `qdrant-client` crate.

## Prerequisites

1. Qdrant running at `http://localhost:6333` (Docker quickstart below)
2. Enable the feature flag:

```toml
ancora-memory = { version = "*", features = ["qdrant"] }
```

```bash
docker run -p 6333:6333 qdrant/qdrant
```

## Quick start

```rust
use ancora_memory::backends::qdrant::{
    QdrantConfig, create_collection_body, upsert_body,
    search_body_with_threshold, payload_to_json, parse_search_response,
};
use ancora_memory::vector_store::{Distance, Payload, PayloadValue};

let cfg = QdrantConfig::local();
// PUT /collections/docs
let body = create_collection_body(1536, &Distance::Cosine);
// upsert
let pts = vec![(1u64, embedding, payload_to_json(&payload))];
let upsert = upsert_body(&pts);
// search
let query = search_body_with_threshold(&query_vec, 10, 0.75, None);
// parse
let results = parse_search_response(&response_json);
```

## Configuration

```rust
let cfg = QdrantConfig::new("https://my-cluster.qdrant.io")
    .with_api_key("API_KEY_FROM_ENV")
    .with_timeout(15);

let auth = cfg.auth_header(); // Some("Bearer <key>")
```

## Distance metrics

| Metric | Qdrant name | When to use |
|---|---|---|
| `Distance::Cosine` | `"Cosine"` | Text embeddings (normalized vectors) |
| `Distance::Dot` | `"Dot"` | Inner-product tasks |
| `Distance::L2` | `"Euclid"` | Image and dense feature embeddings |

## Multi-vector collections

```rust
use ancora_memory::backends::qdrant::create_multi_vector_collection_body;

let body = create_multi_vector_collection_body(&[
    ("text", 384, Distance::Cosine),
    ("image", 512, Distance::L2),
]);
```

Search a specific named vector:

```rust
let body = search_named_vector_body("text", &query_vec, 10, None);
```

## Metadata filters

```rust
use ancora_memory::backends::qdrant::filter_to_qdrant;
use ancora_memory::vector_store::{Filter, PayloadValue};

let f = Filter::Gt("year".to_owned(), PayloadValue::Integer(2020))
    .and(Filter::Eq("lang".to_owned(), PayloadValue::String("en".to_owned())));
let json = filter_to_qdrant(&f);
// { "must": [{ "key": "year", "range": { "gt": 2020 } },
//            { "key": "lang", "match": { "value": "en" } }] }
```

## Hybrid search

Qdrant v1.7+ supports RRF fusion with the `/points/query` endpoint:

```rust
use ancora_memory::backends::qdrant::{rrf_fusion_body, query_url};

let body = rrf_fusion_body(&[dense_vec.as_slice(), second_vec.as_slice()], 10);
// POST /collections/{name}/points/query
```

## Result post-processing

```rust
use ancora_memory::backends::qdrant::{
    sort_by_score, apply_score_threshold, dedup_by_id,
};

let sorted = sort_by_score(raw_results);
let thresholded = apply_score_threshold(sorted, 0.75);
// When combining multiple search results:
let deduped = dedup_by_id(combined_results);
```

## Collection aliases

Aliases enable zero-downtime re-indexing:

```rust
use ancora_memory::backends::qdrant::{create_alias_body, aliases_url};

// Rename alias atomically to point at docs_v2
let body = rename_alias_body("docs", "docs_v2");
// PUT /collections/aliases
```

## Error handling and retry

```rust
use ancora_memory::backends::qdrant::{
    QdrantError, should_retry_status, qdrant_retry_delay_ms, MAX_RETRIES,
};

let err = QdrantError::from_response(status, &body_text);
if err.is_transient() && attempt < MAX_RETRIES {
    std::thread::sleep(std::time::Duration::from_millis(
        qdrant_retry_delay_ms(attempt)
    ));
}
```

Backoff: `200ms * 2^attempt`, capped at 16 seconds. Max 4 retries.

## Snapshots and backups

```rust
use ancora_memory::backends::qdrant::{create_snapshot_url, recover_from_snapshot_body};

// POST create_snapshot_url(base, "docs")
// POST /collections/docs_new/snapshots/recover with recover_from_snapshot_body(snapshot_url)
```

## Integration tests

Integration tests that need a live Qdrant are all `#[ignore]` and read
`QDRANT_URL` from the environment:

```bash
QDRANT_URL=http://localhost:6333 \
  cargo test -p ancora-memory -- --ignored qdrant
```
