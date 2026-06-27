# Quickstart (Python)

Run a minimal agent locally in under five minutes.

## Prerequisites

- `pip install ancora`
- Ollama running: `ollama serve && ollama pull llama3`

## Minimal example

```python
from ancora import Runtime, AgentSpec

rt = Runtime()

spec = AgentSpec(
    model="llama3",
    instructions="Answer concisely.",
)

result = rt.run(spec, "What is a durable agent?")
print(result.output)
```

## Run it

```bash
python quickstart.py
```

## What happened

1. `Runtime()` initialises the native Ancora engine via CFFI.
2. `AgentSpec` declares the model and instructions.
3. `rt.run(spec, prompt)` submits the prompt, waits for completion, and
   returns the final output.

## Next steps

- [Defining tools](tools.md) -- give the agent Python callables
- [Structured output](structured-output.md) -- parse the response as a Pydantic model
- [Streaming](streaming.md) -- process tokens as they arrive
