# Trace Propagation Guide

ancora-trace propagates trace context across agent-to-agent (a2a) boundaries
using the W3C Trace Context `traceparent` header format.

## Header format

```
traceparent: 00-<trace-id>-<parent-span-id>-<flags>
```

- `00` - version (always `00`)
- `<trace-id>` - the `TraceId` value
- `<parent-span-id>` - the calling span's `SpanId`
- `<flags>` - two hex digits; bit 0 = sampled (`01`) or unsampled (`00`)

## Injecting context

Before making an a2a call, create a `TraceContext` from the current span
and inject it into a `HeaderCarrier`:

```rust
let ctx = TraceContext::sampled(current_trace_id, current_span_id);
let mut carrier = HeaderCarrier::new();
carrier.inject(&ctx);
// Pass carrier headers with the outgoing request.
```

## Extracting context

On the receiving agent, extract the context and use it to create a child span:

```rust
let received = carrier.extract().unwrap();
let child_span = Span::child(
    "child-agent-run",
    received.parent_span_id,
    received.trace_id,
    start_ns,
);
```

## Sampling

Set `flags = 0x01` (sampled) when the trace should be exported.
Unsampled spans (`flags = 0x00`) are propagated but not collected by the exporter.

## Multi-hop traces

Each hop re-injects its own span as the new parent but preserves the original
`trace_id`. This ensures all spans across all agents share one trace root,
enabling a single unified trace view in your observability backend.
