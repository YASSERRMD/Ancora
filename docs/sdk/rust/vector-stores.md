# Vector Stores (Rust)

## LanceDB (embedded)

LanceDB runs in-process -- no separate server needed.

```toml
ancora-core = { git = "...", features = ["lancedb"] }
lancedb = "0.4"
```

```rust
use lancedb::{connect, DistanceType};
use arrow_array::{RecordBatch, StringArray, Float32Array};

let db = connect("./data/lance_db").execute().await?;
let table = db.open_table("docs").execute().await?;

let results = table
    .search(query_embedding)
    .distance_type(DistanceType::Cosine)
    .limit(5)
    .execute()
    .await?;
```

## pgvector (PostgreSQL)

```toml
sqlx = { version = "0.7", features = ["postgres", "runtime-tokio-native-tls"] }
pgvector = { version = "0.3", features = ["sqlx"] }
```

```rust
use sqlx::postgres::PgPool;
use pgvector::Vector;

let pool = PgPool::connect(&std::env::var("DATABASE_URL")?).await?;

let results: Vec<(String, f64)> = sqlx::query_as(
    "SELECT content, embedding <=> $1 AS dist FROM docs ORDER BY dist LIMIT 5"
)
.bind(Vector::from(query_embedding))
.fetch_all(&pool)
.await?;
```

## Qdrant

```toml
qdrant-client = "1"
```

```rust
use qdrant_client::{Qdrant, qdrant::{SearchPoints, VectorsConfig, VectorParamsBuilder}};

let client = Qdrant::from_url("http://localhost:6334").build()?;

let results = client.search_points(SearchPoints {
    collection_name: "docs".into(),
    vector: query_embedding,
    limit: 5,
    ..Default::default()
}).await?;
```

## Milvus

```toml
milvus-sdk-rust = "0.3"
```

```rust
use milvus::client::Client;

let client = Client::new("http://localhost:19530").await?;
let results = client
    .search("docs", query_embedding, 5, "embedding")
    .await?;
```

## See also

- [Memory and RAG](memory-and-rag.md)
- [Durability](durability.md)
