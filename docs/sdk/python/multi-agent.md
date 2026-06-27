# Multi-Agent Graphs (Python)

Orchestrate multiple agents by defining a `GraphSpec` with nodes and edges.
Each node is an agent; edges define the data flow between them.

## Define a graph

```python
from ancora import Runtime, AgentSpec, GraphSpec, GraphNode, GraphEdge

writer_spec = AgentSpec(
    model="llama3",
    instructions="Write a concise paragraph on the given topic.",
)

reviewer_spec = AgentSpec(
    model="llama3",
    instructions="Review the paragraph and suggest one improvement.",
)

graph = GraphSpec(
    nodes=[
        GraphNode(id="writer", spec=writer_spec),
        GraphNode(id="reviewer", spec=reviewer_spec),
    ],
    edges=[
        GraphEdge(from_node="writer", to_node="reviewer"),
    ],
)

rt = Runtime()
result = rt.run_graph(graph, "durable AI agents")
print(result.output)
```

## Fan-out graph

Run multiple reviewer agents concurrently and collect their outputs:

```python
graph = GraphSpec(
    nodes=[
        GraphNode(id="writer", spec=writer_spec),
        GraphNode(id="reviewer_a", spec=AgentSpec("llama3", "Focus on clarity.")),
        GraphNode(id="reviewer_b", spec=AgentSpec("llama3", "Focus on accuracy.")),
    ],
    edges=[
        GraphEdge(from_node="writer", to_node="reviewer_a"),
        GraphEdge(from_node="writer", to_node="reviewer_b"),
    ],
)
```

The engine runs `reviewer_a` and `reviewer_b` concurrently once `writer`
completes.

## Accessing node outputs

```python
run = rt.start_graph(graph, "durable AI agents")
for node_id, output in run.node_outputs().items():
    print(node_id, "->", output[:80])
```

## See also

- [Verifier pattern](verifier.md)
- [Orchestration graph concept](../../concepts/orchestration-graph.md)
