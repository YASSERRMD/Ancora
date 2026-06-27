# Type Checking (Python)

The Ancora Python SDK ships a `py.typed` marker and inline type annotations.
Full mypy and pyright support is included.

## mypy

```bash
pip install mypy
mypy agent.py
```

All public API types are annotated:

```python
from ancora import Runtime, AgentSpec, RunResult, RunHandle

rt: Runtime = Runtime()
spec: AgentSpec = AgentSpec(model="llama3", instructions="Answer.")
result: RunResult = rt.run(spec, "What is 2+2?")
print(result.output)   # str
```

## pyright

```bash
pip install pyright
pyright agent.py
```

## Pydantic structured output type safety

```python
from pydantic import BaseModel
from ancora import Runtime, AgentSpec

class Summary(BaseModel):
    headline: str
    body: str

rt = Runtime()
spec = AgentSpec(model="llama3", instructions="Summarise.", output_schema=Summary)
result = rt.run(spec, "Ancora is a durable agent runtime.")
summary: Summary = result.parse(Summary)   # pyright/mypy infers Summary
print(summary.headline)                    # str -- no Any leakage
```

## Generic `parse` signature

`RunResult.parse` is typed as:

```python
from typing import TypeVar, Type
from pydantic import BaseModel

T = TypeVar("T", bound=BaseModel)

def parse(self, model: Type[T]) -> T: ...
```

Passing the model class returns a fully-typed instance.

## See also

- [Structured output](structured-output.md)
- [API reference](api-reference.md)
