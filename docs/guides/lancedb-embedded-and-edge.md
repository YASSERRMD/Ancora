# LanceDB Embedded and Edge Notes

LanceDB is an embedded, serverless vector database. It stores data in the
Lance columnar format on the local filesystem or object storage (S3, GCS,
Azure Blob). No server process is required -- all vector operations happen
in-process.

## Why LanceDB for edge and single-binary deployments

- **Zero ops**: no Docker, no external service, no port management
- **Offline-first**: works without network access
- **Single binary**: ship your Rust binary; the DB is just a directory
- **Object-storage native**: same code reads from local disk or S3
- **Versioning built-in**: every write is a new version; time-travel is free

## Enabling the feature

```toml
[dependencies]
ancora-memory = { version = "*", features = ["lancedb"] }
```

## Opening a database

```rust
use ancora_memory::backends::lancedb::{LanceDbConfig, edge_config};

// Edge / single-binary -- reads ANCORA_LANCEDB_DIR or falls back to ./ancora_lancedb
let cfg = edge_config();

// Explicit local path
let cfg = LanceDbConfig::local("/data/lancedb");

// S3 (same API, different URI)
let cfg = LanceDbConfig::s3("s3://my-bucket/lancedb", "us-east-1");
```

## Table schema

```rust
use ancora_memory::backends::lancedb::{table_schema, ColumnDef, column_type};

let schema = table_schema(384, &[
    ColumnDef::new("title", column_type::UTF8),
    ColumnDef::new("year",  column_type::INT64).required(),
]);
```

## Adding rows

```rust
use ancora_memory::backends::lancedb::{row, rows};

let batch = rows(&[
    (1, embedding_a, serde_json::json!({"title": "Intro"})),
    (2, embedding_b, serde_json::json!({"title": "Guide"})),
]);
```

## Vector search

```rust
use ancora_memory::backends::lancedb::VectorQuery;

let q = VectorQuery::new("documents", query_embedding, 10)
    .metric("cosine")
    .filter("year > 2022")
    .ef(64)
    .select(&["id", "title", "year"]);
```

## Hybrid vector + full-text search

LanceDB supports hybrid search when a full-text index exists on the column.

```rust
use ancora_memory::backends::lancedb::{HybridQuery, FullTextIndex};

// Create FTS index descriptor
let fts = FullTextIndex::new("title").with_position();

// Build hybrid query
let q = HybridQuery::new("documents", query_embedding, "RAG pipelines", 10)
    .reranker("rrf")
    .filter("year > 2021");
```

## ANN indexing

```rust
use ancora_memory::backends::lancedb::AnnIndex;

// IVF_PQ index for large tables
let idx = AnnIndex::new(256, 16).metric("cosine");
```

## Versioning and time-travel

Every table write in LanceDB creates a new version. You can check out any
prior version or restore to it.

```rust
use ancora_memory::backends::lancedb::{
    VersionCheckout, checkout_as_of, restore_version,
};

// Check out a specific version number
let vc = VersionCheckout::new("documents", 3);

// Check out by Unix timestamp
let ts = checkout_as_of("documents", 1720000000);

// Restore to a prior version (discards newer versions)
let rv = restore_version("documents", 2);
```

## Deleting rows

```rust
use ancora_memory::backends::lancedb::delete_predicate;

// Delete by SQL predicate
let pred = delete_predicate("documents", "year < 2020");
```

## Multimodal support

```rust
use ancora_memory::backends::lancedb::multimodal_row;

// Dual-embedding row (text + image)
let r = multimodal_row(1, text_embedding, image_embedding, serde_json::json!({}));
```

## Object-storage paths

The same `LanceDbConfig` API works for any storage backend:

| URI prefix | Backend |
|---|---|
| `/local/path` | Local filesystem |
| `s3://bucket/prefix` | Amazon S3 (requires `aws_region`) |
| `gs://bucket/prefix` | Google Cloud Storage |
| `az://container/blob` | Azure Blob Storage |

```rust
use ancora_memory::backends::lancedb::detect_storage_type;

assert_eq!(detect_storage_type("s3://b/k"), "s3");
```

## Running integration tests

```bash
LANCEDB_DIR=/tmp/test_lancedb cargo test -p ancora-memory --features lancedb -- --ignored lancedb
```

## Further reading

- LanceDB docs: <https://lancedb.github.io/lancedb>
- Lance format: <https://lancedb.github.io/lance>
- Versioning: <https://lancedb.github.io/lancedb/guides/versioning>
