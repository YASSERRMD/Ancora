# Tools and Effects

A **tool** is a function the model can call during the agent loop. Ancora
tracks the effect class of every tool to enforce idempotency and policy.

## ToolSpec

| Field | Description |
|-------|-------------|
| `name` | Unique name used in the model's tool-call request |
| `description` | Natural-language description shown to the model |
| `input_schema_json` | JSON Schema describing the expected input |
| `output_schema_json` | JSON Schema describing the return value |
| `effect_class` | `READ`, `WRITE`, or `IDEMPOTENT_WRITE` |
| `idempotency_key_template` | Template for deduplication keys |

## Effect classes

| Class | Meaning | Replayed? |
|-------|---------|-----------|
| `READ` | Safe to repeat (no side effects) | Re-executed on replay |
| `WRITE` | Has side effects; must not repeat | Cached in journal |
| `IDEMPOTENT_WRITE` | Side effects are safe to repeat | Re-executed on replay |

## Idempotency keys

For `WRITE` tools Ancora records a key before executing the tool. If replay
encounters the same key, it returns the cached result without re-running the
tool. The key template can reference run, node, and step identifiers:

```
{run_id}-{node_id}-{seq}
```

## Built-in tools

`ancora-tools` ships a set of ready-to-use tools:

- `web_search` -- keyword and semantic web search
- `code_exec` -- sandboxed code execution
- `file_read` / `file_write` -- local file I/O

See the [Tools guide](../guides/adding-a-provider.md) for custom tools.
