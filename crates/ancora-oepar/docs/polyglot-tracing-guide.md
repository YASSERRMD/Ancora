# Polyglot Tracing Guide

This guide explains how to stitch together traces that span multiple language SDKs in Ancora's agent-to-agent (a2a) architecture.

## Overview

A polyglot trace is a single distributed trace whose spans originate from agents written in different languages. The trace is unified by a shared `trace_id` propagated via W3C TraceContext headers.

## Propagation

When a Rust orchestrator calls a Python tool agent:

1. The Rust span sets `traceparent` in the outgoing HTTP header.
2. The Python agent extracts `traceparent` and creates a child span with the same `trace_id`.
3. Both spans are exported to the same OTLP endpoint and stitched by the backend.

## A2A Context

```
A2AContext {
    trace_id:       "trace-poly-001"
    parent_span_id: "span-rust-1"
    baggage: {
        "session_id": "sess-42"
        "user_id":    "usr-7"
    }
}
```

Baggage is propagated via the W3C Baggage header and is available to all downstream agents regardless of language.

## Validation

The `PolyglotTrace::validate_parent_links` method checks that every `parent_span_id` references a known `span_id` within the assembled trace. Run this check after stitching to catch propagation bugs early.

## Reference Implementation

See `crates/ancora-oepar/src/polyglot.rs` for the reference stitching implementation and `src/tests/test_polyglot_stitching.rs` for end-to-end tests covering all six languages.

## CI Checks

The CI suite runs `cargo test -p ancora-oepar` which includes polyglot stitching tests. All tests must pass offline with no network calls.
