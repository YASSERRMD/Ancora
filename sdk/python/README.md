# Ancora Python SDK

Python bindings for the Ancora agent runtime via PyO3 and maturin.

## Install

Pre-built wheels are available on PyPI for Linux, macOS, and Windows on
CPython 3.9 through 3.12:

```bash
pip install ancora
```

## Development install

Build from source using maturin:

```bash
pip install maturin
cd sdk/python
maturin develop --release
```

## Supported platforms

| Platform | Python |
|----------|--------|
| Linux (x86_64, aarch64) | 3.9, 3.10, 3.11, 3.12 |
| macOS (x86_64, arm64) | 3.9, 3.10, 3.11, 3.12 |
| Windows (x86_64) | 3.9, 3.10, 3.11, 3.12 |

## Usage

```python
import ancora

rt = ancora.Runtime()
print(ancora.version())
rt.free()
```

Or with a context manager:

```python
import ancora

with ancora.Runtime() as rt:
    print(rt)
```

## Tool decorator

Register Python functions as tools with automatic JSON Schema generation:

```python
from ancora.tools import tool, ToolRegistry
from ancora.models import EffectClass

@tool(effect_class=EffectClass.READ)
def search(query: str, limit: int = 10) -> str:
    """Search the web for a query."""
    return f"results for {query}"

registry = ToolRegistry()
registry.register(search)

# Add the tool spec to an agent
import ancora
spec = ancora.AgentSpec(name="agent", model_id="llama3", tools=[search.spec])

# Dispatch a tool call from a "tool_call" event
result = registry.dispatch("search", '{"query": "hello", "limit": 5}')
```

## Async agent runs

Start runs and iterate over events asynchronously:

```python
import asyncio
import ancora

async def main():
    rt = ancora.Runtime()
    spec = ancora.AgentSpec(name="my-agent", model_id="llama3", instructions="do stuff")
    agent = ancora.Agent(rt, spec)

    run = await agent.run()
    print("run started:", run.run_id)

    async for event in run:
        print(json.loads(event)["kind"])

    rt.free()

asyncio.run(main())
```

## Streaming tokens

Stream tokens as they arrive using the `stream_tokens` async generator:

```python
import asyncio
import ancora

async def main():
    rt = ancora.Runtime()
    spec = ancora.AgentSpec(name="a", model_id="llama3")
    agent = ancora.Agent(rt, spec)
    run = await agent.run()

    tokens = []
    async for token in run.stream_tokens():
        tokens.append(token)
    print("".join(tokens))

    rt.free()

asyncio.run(main())
```

To receive all raw event bytes (including non-token events), use `stream_events()`:

```python
async for raw in run.stream_events():
    event = ancora.StreamEvent.from_bytes(raw)
    print(event.kind)
```

## Memory store

Persist agent state across steps using `MemoryStore`:

```python
from ancora import MemoryStore, Agent, AgentSpec, Runtime

rt = Runtime()
mem = MemoryStore()
mem.write("user", "Alice")

spec = AgentSpec(name="a", model_id="llama3")
agent = Agent(rt, spec, memory=mem)

print(agent.memory.read("user"))   # "Alice"
agent.memory.write("step", 1)
agent.memory.delete("step")
agent.memory.clear()
```

## Conformance suite

Verify that your runtime produces the expected event sequences:

```python
import asyncio
import ancora
from ancora import ConformanceSuite, register_builtin_scenarios

async def main():
    suite = ConformanceSuite()
    register_builtin_scenarios(suite)

    rt = ancora.Runtime()
    results = await suite.run_all(rt)
    rt.free()

    for name, passed in results.items():
        print(f"{name}: {'PASS' if passed else 'FAIL'}")

asyncio.run(main())
```

Register custom scenarios with `suite.register(name, async_fn)`.

## Pydantic models and wire format

Build agent specs with Pydantic validation and serialize to JSON wire bytes:

```python
from ancora import AgentSpecBuilder, ToolSpecBuilder, EffectClass, to_wire_bytes, from_wire_bytes

tool = ToolSpecBuilder().with_name("search").with_effect_class(EffectClass.READ).build()

spec = (
    AgentSpecBuilder()
    .with_name("my-agent")
    .with_model_id("llama3")
    .with_instructions("research and summarize")
    .with_tool(tool)
    .build()
)

wire = to_wire_bytes(spec)
recovered = from_wire_bytes(wire)
print(recovered.name)
```
