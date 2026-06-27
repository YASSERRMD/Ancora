# Error Handling (Python)

## Exception hierarchy

```
AncorError
├── NativeError          # native library error (CFFI)
├── RunFailedError       # run terminated with an error event
├── PolicyViolationError # policy rule blocked the run
├── TimeoutError         # run exceeded max_runtime_seconds
└── JournalError         # journal read/write failure
```

## Catching run errors

```python
from ancora import Runtime, AgentSpec, RunFailedError

rt = Runtime()
spec = AgentSpec(model="llama3", instructions="Answer.")

try:
    result = rt.run(spec, "What is 2+2?")
    print(result.output)
except RunFailedError as e:
    print(f"Run failed: {e.message} (run_id={e.run_id})")
```

## Catching policy violations

```python
from ancora import PolicyViolationError

try:
    result = rt.run(spec, prompt)
except PolicyViolationError as e:
    print(f"Policy blocked: {e}")
```

## Retry on transient errors

```python
import time
from ancora import RunFailedError

def run_with_retry(rt, spec, prompt, max_attempts=3):
    for attempt in range(max_attempts):
        try:
            return rt.run(spec, prompt)
        except RunFailedError as e:
            if attempt == max_attempts - 1 or not e.is_transient:
                raise
            time.sleep(2 ** attempt)
```

## Async error handling

```python
import asyncio
from ancora import RunFailedError

async def safe_run(rt, spec, prompt):
    try:
        async for event in rt.stream_async(spec, prompt):
            if event.type == "token":
                print(event.token, end="", flush=True)
    except RunFailedError as e:
        print(f"\nRun failed: {e.message}")

asyncio.run(safe_run(rt, spec, "What is 2+2?"))
```

## Inspecting failed runs from the journal

```python
from ancora import SqliteStore

store = SqliteStore("/var/lib/myapp/journal.db")
run = store.get_run("run-abc-123")
print(run.status)      # "FAILED"
print(run.error)       # error message
print(run.last_event)  # the event that triggered failure
```

## See also

- [Troubleshooting](troubleshooting.md)
- [Durability](durability.md)
