# RAG with LanceDB Example

Demonstrates an offline keyword-retrieval step that stands in for a LanceDB
vector similarity search, then injects the retrieved passages as context.

## What it tests

- A `Passage` struct pairs a key with text content
- `keyword_retrieve(corpus, query, top_k)` returns passages ranked by
  keyword overlap with the query
- "lancedb.md" ranks first for a "lancedb vector" query
- An empty query still returns the requested number of results

## Pattern

```rust
use ancora_examples::{keyword_retrieve, Passage};

let corpus = vec![
    Passage::new("lancedb.md",  "LanceDB stores vectors with column-level compression."),
    Passage::new("ancora.md",   "Ancora is a multi-agent runtime."),
];

let hits = keyword_retrieve(&corpus, "lancedb vector", 1);
assert_eq!("lancedb.md", hits[0].key);

let context = hits.iter().map(|p| p.content.as_str()).collect::<Vec<_>>().join("\n");
// Inject context into AgentSpec.instructions
```

## Offline

All retrieval is in-process. No LanceDB instance required.
