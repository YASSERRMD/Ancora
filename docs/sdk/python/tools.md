# Defining Tools (Python)

Tools are Python callables registered with a `ToolRegistry` and passed to an
`AgentSpec`. The agent calls them during a run; their return values are
injected back into the model context.

## Decorator syntax

```python
from ancora import Runtime, AgentSpec, ToolRegistry

registry = ToolRegistry()

@registry.tool(description="Return the current weather for a city.")
def get_weather(city: str) -> str:
    return f"{city}: 22 C, sunny"

spec = AgentSpec(
    model="llama3",
    instructions="Use get_weather to answer weather questions.",
    tools=registry,
)

rt = Runtime()
result = rt.run(spec, "What is the weather in Cairo?")
print(result.output)
```

## Explicit registration

```python
from ancora import ToolSpec

def get_weather(city: str) -> str:
    return f"{city}: 22 C, sunny"

tool_spec = ToolSpec.from_callable(
    name="get_weather",
    fn=get_weather,
    description="Return the current weather for a city.",
)
spec = AgentSpec(model="llama3", instructions="...", tools=[tool_spec])
```

## Async tools

```python
import httpx

@registry.tool(description="Fetch the content of a URL.")
async def fetch_url(url: str) -> str:
    async with httpx.AsyncClient() as client:
        resp = await client.get(url)
        return resp.text[:500]
```

Ancora automatically awaits async tool functions.

## Tool effect classes

Mark write tools to enable policy enforcement:

```python
from ancora import EffectClass

@registry.tool(description="Append a row to the database.", effect=EffectClass.WRITE)
def append_row(table: str, data: dict) -> str:
    ...
```

## See also

- [Structured output](structured-output.md)
- [Policy](policy.md)
