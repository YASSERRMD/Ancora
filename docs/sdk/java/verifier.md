# Verifier and Consensus (Java)

## Simple verifier

```java
import io.ancora.*;
import java.util.List;

var graph = new GraphSpec(
    List.of(
        new GraphNode("primary", new AgentSpec("llama3", "Answer the question.", List.of(), 1024, 0.7f)),
        new GraphNode("verifier", new AgentSpec(
            "llama3",
            "Verify the answer. Reply 'VERIFIED' or 'REJECTED: <reason>'.",
            List.of(), 512, 0.1f))
    ),
    List.of(new GraphEdge("primary", "verifier"))
);

try (var rt = new Runtime(); var agent = new Agent(rt)) {
    for (var ev : agent.runGraph(graph, "What is the capital of Egypt?").events()) {
        if (ev instanceof RunEvent.Completed c) System.out.println(c.output());
    }
}
```

## N-verifier consensus

```java
import java.util.concurrent.CompletableFuture;

var primarySpec = new AgentSpec("llama3", "Answer the question.", List.of(), 1024, 0.7f);
var verifierSpec = new AgentSpec("llama3", "Is the following answer correct? Reply YES or NO.", List.of(), 64, 0.0f);

String candidate = "";
for (var ev : agent.run(primarySpec, "What is the capital of Egypt?").events())
    if (ev instanceof RunEvent.Completed c) candidate = c.output();

final String finalCandidate = candidate;
var tasks = java.util.stream.IntStream.range(0, 3)
    .mapToObj(i -> CompletableFuture.supplyAsync(() -> {
        for (var ev : agent.run(verifierSpec, finalCandidate).events())
            if (ev instanceof RunEvent.Completed c)
                return c.output().trim().toUpperCase().startsWith("YES");
        return false;
    }))
    .toList();

long yesCount = tasks.stream().map(CompletableFuture::join).filter(v -> v).count();
System.out.println(yesCount >= 2 ? "ACCEPTED" : "REJECTED");
```

## See also

- [Multi-agent graphs](multi-agent.md)
- [Human-in-the-loop](human-in-the-loop.md)
