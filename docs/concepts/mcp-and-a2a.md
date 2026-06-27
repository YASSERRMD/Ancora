# MCP and A2A Interop

Ancora interoperates with two open protocols for agent communication:

- **MCP (Model Context Protocol)** -- a standard for exposing tools and
  resources to models.
- **A2A (Agent-to-Agent)** -- a standard for routing tasks between agents
  from different frameworks.

## MCP

Ancora agents can act as both MCP **clients** (consuming tools from an MCP
server) and MCP **servers** (exposing their own tools to external models).

### As an MCP client

Configure an MCP server URL in the provider settings. Ancora fetches the
tool list on startup and makes them available to the model.

### As an MCP server

Enable the MCP server in Ancora's config. External models (including Claude
via the Anthropic API) can then call your Ancora tools over HTTP.

## A2A

A2A defines a JSON-RPC protocol for agent task delegation. An Ancora agent
can:

- **Accept** an A2A task from another framework (LangGraph, CrewAI, etc.)
  and run it as a standard `AgentSpec` run.
- **Delegate** a subtask to an external A2A-compatible agent.

### A2A task lifecycle

```
TaskSent -> TaskAccepted -> (events) -> TaskCompleted | TaskFailed
```

## Transport

Both MCP and A2A communicate over HTTP(S) + Server-Sent Events (SSE). Ancora
wraps this in the `ancora-grpc` crate for internal use and exposes a REST
adapter for external consumers.

## See also

- [Tools and Effects](tools-and-effects.md)
- [Orchestration Graph](orchestration-graph.md)
