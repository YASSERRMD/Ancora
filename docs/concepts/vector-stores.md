# Vector Stores

Ancora supports multiple vector stores for long-term agent memory. All stores
implement the same retrieval interface, so you can switch backends without
changing agent code.

## Supported stores

| Store | Deployment | Distance metric |
|-------|-----------|----------------|
| LanceDB | Embedded (no server) | Cosine / L2 |
| pgvector | PostgreSQL extension | Cosine / L2 / IP |
| Milvus | Dedicated server | Cosine / L2 / IP |
| Qdrant | Dedicated server | Cosine / Dot / Euclid |
| Weaviate | Dedicated server | Cosine |
| In-memory | Process-local | Cosine (hash embedder) |

## Embedders

Ancora provides pluggable embedders:

| Embedder | Use case |
|----------|----------|
| `HashEmbedder` | Offline tests; no model required |
| `OpenAIEmbedder` | Production; high quality |
| `OllamaEmbedder` | Local inference; privacy-preserving |

## Retrieval pipeline

1. **Chunk** -- split documents into passages.
2. **Embed** -- convert passages to vectors.
3. **Store** -- insert into the vector store.
4. **Query** -- embed the query and find the top-K nearest passages.
5. **Rerank** -- optionally rerank with a cross-encoder.
6. **Assemble** -- build a context window from the top passages.

## Choosing a store

- **Edge / offline**: use LanceDB (embedded, zero server setup).
- **Existing PostgreSQL**: use pgvector (no extra infrastructure).
- **Large-scale**: use Milvus or Qdrant (distributed, managed indices).

## See also

- [Memory Tiers](memory-tiers.md)
- [Embeddings and retrieval guide](../guides/embeddings-and-retrieval.md)
