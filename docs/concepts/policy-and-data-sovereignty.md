# Policy and Data Sovereignty

Ancora's policy layer enforces **data residency**, **provider allow-lists**,
and **tool effect limits** at the runtime level -- not in the application code.

## Policy rules

A policy rule is evaluated before each agent step. It can:

- **Block** a model call if the model's datacenter region is not in the
  allowed list.
- **Audit** a tool call and write a compliance record to the journal.
- **Redact** sensitive fields from messages before they leave the perimeter.
- **Rate-limit** specific effect classes (e.g. at most 3 `WRITE` tool calls
  per run).

## Data residency

Residency rules constrain which providers a run may use:

```json
{
  "allow_regions": ["us-east-1", "eu-west-1"],
  "deny_providers": ["openai-gpt4-global"]
}
```

An agent configured with this policy will fail fast if it tries to call a
provider outside the allowed regions.

## Governance integration

Ancora stores policy evaluation outcomes as `ActivityRecorded` events with
`activity_kind = "policy_check"`. These records are replayable and auditable.

## GDPR and HIPAA considerations

- Keep data local with an Ollama or GGUF provider.
- Use `SqliteStore` or `PostgresStore` inside your perimeter.
- Enable audit logging to a tamper-evident journal store.

## See also

- [Providers and Local-First](providers-and-local-first.md)
- [Governance guide](../guides/governance.md)
