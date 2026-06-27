# Milvus Setup Guide

Milvus is an open-source vector database purpose-built for billion-scale
similarity search. It supports multiple index types (HNSW, IVF_FLAT, DISKANN,
AUTOINDEX), partitions, consistency levels, and hybrid dense+sparse search.

## Enabling the feature

```toml
[dependencies]
ancora-memory = { version = "*", features = ["milvus"] }
```

## Running Milvus locally

Docker Compose is the fastest local path:

```bash
# Standalone mode (single node, no Kubernetes)
wget https://raw.githubusercontent.com/milvus-io/milvus/master/deployments/docker/standalone/docker-compose.yml
docker compose up -d
```

Check readiness:

```bash
curl http://localhost:19530/healthz
# -> OK
```

## Environment variables

| Variable | Description |
|---|---|
| `MILVUS_URL` | Base URL, e.g. `http://localhost:19530` |
| `MILVUS_API_KEY` | API key for Zilliz Cloud (omit for anonymous access) |

## Creating a collection

```rust
use ancora_memory::backends::milvus::{
    MilvusConfig, create_collection_hnsw_body,
    collections_url, metric_type,
};

let cfg = MilvusConfig::local();
let body = create_collection_hnsw_body(
    "DocumentIndex", 768, metric_type::COSINE, 16, 200,
);
// POST collections_url(&cfg.url) with body
```

### Index types

| Type | When to use |
|---|---|
| `HNSW` | Low latency, memory fits in RAM |
| `IVF_FLAT` | Large datasets, balanced memory/speed |
| `IVF_SQ8` | Like IVF_FLAT with scalar quantization (4x smaller) |
| `DISKANN` | Billion-scale, disk-backed |
| `AUTOINDEX` | Let Milvus choose (Zilliz Cloud) |

## Sizing guidance

```rust
use ancora_memory::backends::milvus::{sizing_guidance, recommended_nlist};

let guide = sizing_guidance(768, 100_000_000); // 100M docs
let nlist = recommended_nlist(100_000_000);
```

## Inserting entities

```rust
use ancora_memory::backends::milvus::{
    insert_entities_body, entities_insert_url,
};

let docs = vec![
    (embedding_a, serde_json::json!({"title": "Intro to RAG"})),
    (embedding_b, serde_json::json!({"title": "Vector search"})),
];
let body = insert_entities_body("DocumentIndex", &docs);
// POST entities_insert_url(&cfg.url) with body
```

## Searching

```rust
use ancora_memory::backends::milvus::{
    search_body, search_with_filter_body, entities_search_url, metric_type,
};

// Basic search
let body = search_body("DocumentIndex", &query_embedding, 10, metric_type::COSINE, &["payload"]);

// With boolean expression filter
let body = search_with_filter_body(
    "DocumentIndex", &query_embedding, 10, metric_type::COSINE,
    "year > 2022", &["payload"],
);
// POST entities_search_url(&cfg.url) with body
```

## Boolean expression filters

```rust
use ancora_memory::backends::milvus::{
    expr_eq_str, expr_gt, expr_and,
};

let expr = expr_and(&expr_gt("year", 2022), &expr_eq_str("status", "published"));
// "year > 2022" and "status == \"published\""
```

## Partitions

Partitions scope inserts and searches to a subset of the collection.
Useful for multi-tenancy, time-based sharding, or regional isolation.

```rust
use ancora_memory::backends::milvus::{
    create_partition_body, load_partition_body,
    insert_into_partition_body, search_partition_body,
    partitions_url, partition_load_url,
};

let create = create_partition_body("DocumentIndex", "region_us");
// POST partitions_url(&cfg.url) with create

let load = load_partition_body("DocumentIndex", &["region_us"]);
// POST partition_load_url(&cfg.url) with load

let insert = insert_into_partition_body("DocumentIndex", "region_us", &docs);
let search = search_partition_body("DocumentIndex", "region_us", &query, 10, metric_type::COSINE);
```

## Consistency levels

| Level | Trade-off |
|---|---|
| `Strong` | Linearizable reads, highest latency |
| `Bounded` | Bounded staleness, good default |
| `Session` | Read-your-own-writes |
| `Eventually` | Lowest latency, may see stale data |

```rust
use ancora_memory::backends::milvus::{
    search_with_consistency_body, consistency,
};

let body = search_with_consistency_body(
    "DocumentIndex", &query, 10, metric_type::COSINE, consistency::BOUNDED,
);
```

## Hybrid dense+sparse search

```rust
use ancora_memory::backends::milvus::hybrid_search_body;

let sparse = vec![(0u32, 0.9f32), (42u32, 0.4f32)];
let body = hybrid_search_body(
    "DocumentIndex", &dense_embedding, "sparse_embedding", &sparse,
    10, metric_type::COSINE,
);
```

## Deleting entities

```rust
use ancora_memory::backends::milvus::{delete_by_expr_body, delete_by_ids_body};

let body = delete_by_expr_body("DocumentIndex", "status == \"archived\"");
let body = delete_by_ids_body("DocumentIndex", &[101, 102, 103]);
```

## Running integration tests

```bash
MILVUS_URL=http://localhost:19530 cargo test -p ancora-memory --features milvus -- --ignored milvus
```

## Further reading

- Milvus docs: <https://milvus.io/docs>
- Index selection guide: <https://milvus.io/docs/index.md>
- Partition concepts: <https://milvus.io/docs/partitions_in_milvus.md>
- Zilliz Cloud (managed Milvus): <https://zilliz.com>
