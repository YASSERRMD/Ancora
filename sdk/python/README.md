# Ancora Python SDK

Python bindings for the Ancora agent runtime via PyO3 and maturin.

## Install

```bash
pip install maturin
maturin develop
```

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
