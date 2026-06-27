# When to Choose Milvus Over Alternatives

This guide helps you pick the right vector store for your workload. Every store
has a sweet spot; Milvus is not always the answer.

## Quick decision matrix

| Requirement | Best fit |
|---|---|
| Billions of vectors, distributed, production | Milvus |
| Sub-second latency at millions of vectors | Qdrant or Milvus HNSW |
| Embedded, single-binary, offline-first | LanceDB |
| Semantic + keyword hybrid, multi-modal | Weaviate |
| Existing Postgres infra, SQL familiarity | pgvector |
| Rapid prototype, minimal infra | Chroma |
| Fully managed, zero-ops | Pinecone or Zilliz Cloud |

## When Milvus is the right choice

**Scale**: Milvus is designed for billion-scale similarity search. Its
distributed architecture (Pulsar for WAL, MinIO for segment storage, etcd for
metadata) can scale horizontally to handle workloads that would overwhelm a
single-node store.

**Index flexibility**: HNSW for low-latency recall, IVF_FLAT/IVF_SQ8 for
memory-constrained clusters, DISKANN for datasets that don't fit in RAM. No
other open-source store offers this range.

**Partitions**: First-class partition support makes Milvus a natural fit for
multi-tenant SaaS products where each tenant's data must be isolated at the
storage level.

**Consistency control**: Four consistency levels (Strong, Bounded, Session,
Eventually) let you tune the latency vs. freshness trade-off per query.

**Hybrid dense+sparse**: Milvus supports sparse vector fields natively,
enabling BM25 + dense fusion without a separate keyword engine.

## When to prefer something else

| Scenario | Alternative | Reason |
|---|---|---|
| You're starting out (<1M vectors) | Qdrant or Chroma | Simpler ops, no Kubernetes |
| You need rich filtering without a separate filter language | Qdrant | JSON payload filtering is more expressive than Milvus boolean expressions |
| You want full-text search + vector in one query | Weaviate | BM25 + nearVector in GraphQL is native |
| Your data lives in Postgres already | pgvector | No additional infra |
| You need offline / edge deployment | LanceDB | Embedded, no server |
| You need sub-10ms p99 at a few million vectors | Qdrant | HNSW over gRPC is marginally faster for small clusters |

## Milvus vs Qdrant

- **Ops**: Qdrant is a single binary; Milvus standalone requires Docker Compose
  with Pulsar, MinIO, and etcd.
- **Scale**: Milvus cluster mode scales to billions; Qdrant clusters are simpler
  but top out sooner.
- **Filtering**: Qdrant's JSON payload filter is more expressive (nested fields,
  geo). Milvus uses boolean expressions on flat fields.
- **Index types**: Both support HNSW. Milvus additionally offers IVF_*, DISKANN,
  and AUTOINDEX.

## Milvus vs Weaviate

- **Weaviate** integrates text embeddings, BM25 keyword search, and generative
  AI modules natively. You write GraphQL queries, not REST JSON.
- **Milvus** is agnostic to the embedding model; you bring your own vectors.
  It scales better for pure ANN workloads.
- If your use-case is RAG + generative search, Weaviate is often simpler.
  If you need raw throughput and multi-tenancy at scale, Milvus wins.

## Milvus vs pgvector

- **pgvector** lives inside Postgres; if your team already runs Postgres, there
  is no new service to operate.
- Milvus outperforms pgvector by 10-100x at scale (millions+ vectors) because
  it avoids heap-scan overhead and uses dedicated ANN indexes.
- Use pgvector when vectors are a secondary concern. Use Milvus when vector
  search is the primary workload.

## Choosing the right Milvus index

```
Vectors fit in RAM and latency is critical -> HNSW (M=16, ef_construction=200)
Vectors don't fit in RAM, moderate latency -> DISKANN
Vectors fit in RAM, storage is tight -> IVF_SQ8 (8x compression)
You don't want to tune -> AUTOINDEX (Zilliz Cloud only)
Development/CI with small datasets -> FLAT (brute force, exact recall)
```

## Summary

Use Milvus when you anticipate hundreds of millions to billions of vectors,
need fine-grained tenancy via partitions, or require index flexibility beyond
HNSW. For anything smaller or simpler, choose a lighter store and migrate to
Milvus when you hit its ceiling.
