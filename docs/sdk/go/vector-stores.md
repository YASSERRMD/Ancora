# Vector Store Selection (Go)

Choose the vector store that matches your deployment model.

## LanceDB (embedded, edge)

No server required. Data lives in a local directory or object storage.

```go
store, _ := ancora.OpenLanceDB("./data/vectors")
defer store.Close()
```

## pgvector (PostgreSQL)

```go
store, _ := ancora.OpenPgvector("postgres://user:pass@localhost/db")
defer store.Close()
```

## Milvus

```go
store, _ := ancora.OpenMilvus(ancora.MilvusConfig{
    Address:    "localhost:19530",
    Collection: "agent_memory",
})
```

## Qdrant

```go
store, _ := ancora.OpenQdrant(ancora.QdrantConfig{
    URL:        "http://localhost:6333",
    Collection: "agent_memory",
})
```

## Choosing

| Scenario | Store |
|----------|-------|
| Edge / offline | LanceDB |
| Existing Postgres | pgvector |
| Large-scale managed | Milvus or Qdrant |

## See also

- [Memory and RAG](memory-and-rag.md)
- [Vector stores concept](../../concepts/vector-stores.md)
