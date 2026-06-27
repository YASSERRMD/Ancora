# Observability and Cost (Python)

Track token usage and export spans to an OpenTelemetry collector.

## In-process cost tracking

Estimate cost without external instrumentation:

```python
def estimate_tokens(text: str) -> int:
    if not text:
        return 1
    return max(1, -(-len(text) // 4))   # ceiling division

total_tokens = 0
for event in rt.stream(spec, prompt):
    if event.type == "token":
        total_tokens += estimate_tokens(event.token)

print(f"Estimated tokens: {total_tokens}")
```

## Collecting per-run cost from events

```python
from ancora import Runtime, AgentSpec

rt = Runtime()
spec = AgentSpec(model="llama3", instructions="Answer.")

events = rt.run(spec, "What is 2+2?").events()
for event in events:
    if event.type == "completed":
        print(f"Input tokens:  {event.usage.input_tokens}")
        print(f"Output tokens: {event.usage.output_tokens}")
```

## OpenTelemetry export

```bash
pip install opentelemetry-sdk opentelemetry-exporter-otlp
```

```python
from opentelemetry import trace
from opentelemetry.sdk.trace import TracerProvider
from opentelemetry.sdk.trace.export import BatchSpanProcessor
from opentelemetry.exporter.otlp.proto.grpc.trace_exporter import OTLPSpanExporter

provider = TracerProvider()
provider.add_span_processor(BatchSpanProcessor(OTLPSpanExporter(endpoint="http://localhost:4317")))
trace.set_tracer_provider(provider)
tracer = trace.get_tracer("ancora")

with tracer.start_as_current_span("agent-run") as span:
    result = rt.run(spec, "What is 2+2?")
    span.set_attribute("ancora.model", spec.model)
    span.set_attribute("ancora.output_tokens", result.usage.output_tokens)
```

## See also

- [Durability](durability.md)
- [Policy](policy.md)
