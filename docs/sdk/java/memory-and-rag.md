# Memory and RAG (Java)

## Offline keyword retrieval

```java
import java.util.*;
import java.util.regex.*;

public record Passage(String key, String content) {}

public static List<Passage> keywordRetrieve(List<Passage> corpus, String query, int topK) {
    var terms = new HashSet<>(Arrays.asList(query.toLowerCase().split("\\W+")));
    return corpus.stream()
        .map(p -> Map.entry(
            (long) Arrays.stream(p.content().toLowerCase().split("\\W+"))
                         .filter(terms::contains).count(),
            p))
        .filter(e -> e.getKey() > 0)
        .sorted(Map.Entry.<Long, Passage>comparingByKey().reversed())
        .limit(topK)
        .map(Map.Entry::getValue)
        .toList();
}

var corpus = List.of(
    new Passage("doc1", "Ancora provides durable agent orchestration."),
    new Passage("doc2", "Milvus is a distributed vector database."),
    new Passage("doc3", "Jackson handles JSON serialisation in Java.")
);

var hits = keywordRetrieve(corpus, "durable agent", 3);
var context = hits.stream().map(Passage::content).reduce("", (a, b) -> a + "\n" + b).strip();
```

## Inject context into the agent

```java
var spec = new AgentSpec(
    "llama3",
    "Answer using only the following context:\n\n" + context,
    List.of(), 1024, 0.3f
);
```

## Context as a tool

```java
var retrieveTool = new ToolSpec(
    "retrieve",
    "Retrieve relevant passages for a query.",
    new ToolInputSchema("object",
        Map.of("query", new ToolInputProperty("string", "Search query")),
        List.of("query")),
    args -> {
        String query = args.get("query").asText();
        return keywordRetrieve(corpus, query, 3).stream()
            .map(Passage::content).reduce("", (a, b) -> a + "\n" + b).strip();
    }
);
```

## See also

- [Vector stores](vector-stores.md)
- [Providers](providers.md)
