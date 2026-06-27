# Memory Tiers

Ancora organises agent memory into three tiers based on scope, latency, and
persistence requirements.

## Tier overview

| Tier | Scope | Storage | Latency |
|------|-------|---------|---------|
| **In-context** | Current model turn | Token window | None |
| **Working** | Current run | In-process / SQLite | Microseconds |
| **Long-term** | Across runs | Vector store | Milliseconds |

## In-context memory

The model's token window is the fastest tier. Ancora automatically populates
it from the agent's message history and any retrieved passages.

## Working memory

Working memory lives for the duration of a single run. Ancora uses
`MemoryStore` (in-process) or SQLite for this tier. It holds:

- Retrieved passages (before injecting into context)
- Intermediate tool results
- Checkpoint data for replay

## Long-term memory

Long-term memory persists across runs and is backed by a vector store:
LanceDB (edge, embedded), pgvector (PostgreSQL), Milvus, Qdrant, or Weaviate.

Agents retrieve relevant passages at the start of each turn using semantic or
keyword search. Embeddings are computed locally (HashEmbedder for offline
tests, sentence-transformers for production).

## Choosing a tier

- Use **in-context** for conversational history and retrieved passages.
- Use **working** for temporary state within a run (scratch pad, partial
  results).
- Use **long-term** for facts, documents, and knowledge bases that outlive
  individual runs.

## See also

- [Vector Stores](vector-stores.md)
- [Memory guide](../guides/memory.md)
