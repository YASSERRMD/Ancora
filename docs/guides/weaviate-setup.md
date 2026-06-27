# Weaviate Setup Guide

Weaviate is an open-source vector database that combines dense-vector search, BM25
keyword search, and generative AI modules in a single service. The `ancora-memory`
crate includes a request-building layer for Weaviate's REST and GraphQL APIs.

## Enabling the feature

```toml
[dependencies]
ancora-memory = { version = "*", features = ["weaviate"] }
```

## Running Weaviate locally

The fastest option is Docker Compose:

```bash
docker run -d \
  --name weaviate \
  -p 8080:8080 \
  -e QUERY_DEFAULTS_LIMIT=20 \
  -e AUTHENTICATION_ANONYMOUS_ACCESS_ENABLED=true \
  -e PERSISTENCE_DATA_PATH=/var/lib/weaviate \
  cr.weaviate.io/semitechnologies/weaviate:latest
```

Check readiness:

```bash
curl http://localhost:8080/v1/.well-known/ready
# -> {}
```

## Environment variables

| Variable | Description |
|---|---|
| `WEAVIATE_URL` | Base URL, e.g. `http://localhost:8080` |
| `WEAVIATE_API_KEY` | WCD API key (omit for anonymous access) |

## Creating a class (schema)

```rust
use ancora_memory::backends::weaviate::{
    WeaviateConfig, create_class_with_properties_body,
    schema_url, data_type,
};

let cfg = WeaviateConfig::local();
let body = create_class_with_properties_body(
    "Document", "A RAG document", "none",
    &[
        ("title", data_type::TEXT, "Document title"),
        ("body",  data_type::TEXT, "Document body"),
        ("year",  data_type::INT,  "Publication year"),
    ],
);
// POST cfg.url/v1/schema with body
```

## Upserting objects

```rust
use ancora_memory::backends::weaviate::{
    batch_objects_body, batch_objects_url,
};

let objects = vec![
    ("Document".to_owned(), serde_json::json!({"title": "intro"}), Some(vec![0.1f32; 384])),
    ("Document".to_owned(), serde_json::json!({"title": "usage"}), Some(vec![0.2f32; 384])),
];
let body = batch_objects_body(&objects);
// POST cfg.url/v1/batch/objects with body
```

## Near-vector search

```rust
use ancora_memory::backends::weaviate::{
    graphql_near_vector_query, graphql_url, parse_graphql_get,
};

let query = graphql_near_vector_query("Document", &embedding, 5, &["title", "body"]);
// POST graphql_url(&cfg.url) with query, then:
let results = parse_graphql_get(&response_body, "Document");
```

## Hybrid search

```rust
use ancora_memory::backends::weaviate::graphql_hybrid_query;

// alpha=1.0 is pure vector, alpha=0.0 is pure BM25
let query = graphql_hybrid_query("Document", "machine learning", Some(&embedding), 0.75, 10, &["title"]);
```

## Filtering

```rust
use ancora_memory::backends::weaviate::{where_filter_text, where_filter_and};

let f = where_filter_and(&[
    where_filter_text("title", "Like", "intro*"),
    serde_json::json!({"path": ["year"], "operator": "GreaterThan", "valueInt": 2020}),
]);
```

## Generative search (RAG)

```rust
use ancora_memory::backends::weaviate::graphql_generative_query;

let body = graphql_generative_query(
    "Document", &embedding, 3,
    "Summarize {title} in one sentence.",
    &["title", "body"],
);
// POST graphql_url(&cfg.url) with body
```

## Multi-tenancy

```rust
use ancora_memory::backends::weaviate::{add_tenants_body, tenants_url};

let body = add_tenants_body(&["tenant-a", "tenant-b"]);
// POST tenants_url(&cfg.url, "Document") with body
```

## Running integration tests

```bash
WEAVIATE_URL=http://localhost:8080 cargo test -p ancora-memory --features weaviate -- --ignored weaviate
```

## Index configuration

By default Weaviate creates an HNSW index for every class. You can tune it:

```rust
use ancora_memory::backends::weaviate::{hnsw_index_config, create_class_with_index_body};

let idx = hnsw_index_config(128, 64, 64);
let body = create_class_with_index_body("Document", "none", idx);
```

For low-memory deployments use the flat index with binary quantisation:

```rust
use ancora_memory::backends::weaviate::flat_index_config;

let idx = flat_index_config(true); // true = enable BQ compression
```

## Further reading

- <https://weaviate.io/developers/weaviate>
- Vector index tuning: <https://weaviate.io/developers/weaviate/configuration/indexes>
- Weaviate Cloud (WCD): <https://console.weaviate.cloud>
