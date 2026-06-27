# Chroma and Managed-Service Vector Backends

Ancora ships with four additional vector backends alongside the embedded and
self-hosted options: **Chroma**, **Pinecone**, **Vespa**, and **Redis Vector**
(RediSearch). This guide explains when to pick each one, how to configure it,
and what the `ancora-memory` feature flags look like.

---

## Choosing a Backend

| Backend | Deployment | Best for |
|---------|-----------|---------|
| Chroma | Self-hosted or cloud | Lightweight dev, RAG prototypes, local Docker |
| Pinecone | Fully managed SaaS | Zero-ops production at scale, serverless billing |
| Vespa | Self-hosted or Vespa Cloud | Hybrid ANN+BM25, ranking pipelines, large corpora |
| Redis Vector | Self-hosted Redis Stack | Sub-millisecond latency, existing Redis infra |

Use the `backend_selector::select_backend("name")` helper to resolve a
backend at runtime and verify that the required Cargo feature is present.

---

## Cargo Features

```toml
# Cargo.toml
[dependencies]
ancora-memory = { version = "0.1", features = ["chroma", "pinecone", "vespa", "redis-vector"] }
```

Each feature is independent; enable only the backends you need.

---

## Chroma

Chroma is an open-source embedding database designed for RAG and developer
experience. The v2 API uses tenant and database scoping.

### Configuration

```rust
use ancora_memory::backends::chroma::{ChromaConfig, create_collection_body};

let cfg = ChromaConfig {
    url: "http://localhost:8000".to_owned(),
    tenant: "default_tenant".to_owned(),
    database: "default_database".to_owned(),
    api_key: None,
    timeout_secs: 30,
};
```

### Metadata Filters

Chroma's `where` clause uses MongoDB-style operators:

```rust
use ancora_memory::backends::chroma::{where_eq, where_gt, where_and, query_body};

let filter = where_and(
    where_eq("lang", serde_json::json!("en")),
    where_gt("score", serde_json::json!(0.7)),
);
let body = query_body(&[embedding], 10, Some(filter), &["distances", "ids"]);
```

Operators: `$eq`, `$ne`, `$gt`, `$lt`, `$in`, `$and`, `$or`.

### Retry Policy

`ChromaError::InternalError` (5xx) is transient. Default `MAX_RETRIES = 3`
with exponential backoff via `chroma_retry_delay_ms(attempt)`.

---

## Pinecone

Pinecone is a managed cloud vector database offering serverless and pod-based
tiers. Ops are split: index management uses the controller URL, vector
operations use the per-index data-plane host.

### Configuration

```rust
use ancora_memory::backends::pinecone::PineconeConfig;

let cfg = PineconeConfig {
    api_key: std::env::var("PINECONE_API_KEY").unwrap(),
    environment: "us-east1-gcp".to_owned(),
    timeout_secs: 30,
};
// controller URL for index creation
let ctrl = cfg.controller_url();
// per-index data-plane host (from DescribeIndex response)
let host = "my-idx-abc.svc.pinecone.io";
let data_url = cfg.index_url(host);
```

### Index Creation

```rust
use ancora_memory::backends::pinecone::{create_serverless_index_body, metric};

let body = create_serverless_index_body("my-idx", 768, metric::COSINE, "aws", "us-east-1");
```

### Metadata Filters

```rust
use ancora_memory::backends::pinecone::{filter_eq, filter_gte, filter_and, query_body};

let filter = filter_and(vec![
    filter_eq("lang", serde_json::json!("en")),
    filter_gte("score", serde_json::json!(0.8)),
]);
let body = query_body(&embedding, 20, Some(filter), true);
```

### Namespaces

Use namespaces to isolate data within an index:

```rust
use ancora_memory::backends::pinecone::{upsert_namespace_body, query_namespace_body};

let upsert = upsert_namespace_body(&vectors, "tenant-a");
let query  = query_namespace_body(&embedding, 10, "tenant-a");
```

### Retry Policy

`PineconeError::InternalError` (5xx) and `RateLimited` (429) are transient.
Default `MAX_RETRIES = 4` with exponential backoff capped at 10 seconds.

---

## Vespa

Vespa is a large-scale serving engine supporting ANN, BM25, and custom
machine-learned ranking. It is particularly strong for hybrid retrieval
and personalised ranking pipelines.

### Configuration

```rust
use ancora_memory::backends::vespa::VespaConfig;

let cfg = VespaConfig {
    url: "http://localhost:8080".to_owned(),
    application: "my-app".to_owned(),
    api_key: None,
    timeout_secs: 60,
};
```

### Queries

```rust
use ancora_memory::backends::vespa::{ann_query, bm25_query, hybrid_query};

// ANN only
let ann = ann_query("doc", 10, "embedding", "query_embedding");

// BM25 keyword search
let bm25 = bm25_query("doc", "body", "vector database tutorial", 10);

// Hybrid -- alpha weights dense vs sparse
let hybrid = hybrid_query("doc", 10, "embedding", "query_embedding", "body", 5, 0.6);
```

### Ranking Profiles

Use `hybrid_ranking_profile(alpha)` to generate a Vespa ranking profile
definition that blends closeness (dense) and BM25 (sparse) linearly:

```rust
use ancora_memory::backends::vespa::hybrid_ranking_profile;
let profile = hybrid_ranking_profile(0.7); // 70% dense, 30% sparse
```

### Document Feed

```rust
use ancora_memory::backends::vespa::{feed_url, put_document_body};
use ancora_memory::backends::vespa::VespaConfig;

let cfg = VespaConfig::local();
let url = feed_url(&cfg, "doc", "doc-001");
let body = put_document_body(serde_json::json!({"title": "hello", "embedding": [0.1, 0.2]}));
```

---

## Redis Vector (RediSearch)

Redis Stack bundles RediSearch, which adds vector similarity search to Redis.
It is an excellent choice when you already run Redis and need sub-millisecond
ANN latency.

### Configuration

```rust
use ancora_memory::backends::redis_vector::RedisVectorConfig;

let cfg = RedisVectorConfig::new("localhost", 6379)
    .with_password("your-redis-password")
    .with_tls();

println!("{}", cfg.url()); // rediss://:your-redis-password@localhost:6379
```

### Index Creation (HNSW)

```rust
use ancora_memory::backends::redis_vector::{CreateIndexArgs, field_type};

let idx = CreateIndexArgs::new("docs_idx", "doc:", 768)
    .hnsw_params(200, 16)
    .add_field("lang", field_type::TAG)
    .add_field("score", field_type::NUMERIC)
    .add_field("title", field_type::TEXT);

let descriptor = idx.to_json();
// Pass descriptor["schema"] to FT.CREATE via your Redis client
```

### Filtered ANN Search

```rust
use ancora_memory::backends::redis_vector::{SearchArgs, tag_filter, numeric_range};

let pre = format!(
    "({}) ({})",
    tag_filter("lang", "en"),
    numeric_range("score", 0.8, 1.0),
);
let search = SearchArgs::filtered_ann("docs_idx", &pre, "embedding", 20)
    .returns(&["score", "payload"]);
```

### Error Handling

`RedisVectorError::OutOfMemory` is the only transient error. Index-not-found
and wrong-type errors are permanent. Check `is_transient()` before retrying.

---

## Backend Selector

Use `backend_selector::select_backend` to validate backend names at startup:

```rust
use ancora_memory::backends::backend_selector::{select_backend, known_backends};

let name = std::env::var("VECTOR_BACKEND").unwrap_or_else(|_| "chroma".to_owned());
let info = select_backend(&name).expect("unknown backend");
println!("Using {} on port {:?}", info.display_name, info.default_port);
```

All nine supported names: `pgvector`, `qdrant`, `weaviate`, `milvus`,
`lancedb`, `chroma`, `pinecone`, `vespa`, `redis-vector`.
