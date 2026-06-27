# Agents

An **agent** in Ancora is a model-driven control loop that:

1. Receives an initial prompt (the `instructions` field of `AgentSpec`).
2. Calls the configured model.
3. Dispatches any tool calls the model requests.
4. Repeats until the model produces a final text response or `max_steps` is
   reached.

## AgentSpec

Every agent is configured via an `AgentSpec`. The fields are shared across
all language SDKs:

| Field | Type | Description |
|-------|------|-------------|
| `name` | string | Human-readable label for logs and traces |
| `model_id` | string | Model identifier (e.g. `claude-3-5-haiku-20241022`) |
| `instructions` | string | System prompt |
| `tools` | `[]ToolSpec` | Tools the model may call |
| `max_steps` | integer | Maximum agent loop iterations |
| `model_retry` | `RetryPolicy` | Retry config for transient model errors |
| `output_schema_json` | string | JSON Schema for structured output |
| `model_params_json` | string | Provider-specific params (temperature, etc.) |

## Run lifecycle

```
Pending -> Running -> Completed
                   -> Cancelled
                   -> Failed
```

A `Run` is created for each `AgentSpec` invocation. Its `id` is a UUID v4
generated once and stable across journal replay.

## Multiple agents

You can run multiple agents concurrently. Each gets its own `Run` and its
own event stream. Agents communicate by passing run IDs and reading each
other's outputs from the journal.

See [Orchestration Graph](orchestration-graph.md) for multi-agent routing.
