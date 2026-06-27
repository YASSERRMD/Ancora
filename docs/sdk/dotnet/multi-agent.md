# Multi-Agent Graphs (.NET)

Orchestrate multiple agents by defining a `GraphSpec` with nodes and edges.

## Define a graph

```csharp
using Ancora;

var rt = new Runtime();
await using var agent = new Agent(rt);

var writerSpec = new AgentSpec
{
    Model = "llama3",
    Instructions = "Write a concise paragraph on the given topic.",
};

var reviewerSpec = new AgentSpec
{
    Model = "llama3",
    Instructions = "Review the paragraph and suggest one improvement.",
};

var graph = new GraphSpec
{
    Nodes = new List<GraphNode>
    {
        new() { Id = "writer", Spec = writerSpec },
        new() { Id = "reviewer", Spec = reviewerSpec },
    },
    Edges = new List<GraphEdge>
    {
        new() { From = "writer", To = "reviewer" },
    },
};

await foreach (var ev in agent.RunGraph(graph, "durable AI agents").Events())
{
    if (ev is CompletedEvent completed)
        Console.WriteLine(completed.Output);
}
```

## Fan-out graph

```csharp
var graph = new GraphSpec
{
    Nodes = new List<GraphNode>
    {
        new() { Id = "writer", Spec = writerSpec },
        new() { Id = "reviewer_a", Spec = new AgentSpec { Model = "llama3", Instructions = "Focus on clarity." } },
        new() { Id = "reviewer_b", Spec = new AgentSpec { Model = "llama3", Instructions = "Focus on accuracy." } },
    },
    Edges = new List<GraphEdge>
    {
        new() { From = "writer", To = "reviewer_a" },
        new() { From = "writer", To = "reviewer_b" },
    },
};
```

`reviewer_a` and `reviewer_b` run concurrently once `writer` completes.

## See also

- [Verifier](verifier.md)
- [Orchestration graph concept](../../concepts/orchestration-graph.md)
