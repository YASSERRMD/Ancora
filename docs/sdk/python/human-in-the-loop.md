# Human-in-the-Loop (Python)

Suspend a run at a tool call boundary and resume it with human-supplied input.

## Pattern

```python
from ancora import Runtime, AgentSpec, ToolSpec, RunStatus

rt = Runtime()

def request_approval(action: str) -> str:
    raise StopIteration(f"Approve this action? {action}")

spec = AgentSpec(
    model="llama3",
    instructions="Before modifying any file, call request_approval.",
    tools=[ToolSpec.from_callable("request_approval", request_approval,
                                  description="Ask a human to approve an action.")],
)

handle = rt.start(spec, "Delete the temp directory.")

try:
    handle.run_until_pause()
except StopIteration as e:
    print("Human prompt:", e)
    approval = input("Type YES to approve: ")
    handle.resume(approval)

result = handle.collect()
print(result.output)
```

## Async human-in-the-loop

```python
import asyncio

async def run_with_approval():
    handle = rt.start(spec, "Delete the temp directory.")
    status = await handle.run_until_pause_async()

    if status == RunStatus.PAUSED:
        approval = await asyncio.get_event_loop().run_in_executor(
            None, input, "Approve? (YES/NO): "
        )
        await handle.resume_async(approval)

    result = await handle.collect_async()
    print(result.output)

asyncio.run(run_with_approval())
```

## Resume with binary payload

```python
handle.resume_bytes(b'{"approved": true, "reason": "looks safe"}')
```

## Timeout

```python
import threading

def timeout_resume():
    threading.Timer(30.0, lambda: handle.resume("TIMEOUT")).start()
```

## See also

- [Streaming](streaming.md)
- [Durability](durability.md)
