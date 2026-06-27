# Building a Multi-Agent Graph

Use `ancora.NewGraphSpec` to connect agents into a directed graph.

## Define the graph

```go
primary := ancora.NewGraphNode("primary",  ancora.NodeKindAgent, primarySpec)
verifier := ancora.NewGraphNode("verifier", ancora.NodeKindAgent, verifierSpec)

graph := ancora.NewGraphSpec(
    []ancora.GraphNode{primary, verifier},
    []ancora.GraphEdge{
        {From: "primary", To: "verifier"},
    },
)
```

## Run the graph

```go
agent, _ := ancora.NewAgent()
defer agent.Close()

handle, _ := agent.RunGraph(graph)
events, _ := handle.CollectAll()
```

## Concurrent runs (without a graph)

For ad-hoc concurrency, run two agents independently and wait:

```go
h1, _ := agent.Run(primarySpec)
h2, _ := agent.Run(verifierSpec)

var wg sync.WaitGroup
wg.Add(2)
go func() { defer wg.Done(); h1.CollectAll() }()
go func() { defer wg.Done(); h2.CollectAll() }()
wg.Wait()

fmt.Println("different IDs:", h1.RunID() != h2.RunID())
```

## See also

- [Verifier pattern](verifier.md)
- [Orchestration graph](../../concepts/orchestration-graph.md)
