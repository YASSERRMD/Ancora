# RAG with Milvus Example

Demonstrates an offline keyword-retrieval step that stands in for a Milvus
vector search, then injects retrieved passages as context before running
the agent.

## What it tests

- A `Passage` record pairs a key with text content
- `keywordRetrieve(corpus, query, topK)` returns passages ranked by keyword
  overlap
- Ranking: "milvus.md" scores highest for a "milvus vector database" query
- Agent run with injected context does not throw

## Pattern

```java
record Passage(String key, String content) {}

static List<Passage> keywordRetrieve(List<Passage> corpus, String query, int topK) {
    String[] terms = query.toLowerCase().split("\\s+");
    return corpus.stream()
        .sorted(Comparator.comparingLong((Passage p) ->
            Arrays.stream(terms)
                .filter(t -> p.content().toLowerCase().contains(t))
                .count()
        ).reversed())
        .limit(topK)
        .toList();
}

List<Passage> hits = keywordRetrieve(corpus, "milvus vector database", 1);
assertEquals("milvus.md", hits.get(0).key());
```

## Offline behaviour

Keyword retrieval is entirely in-process. The agent run catches
`UnsatisfiedLinkError`.
