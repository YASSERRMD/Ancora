# Semantic Conventions Used

ancora-trace follows the OpenTelemetry GenAI semantic conventions
(https://opentelemetry.io/docs/specs/semconv/gen-ai/) and adds
Ancora-specific extensions under the `ancora.*` namespace.

## GenAI attributes

| Key | Type | Description |
|-----|------|-------------|
| `gen_ai.system` | string | Provider name (e.g. `anthropic`) |
| `gen_ai.request.model` | string | Model requested |
| `gen_ai.response.model` | string | Model that actually responded |
| `gen_ai.request.max_tokens` | int | Token budget |
| `gen_ai.request.temperature` | float | Sampling temperature |
| `gen_ai.usage.input_tokens` | int | Prompt tokens consumed |
| `gen_ai.usage.output_tokens` | int | Completion tokens generated |
| `gen_ai.prompt` | string | Prompt content (subject to redaction) |
| `gen_ai.completion` | string | Completion content (subject to redaction) |
| `gen_ai.operation.name` | string | Operation type (e.g. `chat`) |
| `gen_ai.response.finish_reasons` | string | Finish reason(s) |

## Ancora extensions

| Key | Type | Description |
|-----|------|-------------|
| `ancora.tenant.id` | string | Multi-tenant identifier |
| `ancora.run.id` | string | Unique run identifier |
| `ancora.agent.id` | string | Agent identity |
| `ancora.tool.name` | string | Tool invoked |
| `ancora.cost.usd` | float | Estimated USD cost for this span |
| `ancora.retry.count` | int | Number of retries attempted |
| `ancora.error.kind` | string | Structured error category |

## Redaction

Attributes with keys that match `gen_ai.prompt*` or `gen_ai.completion*`
prefixes are subject to the active `RedactPolicy`. The default policy is
`passthrough`; production deployments should use `redact_content` or
`truncate_content`.
