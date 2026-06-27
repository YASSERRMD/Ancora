# Human-in-Loop Example

Demonstrates persisting `HumanDecisionRequested` and `HumanDecisionReceived`
events in `MemoryStore` and reading them back in order.

## What it tests

- `HumanDecisionRequestedEvent` stores `prompt` and `options`
- `HumanDecisionReceivedEvent` stores the `decision` string
- Events are readable in seq order after being appended
- Decision JSON can carry arbitrary key-value payload

## Pattern

```rust
store.append(run_id, make_event(run_id, 1,
    Event::HumanDecisionRequested(HumanDecisionRequestedEvent {
        prompt: "Approve this action?".to_string(),
        options: vec!["yes".to_string(), "no".to_string()],
        timeout_at_ns: 0,
    })
)).unwrap();

store.append(run_id, make_event(run_id, 2,
    Event::HumanDecisionReceived(HumanDecisionReceivedEvent {
        decision: r#"{"action":"approve"}"#.to_string(),
    })
)).unwrap();
```

## Offline

All event types live in `ancora-proto`. No network calls.
