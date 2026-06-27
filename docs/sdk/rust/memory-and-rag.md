# Memory and RAG (Rust)

## In-memory context injection

Prepend retrieved passages directly in the prompt:

```rust
let passages = vec![
    "Rust ownership ensures memory safety without a GC.",
    "The borrow checker enforces single mutable ownership.",
];

let context = passages.join("\n\n");
let prompt = format!("Context:\n{}\n\nQuestion: {}", context, user_question);

let mut run = rt.run(&spec, &prompt).await?;
```

## LanceDB vector retrieval

```rust
use ancora_core::lancedb::{LanceStore, EmbeddingModel};

let store = LanceStore::open("./data/knowledge.lance").await?;

let results = store
    .search("Rust memory model", 5)
    .await?;

let context = results
    .iter()
    .map(|r| r.text.as_str())
    .collect::<Vec<_>>()
    .join("\n\n");

let prompt = format!("Context:\n{}\n\nQ: {}", context, user_question);
```

## Indexing documents

```rust
let store = LanceStore::create("./data/knowledge.lance",
    EmbeddingModel::AllMiniLML6V2).await?;

store.upsert(vec![
    ("doc-1", "Rust ownership ensures memory safety without GC."),
    ("doc-2", "The borrow checker enforces single mutable ownership."),
]).await?;
```

## Keyword retrieval (no embeddings)

```rust
use ancora_core::store::MemoryStore;

let mut mem = MemoryStore::new();
mem.add("doc-1", "Rust ownership...");
mem.add("doc-2", "Borrow checker...");

let hits = mem.keyword_search("ownership", 3);
```

## See also

- [Vector stores](vector-stores.md)
- [Durability](durability.md)
