# MCP and A2A (Python)

## Using MCP tools

Configure an MCP server URL and Ancora fetches the tool list at startup:

```python
spec = AgentSpec(
    model="llama3",
    instructions="Use the weather tool.",
    mcp_servers=["http://localhost:8090/mcp"],
)
```

Tools from the MCP server are automatically available to the model.

## Exposing tools via MCP

Start the built-in MCP server to expose your `ToolRegistry` to other agents:

```python
from ancora import McpServer, ToolRegistry

registry = ToolRegistry()

@registry.tool(description="Get the weather for a city.")
def get_weather(city: str) -> str:
    return f"{city}: 22 C"

server = McpServer(registry, port=8090)
server.serve_background()
```

## A2A: accepting external tasks

```python
from ancora import A2AServer

a2a = A2AServer(agent=rt, port=8091)
a2a.serve_background()
# External agents can now delegate tasks to this Ancora agent
```

## A2A: delegating to an external agent

```python
from ancora import A2AClient

delegate = A2AClient("http://external-agent:8091")
result = delegate.delegate("Summarise this document.", context=doc_text)
print(result.output)
```

## Async A2A

```python
import asyncio
from ancora import A2AClient

async def main():
    delegate = A2AClient("http://external-agent:8091")
    result = await delegate.delegate_async("Summarise this document.", context=doc_text)
    print(result.output)

asyncio.run(main())
```

## See also

- [MCP and A2A concept](../../concepts/mcp-and-a2a.md)
- [Tools](tools.md)
