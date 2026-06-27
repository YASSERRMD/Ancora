# Async Streaming (Python)

Iterate over run events as they arrive instead of waiting for the full
response.

## Sync iteration

```python
from ancora import Runtime, AgentSpec

rt = Runtime()
spec = AgentSpec(model="llama3", instructions="Tell a short story.")

for event in rt.stream(spec, "Once upon a time..."):
    if event.type == "token":
        print(event.token, end="", flush=True)
print()
```

## Async iteration

```python
import asyncio
from ancora import Runtime, AgentSpec

async def main():
    rt = Runtime()
    spec = AgentSpec(model="llama3", instructions="Tell a short story.")

    async for event in rt.stream_async(spec, "Once upon a time..."):
        if event.type == "token":
            print(event.token, end="", flush=True)
    print()

asyncio.run(main())
```

## Accumulating tokens

```python
tokens = []
async for event in rt.stream_async(spec, prompt):
    if event.type == "token":
        tokens.append(event.token)

full_text = "".join(tokens)
```

## Event types

| `event.type` | Description |
|-------------|-------------|
| `"started"` | Run has begun |
| `"token"` | One model output token |
| `"tool_call"` | Agent called a tool |
| `"completed"` | Run finished; `event.output` has the final text |
| `"resumed"` | Run resumed after a pause |
| `"error"` | Run failed; `event.message` has the error |

## See also

- [Human-in-the-loop](human-in-the-loop.md)
- [Quickstart](quickstart.md)
