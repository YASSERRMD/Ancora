# Cost and OTEL Tracing Example

Demonstrates wrapping an agent run in an in-process `Span` to record event
counts, estimated token usage, and wall-clock duration -- matching what an
OTEL exporter would consume.

## What it tests

- `Span::new` creates a span with `name` and empty attributes
- `set_attribute` stores key-value pairs readable after `end_ms()`
- `end_ms()` returns elapsed milliseconds and sets `duration_ms`
- `TokenEstimator::estimate_tokens` returns at least 1 for any input
- Multiple child spans can accumulate independently into a root summary

## Pattern

```rust
use ancora_examples::{Span, TokenEstimator};

let mut root = Span::new("agent.run");

// ... collect events from agent loop ...
let total_tokens: usize = token_texts.iter()
    .map(|t| TokenEstimator::estimate_tokens(t))
    .sum();

root.set_attribute("event.count", events.len().to_string());
root.set_attribute("tokens.estimated", total_tokens.to_string());
let ms = root.end_ms();
assert!(ms < 5_000);
```

## Offline

`Span` and `TokenEstimator` are fully in-process.
