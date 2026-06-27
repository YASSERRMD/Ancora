# Async Patterns (Python)

Ancora's Python SDK supports both sync and async usage. All sync methods have
`_async` variants. This page covers advanced asyncio integration patterns.

## Concurrent runs with `asyncio.gather`

```python
import asyncio
from ancora import Runtime, AgentSpec

rt = Runtime()
spec = AgentSpec(model="llama3", instructions="Summarise the text.")

texts = ["Text A...", "Text B...", "Text C..."]

async def summarise(text: str) -> str:
    result = await rt.run_async(spec, text)
    return result.output

async def main():
    summaries = await asyncio.gather(*[summarise(t) for t in texts])
    for summary in summaries:
        print(summary)

asyncio.run(main())
```

## Streaming with `asyncio.Queue`

```python
import asyncio
from ancora import Runtime, AgentSpec

async def stream_to_queue(rt, spec, prompt, queue: asyncio.Queue):
    async for event in rt.stream_async(spec, prompt):
        await queue.put(event)
    await queue.put(None)  # sentinel

async def consume_queue(queue: asyncio.Queue):
    while True:
        event = await queue.get()
        if event is None:
            break
        if event.type == "token":
            print(event.token, end="", flush=True)

async def main():
    rt = Runtime()
    spec = AgentSpec(model="llama3", instructions="Tell a story.")
    queue: asyncio.Queue = asyncio.Queue()
    await asyncio.gather(
        stream_to_queue(rt, spec, "Once upon a time...", queue),
        consume_queue(queue),
    )

asyncio.run(main())
```

## Fan-out verifier with `asyncio`

```python
import asyncio

async def verify(rt, primary_output: str) -> bool:
    spec = AgentSpec(
        model="llama3",
        instructions="Is the following answer correct? Reply YES or NO.",
    )
    result = await rt.run_async(spec, primary_output)
    return result.output.strip().upper().startswith("YES")

async def run_with_consensus(rt, primary_spec, prompt):
    primary_result = await rt.run_async(primary_spec, prompt)
    verdicts = await asyncio.gather(*[verify(rt, primary_result.output) for _ in range(3)])
    if sum(verdicts) >= 2:
        return primary_result.output
    return None

asyncio.run(run_with_consensus(rt, spec, "What is 2+2?"))
```

## Using with FastAPI

```python
from fastapi import FastAPI
from ancora import Runtime, AgentSpec

app = FastAPI()
rt = Runtime()
spec = AgentSpec(model="llama3", instructions="Answer questions.")

@app.post("/ask")
async def ask(prompt: str) -> dict:
    result = await rt.run_async(spec, prompt)
    return {"output": result.output}
```

## See also

- [Streaming](streaming.md)
- [Human-in-the-loop](human-in-the-loop.md)
