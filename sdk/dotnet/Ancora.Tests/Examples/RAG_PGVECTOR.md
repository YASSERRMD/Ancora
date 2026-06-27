# RAG with pgvector Example

Demonstrates an offline keyword-retrieval step that stands in for a real
pgvector similarity search, then injects the retrieved passages as context
before running the agent.

## What it tests

- A `Passage` record pairs a key with text content
- `KeywordRetrieve(corpus, query, topK)` returns the most relevant passages
  ranked by keyword overlap
- Ranking: "pgvector.md" scores highest for a "pgvector" query
- Agent run with injected context does not throw

## Pattern

```csharp
record Passage(string Key, string Content);

static IReadOnlyList<Passage> KeywordRetrieve(
    IEnumerable<Passage> corpus, string query, int topK)
{
    var terms = query.ToLowerInvariant().Split(' ',
        StringSplitOptions.RemoveEmptyEntries);
    return corpus
        .Select(p => (p, score: terms.Count(t =>
            p.Content.Contains(t, StringComparison.OrdinalIgnoreCase))))
        .OrderByDescending(x => x.score)
        .Take(topK)
        .Select(x => x.p)
        .ToList();
}

var corpus = new[]
{
    new Passage("pgvector.md", "pgvector enables fast approximate nearest-neighbour search."),
    new Passage("intro.md",    "Ancora is a multi-agent runtime."),
};
var hits = KeywordRetrieve(corpus, "pgvector similarity", topK: 1);
Assert.Equal("pgvector.md", hits[0].Key);
```

## Offline behaviour

Keyword retrieval is entirely in-process. The agent run catches
`DllNotFoundException`.
