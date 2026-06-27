# Memory and RAG

Use LanceDB (embedded) or another vector store to inject retrieved passages
as context before the model turn.

## Offline keyword retrieval (tests)

```go
type Passage struct {
    Key     string
    Content string
}

func keywordRetrieve(corpus []Passage, query string, topK int) []Passage {
    // rank by keyword overlap (offline, no embedding model needed)
    ...
}
```

## LanceDB (production)

```go
import "ancora.io/sdk"

store, err := ancora.OpenLanceDB("./data/vectors")
defer store.Close()

// Ingest
store.Upsert(ctx, "doc-1", embedding, map[string]any{"content": "..."})

// Query
results, _ := store.Search(ctx, queryEmbedding, 5)
context := buildContext(results)
spec.Instructions += "\n\nContext:\n" + context
```

## Injecting context into the agent

```go
passages := keywordRetrieve(corpus, userQuery, 3)
context := strings.Join(extractContent(passages), "\n")
spec := ancora.NewAgentSpec("llama3", "Answer using the context below.")
spec.Instructions += "\n\nContext:\n" + context
```

## See also

- [Vector stores](vector-stores.md)
- [Memory tiers concept](../../concepts/memory-tiers.md)
