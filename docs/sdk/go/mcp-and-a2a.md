# MCP and A2A (Go)

## Using MCP tools

Configure an MCP server URL and Ancora fetches the tool list at startup:

```go
spec := ancora.NewAgentSpec("llama3", "Use weather tool.")
spec.McpServers = []string{"http://localhost:8090/mcp"}
// Tools from the MCP server are automatically available to the model
```

## Exposing tools via MCP

Start the built-in MCP server to expose your `GoToolRegistry` tools:

```go
registry := ancora.NewGoToolRegistry()
registry.Register("get_weather", weatherSpec, weatherFn)

server := ancora.NewMcpServer(registry, ":8090")
go server.Serve()
```

## A2A: accepting external tasks

```go
a2a := ancora.NewA2AServer(agent, ":8091")
go a2a.Serve()
// External agents can now delegate tasks to your Ancora agent
```

## A2A: delegating to an external agent

```go
delegate := ancora.NewA2AClient("http://external-agent:8091")
result, _ := delegate.Delegate(ctx, ancora.A2ATask{
    Instructions: "Summarise this document.",
    Context:      docText,
})
```

## See also

- [MCP and A2A concept](../../concepts/mcp-and-a2a.md)
- [Tools](tools.md)
