# structured_output

Derives a JSON Schema from Pydantic models and injects the schema into the
agent system prompt so the agent produces output matching the expected shape.
Runs fully offline.

## Run

```bash
cd sdk/python
python -m examples.structured_output
```

## What it shows

- Defining `BaseModel` subclasses with `Field` descriptions
- Using `model_json_schema()` to derive a JSON Schema string
- Embedding the schema in the agent system prompt
- Running two agents with different output shapes (analysis and classification)
