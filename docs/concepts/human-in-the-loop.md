# Human-in-the-Loop

Ancora supports pausing an agent run and waiting for a human decision before
continuing. This is useful for approval flows, content moderation, and
high-stakes tool calls.

## How it works

1. The agent (or a policy rule) emits `HumanDecisionRequested` with a prompt
   and a list of options.
2. The run transitions to a suspended state. No further model calls are made.
3. A human reviews the prompt (via a UI, webhook, or CLI) and submits a
   decision.
4. The decision is delivered as `HumanDecisionReceived`.
5. The agent resumes from the suspension point.

## Resume payload

The decision payload is a JSON string. It can carry arbitrary structured data:

```json
{ "action": "approve", "reason": "reviewed by alice@example.com" }
```

## Timeouts

`HumanDecisionRequestedEvent.timeout_at_ns` sets a deadline. If no decision
arrives before the timeout, the run transitions to `Failed` with an `ERROR`
event.

## Integration patterns

- **Webhook**: send `HumanDecisionRequested` to a webhook endpoint; receive
  the decision via `RunHandle.resume(decision_json)`.
- **Slack / email**: post a rich message; capture the reply and call `resume`.
- **CLI**: print the prompt to stdout; read the decision from stdin.

## See also

- [Orchestration Graph](orchestration-graph.md)
- [Agents](agents.md)
