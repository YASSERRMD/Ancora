# Responding to Drift

This document describes how to triage and respond to drift alerts from
ancora-drift.

## Alert severities

| Severity | Meaning | Recommended response |
|---|---|---|
| `INFO` | Marginal signal, within noise band | Log and monitor trend |
| `WARNING` | Meaningful shift detected | Investigate within 4 hours |
| `CRITICAL` | Severe drift or cost spike | Page on-call immediately |

## Runbook: Input drift

1. Check whether a new workload type was onboarded recently.
2. Compare the current input length histogram against the reference.
3. If the workload is legitimate, rebuild the reference distribution.
4. If inputs look malformed, check upstream routing.

## Runbook: Output drift

1. Check model version - was the provider updated?
2. Compare output lengths and bigram richness against reference.
3. If the model changed, trigger a full eval run before accepting the new
   distribution as the new reference.

## Runbook: Cost drift

1. Identify which provider and model are driving the cost increase.
2. Check whether a higher-tier model was accidentally selected.
3. Review routing rules and fallback chains.
4. If expected (new feature launch), update the cost budget and reference.

## Runbook: Tool drift

1. Identify which tool's frequency changed.
2. Check whether tool routing rules were modified.
3. Replay a sample of recent requests against a shadow environment.

## Rebuilding the reference

After a confirmed legitimate workload shift, rebuild the reference:

```rust
let mut builder = ReferenceBuilder::new();
for trace in new_baseline_traces {
    builder.add(&trace.input, &trace.output,
                trace.cost_micros, trace.latency_ms,
                &trace.tools_called, &trace.provider);
}
let new_reference = builder.build().unwrap();
// Store new_reference in your configuration store.
```
