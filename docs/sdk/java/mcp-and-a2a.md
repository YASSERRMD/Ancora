# MCP and A2A (Java)

## Using MCP tools

```java
var spec = new AgentSpec.Builder()
    .model("llama3")
    .instructions("Use the weather tool.")
    .mcpServers(List.of("http://localhost:8090/mcp"))
    .build();
```

## Exposing tools via MCP

```java
import io.ancora.*;

var registry = new ToolRegistry();
registry.register(weatherTool);
registry.register(priceTool);

var server = new McpServer(registry, 8090);
server.startBackground();
```

## A2A: accepting external tasks

```java
var a2a = new A2AServer(agent, 8091);
a2a.startBackground();
// External agents can now delegate tasks to this Ancora agent
```

## A2A: delegating to an external agent

```java
var delegate = new A2AClient("http://external-agent:8091");
var result = delegate.delegate(new A2ATask()
    .withInstructions("Summarise this document.")
    .withContext(docText));
System.out.println(result.output());
```

## Async delegation

```java
import java.util.concurrent.CompletableFuture;

CompletableFuture<A2AResult> future = delegate.delegateAsync(task);
future.thenAccept(r -> System.out.println(r.output()));
```

## See also

- [MCP and A2A concept](../../concepts/mcp-and-a2a.md)
- [Tools](tools.md)
