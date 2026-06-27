# Concurrency (Python)

## Thread safety

`Runtime` is thread-safe. A single `Runtime` instance can be shared across
threads. Each call to `rt.run()` acquires an independent native handle.

```python
import threading
from ancora import Runtime, AgentSpec

rt = Runtime()
spec = AgentSpec(model="llama3", instructions="Answer.")

def worker(prompt: str):
    result = rt.run(spec, prompt)
    print(threading.current_thread().name, "->", result.output[:40])

threads = [threading.Thread(target=worker, args=(f"Question {i}",)) for i in range(4)]
for t in threads:
    t.start()
for t in threads:
    t.join()
```

## ThreadPoolExecutor

```python
from concurrent.futures import ThreadPoolExecutor
from ancora import Runtime, AgentSpec

rt = Runtime()
spec = AgentSpec(model="llama3", instructions="Summarise.")

prompts = ["Text A", "Text B", "Text C", "Text D"]

with ThreadPoolExecutor(max_workers=4) as pool:
    results = list(pool.map(lambda p: rt.run(spec, p).output, prompts))

for r in results:
    print(r[:60])
```

## asyncio event loop

Async methods (`rt.run_async`, `rt.stream_async`) run on the active event
loop. Do not mix sync and async calls on the same `RunHandle`:

```python
# CORRECT
result = await rt.run_async(spec, prompt)

# INCORRECT -- do not call sync run inside a running event loop
# result = rt.run(spec, prompt)   # blocks the event loop
```

## GIL considerations

The native Ancora engine releases the GIL during inference. CPU-bound Python
code can run concurrently with an in-flight agent run.

## Process-level isolation

Each worker process needs its own `Runtime`. The native library is not
fork-safe; create `Runtime` after forking:

```python
from multiprocessing import Pool
from ancora import Runtime, AgentSpec

def process_worker(prompt: str) -> str:
    rt = Runtime()   # created after fork
    spec = AgentSpec(model="llama3", instructions="Answer.")
    return rt.run(spec, prompt).output

with Pool(processes=4) as pool:
    results = pool.map(process_worker, ["Q1", "Q2", "Q3", "Q4"])
```

## See also

- [Async patterns](async-patterns.md)
- [Deployment](deployment.md)
