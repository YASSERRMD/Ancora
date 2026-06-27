# Testing Your Agents (Python)

All Ancora Python tests run offline by default. No live API keys or network
connections are required.

## Skip when native library is absent

```python
import pytest
import ancora

def skip_if_no_native():
    try:
        ancora.Runtime()
    except OSError:
        pytest.skip("native library not available")

def test_single_agent():
    skip_if_no_native()
    rt = ancora.Runtime()
    spec = ancora.AgentSpec("llama3", "Answer.")
    result = rt.run(spec, "What is 2+2?")
    assert result.output
```

## Testing tool logic

Tool functions are plain Python callables. Test them in isolation:

```python
def test_get_weather():
    result = get_weather("Cairo")
    assert "Cairo" in result
    assert "22 C" in result
```

## Testing schema generation

```python
from ancora import ToolSpec
from pydantic import BaseModel

class Summary(BaseModel):
    headline: str
    body: str

def test_output_schema():
    schema = ToolSpec.schema_from_model(Summary)
    assert schema["type"] == "object"
    assert "headline" in schema["properties"]
```

## Testing with an in-memory journal

```python
from ancora import Runtime, MemoryStore, StoringTransport

def test_durability():
    store = MemoryStore()
    rt = Runtime(transport=StoringTransport(store))
    spec = ancora.AgentSpec("llama3", "Answer.")
    rt.run(spec, "ping", run_id="test-run-1")
    assert store.has_run("test-run-1")
```

## Running with pytest

```bash
cd sdk/python
pytest tests/ -v
```

## See also

- [Quickstart](quickstart.md)
- [Durability](durability.md)
