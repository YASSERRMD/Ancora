# Orchestration Guide

## Graphs and nodes

An Ancora run is driven by a `Graph`. A graph has:

- An `entry_node` -- the node where execution starts.
- A list of `Node` objects, each with a unique `id` and a `NodeKind`.
- A list of `Edge` objects connecting nodes.

Supported node kinds:

| Kind | Description |
|------|-------------|
| `Agent` | Calls a model with tools. Most common. |
| `Tool` | Executes a tool directly without a model call. |
| `Decision` | Routes to one of several downstream nodes based on output. |

## Sequential chains

Connect node A to node B with an unconditional edge:

```json
{
    "from": "extract",
    "to": "summarise",
    "condition": null
}
```

Execution flows A -> B -> ... until a node with no outgoing edges (terminal node).

## Conditional branching

Set a `condition` on an edge. The engine evaluates the condition against the
previous node's output JSON and follows the first matching edge:

```json
[
    { "from": "classify", "to": "handle_complaint", "condition": "$.category == 'complaint'" },
    { "from": "classify", "to": "handle_praise",    "condition": "$.category == 'praise'" }
]
```

If no condition matches, the run completes at that node.

## Parallel branches

Fork from one node to multiple nodes with multiple outgoing edges that share
the same `from` value. All target nodes execute concurrently and their outputs
are collected before the run continues:

```json
[
    { "from": "split", "to": "branch_a", "condition": null },
    { "from": "split", "to": "branch_b", "condition": null }
]
```

Convergence nodes wait for all incoming branches before executing.

## Graph validation

Call `Graph::validate()` before running. Validation checks:

- All edge targets exist as node IDs.
- No cycles exist (Ancora runs are acyclic directed graphs).
- `entry_node` exists.
- All nodes are reachable from `entry_node`.

Validation is fast (O(V + E)) and is enforced automatically by the runner.

## Tool calling within nodes

Each `Agent` node receives a `ToolRegistry`. The agent calls tools by name;
the registry validates the input schema before dispatch. Tool results flow back
into the agent's context automatically.

See the [Architecture overview](../spec/architecture.md) for the full tool
execution pipeline.
