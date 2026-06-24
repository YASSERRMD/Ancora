# Ancora Python SDK Examples

Runnable examples for the Ancora Python SDK. All examples run fully offline
against the in-process runtime -- no external LLM endpoint required.

## Run an example

```bash
cd sdk/python
pip install -e ".[test]"
python -m examples.single_agent
```

## Examples

| Example | Description |
|---------|-------------|
| `single_agent` | Start a run and print each event kind |
| `rag_memory` | Tool-based retrieval with memory persistence |
| `mcp_tool_use` | Multiple tools registered and dispatched |
| `streaming` | Stream tokens as they arrive |
| `human_in_loop` | Pause a run and resume with a decision |
| `multi_agent` | Run two agents concurrently |
| `conformance_runner` | Run the full conformance suite and print results |
| `tool_composition` | Tools that call other tools |
