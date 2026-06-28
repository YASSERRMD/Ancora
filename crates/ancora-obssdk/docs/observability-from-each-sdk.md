# Observability from Each SDK

ancora-obssdk provides trace, cost, and eval helpers for every Ancora SDK language integration.

## Go

Use `GoTraceAccessor` and `GoCostAccessor` to record spans and costs:

```go
// Go SDK (via FFI or gRPC binding)
acc := obssdk.NewGoTraceAccessor("my-trace-id")
acc.RecordSpan("s1", "http.handler", startNs, endNs)
```

## Python

Use `PyTraceAccessor` and `PyCostAccessor`:

```python
from ancora_obssdk import PyTraceAccessor, PyCostAccessor

acc = PyTraceAccessor("my-trace-id")
acc.record_span("s1", "llm.invoke", start_ns, end_ns)
summary = acc.to_dict()
```

## TypeScript

```typescript
const acc = new TsTraceAccessor("my-trace-id");
acc.recordSpan("s1", "api.fetch", startNs, endNs);
const json = acc.toJsonStrings();
```

## .NET

```csharp
var acc = new DotnetTraceAccessor("my-trace-id");
acc.StartActivity("s1", "MVC.Action", startNs, endNs);
string traceparent = acc.Traceparent();
```

## Java

```java
JavaTraceAccessor acc = new JavaTraceAccessor("my-trace-id");
acc.startSpan("s1", "Servlet.doGet", startNs, endNs);
List<String> names = acc.spanNames();
```

## Rust

```rust
let mut acc = RsTraceAccessor::new("my-trace-id");
acc.record_span("s1", "agent.run", 0, 5000);
let dur = acc.root_duration_ns();
```

## Cost Tracking

Each language SDK includes a corresponding cost accessor:

- `GoCostAccessor`, `PyCostAccessor`, `TsCostAccessor`
- `DotnetCostAccessor`, `JavaCostAccessor`, `RsCostAccessor`

All expose `total_tokens()` and `records()`.

## Eval Helpers

Use `EvalRunner` with `EvalCriteria` to validate traces:

```rust
let criteria = EvalCriteria::new("my-eval")
    .with_min_spans(2)
    .with_required_span("agent.run")
    .with_max_duration_ns(10_000_000);

let result = EvalRunner::new().evaluate(&trace, &criteria);
assert!(result.passed);
```

Use `run_multilang_eval` to evaluate traces from all language SDKs at once.
