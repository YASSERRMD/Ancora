# Multi-Agent Quickstart

Run two agents in a graph: a **writer** that drafts a document and a
**reviewer** that critiques it. The writer's output is passed to the
reviewer automatically via a graph edge.

## Prerequisites

Completed the [first agent quickstart](first-agent.md).

---

## Go

```go
import (
    ancora "ancora.io/sdk"
    "sync"
)

func runMultiAgent() {
    rt, _ := ancora.NewRuntime()
    defer rt.Close()

    writerSpec := ancora.NewAgentSpec("llama3", "Write a short paragraph about durable agents.")
    reviewerSpec := ancora.NewAgentSpec("llama3", "Critique the paragraph. Score it 1-10.")

    graph := ancora.GraphSpec{
        Nodes: []ancora.GraphNode{
            {ID: "writer", Spec: writerSpec},
            {ID: "reviewer", Spec: reviewerSpec},
        },
        Edges: []ancora.GraphEdge{
            {From: "writer", To: "reviewer"},
        },
    }

    agent := ancora.NewAgent(rt)
    defer agent.Close()

    var wg sync.WaitGroup
    wg.Add(1)
    go func() {
        defer wg.Done()
        events, _ := agent.RunGraph(graph).CollectAll()
        fmt.Printf("reviewer produced %d events\n", len(events))
    }()
    wg.Wait()
}
```

## Python

```python
from ancora import Runtime, GraphSpec, GraphNode, GraphEdge, AgentSpec

rt = Runtime()

writer_spec = AgentSpec("llama3", "Write a short paragraph about durable agents.")
reviewer_spec = AgentSpec("llama3", "Critique the paragraph. Score it 1-10.")

graph = GraphSpec(
    nodes=[
        GraphNode(id="writer", spec=writer_spec),
        GraphNode(id="reviewer", spec=reviewer_spec),
    ],
    edges=[GraphEdge(from_node="writer", to_node="reviewer")],
)

result = rt.run_graph(graph)
print(result.output)
```

## TypeScript

```typescript
import { buildGraph, Runtime } from 'ancora'

const rt = new Runtime()

const graph = buildGraph({
  nodes: [
    { id: 'writer', model: 'llama3', instructions: 'Write a short paragraph about durable agents.' },
    { id: 'reviewer', model: 'llama3', instructions: 'Critique the paragraph. Score it 1-10.' },
  ],
  edges: [{ from: 'writer', to: 'reviewer' }],
})

const result = await rt.runGraph(graph)
console.log(result.output)
```

## .NET

```csharp
var graph = new GraphSpec {
    Nodes = new List<GraphNode> {
        new() { Id = "writer", Spec = new AgentSpec { Model = "llama3", Instructions = "Write a short paragraph about durable agents." } },
        new() { Id = "reviewer", Spec = new AgentSpec { Model = "llama3", Instructions = "Critique the paragraph. Score it 1-10." } },
    },
    Edges = new List<GraphEdge> { new() { From = "writer", To = "reviewer" } },
};

var handle = agent.RunGraph(graph);
await foreach (var ev in handle.Events()) {
    if (ev is CompletedEvent c) Console.WriteLine(c.Output);
}
```

## Java

```java
var graph = new GraphSpec(
    List.of(
        new GraphNode("writer", new AgentSpec("llama3", "Write a short paragraph about durable agents.", List.of(), 1024, 0.7f)),
        new GraphNode("reviewer", new AgentSpec("llama3", "Critique the paragraph. Score it 1-10.", List.of(), 1024, 0.3f))
    ),
    List.of(new GraphEdge("writer", "reviewer"))
);

for (var ev : agent.runGraph(graph).events()) {
    if (ev instanceof RunEvent.Completed c) System.out.println(c.output());
}
```

---

## How it works

1. The graph starts at the `writer` node.
2. When the writer's run completes, the engine serialises its final output
   and injects it as context into the `reviewer` node's initial message.
3. The reviewer run begins with the writer's output already in scope.

Edge semantics are defined in the
[Orchestration graph concept](../concepts/orchestration-graph.md).

## Next steps

- [Verifier pattern](../sdk/go/verifier.md) -- N-agent consensus
- [Human-in-the-loop](../sdk/go/human-in-the-loop.md) -- pause for approval
- [Durability guide](../guides/durability.md) -- crash recovery across nodes
