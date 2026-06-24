# Ancora Agent and Tool Contracts

Source of truth: `crates/ancora-proto/proto/contracts.proto`

## EffectClass

Classifies the observable side effect a tool may produce.

| Value | Name | Description |
|-------|------|-------------|
| 0 | `EFFECT_UNSPECIFIED` | Default; never use explicitly |
| 1 | `EFFECT_PURE` | No side effects; safe to replay at any time |
| 2 | `EFFECT_READ` | Reads external state; no modification |
| 3 | `EFFECT_WRITE` | Modifies external state; requires idempotency key |

## ToolSpec

```proto
message ToolSpec {
  string      name                      = 1; // stable machine id
  string      description               = 2; // shown to model
  string      input_schema_json         = 3; // JSON Schema string
  string      output_schema_json        = 4; // JSON Schema string
  EffectClass effect_class              = 5;
  string      idempotency_key_template  = 6; // for WRITE tools only
}
```

### Idempotency key template

For `EFFECT_WRITE` tools, `idempotency_key_template` is a string with
optional variable substitutions:

| Variable | Substituted with |
|----------|-----------------|
| `{run_id}` | The stable run identifier |
| `{node_id}` | The stable node identifier within the graph |
| `{seq}` | The monotonic sequence number of this activity |

Example: `"{run_id}-send_email-{seq}"` produces a key like
`"01J5X-send_email-3"`. The key is written to the journal before the
tool is invoked; a duplicate key rejects the invocation.

## AgentSpec

```proto
message AgentSpec {
  string       name               = 1;
  string       model_id           = 2;
  string       instructions       = 3;  // system prompt
  string       output_schema_json = 4;  // empty = free-form text
  repeated ToolSpec tools         = 5;
  uint32       max_steps          = 6;
  RetryPolicy  model_retry        = 7;
  string       model_params_json  = 8;  // e.g. {"temperature":0.7}
}
```

### max_steps

The agent loop terminates with an error if the number of reason-act
iterations reaches `max_steps` without producing a final output. A
value of 0 means use the runtime default (currently 25).

### output_schema_json

When non-empty, the agent's final output is validated against this JSON
Schema. If validation fails, the runtime requests a repair turn (up to a
bounded number of attempts, configured separately).

## RetryPolicy

```proto
message RetryPolicy {
  uint32 max_attempts        = 1;
  uint32 initial_backoff_ms  = 2;
  uint32 max_backoff_ms      = 3;
  float  jitter              = 4;  // fraction in [0,1]
}
```

Backoff is exponential: delay = min(initial * 2^attempt, max) * (1 +
jitter * random[0,1]).
