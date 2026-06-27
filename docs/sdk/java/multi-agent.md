# Multi-Agent Graphs (Java)

Orchestrate multiple agents by defining a `GraphSpec` with nodes and edges.

## Define a graph

```java
import io.ancora.*;
import java.util.List;

var writerSpec = new AgentSpec("llama3", "Write a paragraph on the topic.", List.of(), 1024, 0.7f);
var reviewerSpec = new AgentSpec("llama3", "Review and suggest one improvement.", List.of(), 1024, 0.3f);

var graph = new GraphSpec(
    List.of(
        new GraphNode("writer", writerSpec),
        new GraphNode("reviewer", reviewerSpec)
    ),
    List.of(new GraphEdge("writer", "reviewer"))
);

try (var rt = new Runtime(); var agent = new Agent(rt)) {
    for (var ev : agent.runGraph(graph, "durable AI agents").events()) {
        if (ev instanceof RunEvent.Completed c) {
            System.out.println(c.output());
        }
    }
}
```

## Fan-out graph

```java
var graph = new GraphSpec(
    List.of(
        new GraphNode("writer", writerSpec),
        new GraphNode("reviewer_a", new AgentSpec("llama3", "Focus on clarity.", List.of(), 512, 0.3f)),
        new GraphNode("reviewer_b", new AgentSpec("llama3", "Focus on accuracy.", List.of(), 512, 0.3f))
    ),
    List.of(
        new GraphEdge("writer", "reviewer_a"),
        new GraphEdge("writer", "reviewer_b")
    )
);
```

`reviewer_a` and `reviewer_b` run concurrently once `writer` completes.

## See also

- [Verifier](verifier.md)
- [Orchestration graph concept](../../concepts/orchestration-graph.md)
