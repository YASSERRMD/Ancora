# Trace Model Reference

The `ancora-trace` crate provides a rich trace tree that mirrors the run
structure of an Ancora agent pipeline.

## Core types

| Type | Description |
|------|-------------|
| `TraceId` | Globally-unique identifier for a trace (hex string) |
| `SpanId` | Unique identifier for a span within a trace |
| `Span` | A single unit of work with attributes and events |
| `Trace` | The root container owning all spans for one run |

## Span kinds

- `Internal` - agent-internal computation
- `Client` - outbound call (tool or LLM)
- `Server` - inbound request to this agent
- `Producer` / `Consumer` - a2a handoff sides

## Building a trace

```rust
let root = Span::root("agent-run", start_ns);
let trace = Trace::new(root.trace_id.clone(), root);
```

Spans are linked by `parent_id`. The `Trace::add_span` method validates
that the parent exists before inserting.

## Replay

A trace can be rebuilt deterministically from a flat list of journal events
using `journal::spans_from_journal` followed by `trace::build_trace_from_spans`.
The resulting span ordering is sorted by `start_ns`, guaranteeing identical
output for identical inputs.
