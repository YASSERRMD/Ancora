# Vector Store Coverage Matrix

## Ancora 0.6.0

| Backend | Conformance | Hybrid Search | Offline Test | Notes |
|---------|-------------|---------------|--------------|-------|
| inmemory | full | dot product only | yes | default, always available |
| sqlite | full | FTS + vector | yes | requires sqlite3 |
| pgvector | full | ivfflat + hnsw | yes | requires pg extension |
| qdrant | full | payload filter | yes | recorded fixture |
| weaviate | full | BM25 + vector | yes | recorded fixture |
| milvus | full | IVF_FLAT | yes | recorded fixture |
| lancedb | full | full-text | yes | recorded fixture |
| chroma | full | metadata filter | yes | recorded fixture |
| pinecone | full | metadata filter | yes | recorded fixture |
| vespa | full | BM25 + ANN | yes | recorded fixture |
| redis | full | VSS | yes | recorded fixture |

## Conformance tests

All 11 backends must pass:

1. Insert N vectors without error.
2. Retrieve top-k by cosine similarity.
3. Handle empty result set (query returns []).
4. Reject vectors of wrong dimension.
5. Produce deterministic results for identical inputs.

## Dimension and metric defaults

- Default dimension: 128 (configurable via `VectorStoreConfig.dim`)
- Default metric: cosine similarity
- Cosine similarity via dot product on L2-normalised vectors
