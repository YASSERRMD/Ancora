# Memory and RAG (.NET)

## Offline keyword retrieval

```csharp
public record Passage(string Key, string Content);

public static IEnumerable<Passage> KeywordRetrieve(
    IEnumerable<Passage> corpus, string query, int topK = 3)
{
    var terms = new HashSet<string>(
        query.ToLowerInvariant().Split(' ', StringSplitOptions.RemoveEmptyEntries));

    return corpus
        .Select(p => (Score: p.Content.ToLowerInvariant()
            .Split(' ').Count(terms.Contains), Passage: p))
        .Where(x => x.Score > 0)
        .OrderByDescending(x => x.Score)
        .Take(topK)
        .Select(x => x.Passage);
}

var corpus = new[]
{
    new Passage("doc1", "Ancora provides durable agent orchestration."),
    new Passage("doc2", "LanceDB is an embedded vector database."),
    new Passage("doc3", "System.Text.Json handles JSON in .NET."),
};

var hits = KeywordRetrieve(corpus, "durable agent").ToList();
var context = string.Join("\n", hits.Select(h => h.Content));
```

## Inject context into the agent

```csharp
var spec = new AgentSpec
{
    Model = "llama3",
    Instructions = $"Answer using only the following context:\n\n{context}",
};
```

## Context as a tool

```csharp
registry.Register(new ToolSpec
{
    Name = "retrieve",
    Description = "Retrieve relevant passages for a query.",
    InputSchema = /* ... */,
    Fn = args =>
    {
        var query = args["query"]!.GetString()!;
        var hits = KeywordRetrieve(corpus, query);
        return string.Join("\n", hits.Select(h => h.Content));
    }
});
```

## See also

- [Vector stores](vector-stores.md)
- [Providers](providers.md)
