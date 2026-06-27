# MCP and A2A (.NET)

## Using MCP tools

```csharp
var spec = new AgentSpec
{
    Model = "llama3",
    Instructions = "Use the weather tool.",
    McpServers = new List<string> { "http://localhost:8090/mcp" },
};
```

## Exposing tools via MCP

```csharp
using Ancora;

var registry = new ToolRegistry();
registry.Register(new ToolSpec
{
    Name = "get_weather",
    Description = "Get the weather for a city.",
    // ...
    Fn = args => $"{args["city"]!.GetString()}: 22 C"
});

var server = new McpServer(registry, port: 8090);
server.StartBackground();
```

## A2A: accepting external tasks

```csharp
var a2a = new A2AServer(agent, port: 8091);
a2a.StartBackground();
// External agents can now delegate tasks to this Ancora agent
```

## A2A: delegating to an external agent

```csharp
var delegate_ = new A2AClient("http://external-agent:8091");
var result = await delegate_.DelegateAsync(new A2ATask
{
    Instructions = "Summarise this document.",
    Context = docText,
});
Console.WriteLine(result.Output);
```

## See also

- [MCP and A2A concept](../../concepts/mcp-and-a2a.md)
- [Tools](tools.md)
