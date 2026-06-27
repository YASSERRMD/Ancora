# Orchestration Graph

Ancora's graph executor lets you connect multiple agents into a directed
acyclic graph (DAG), where each node is either an agent or a subgraph.

## GraphSpec

A `GraphSpec` contains:

- **`nodes`** -- list of `GraphNode` entries, each with an `id`, a `kind`
  (`agent` or `subgraph`), and an optional `AgentSpec`.
- **`edges`** -- list of `GraphEdge` entries connecting nodes.

Edges carry the output of one node as the input of the next.

## Node kinds

| Kind | Behaviour |
|------|-----------|
| `agent` | Runs an `AgentSpec` agent loop |
| `subgraph` | Embeds another `GraphSpec` recursively |

## Execution semantics

- Nodes with no incoming edges start immediately.
- A node runs when all its upstream nodes have completed.
- The graph completes when all nodes have completed.
- Any node failure propagates and cancels downstream nodes.

## Example: primary + verifier

```
[primary agent] --> [verifier agent]
```

The verifier receives the primary's output as its system prompt and
returns a structured verdict. Both share the same `Runtime`.

## Routing

The router can branch to different nodes based on the previous node's
output JSON. This enables conditional flows without hardcoding logic in
the model prompt.

## See also

- [Agents](agents.md)
- [Orchestration guide](../guides/orchestration.md)
