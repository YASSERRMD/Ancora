# Choosing SLM vs Larger Model

Use this guide when deciding whether to route a task to a small local model or a larger (API-backed or on-prem) model.

## Decision Framework

```
Can the task be solved with a small model reliably?
├── Yes → Use SLM with appropriate reliability patterns.
└── No ──→ Is it acceptable to fall back to a large model occasionally?
           ├── Yes → Use SLM-first with escalation (ancora-slm).
           └── No ──→ Route directly to the large model.
```

## When SLMs Excel

SLMs perform comparably to large models on:

- **Short, well-defined extraction tasks** (entity extraction, slot filling).
- **Classification with few classes** (sentiment, intent detection with <= 10 labels).
- **Format conversion** (JSON ↔ text, template filling).
- **Simple summarisation** (< 500-word documents into 1-2 sentences).
- **Tool-call routing** (choosing which tool to invoke from a small catalogue).

## When SLMs Struggle

Route to a larger model when:

- **Multi-step reasoning** is required and decomposition is impractical.
- **Uncommon knowledge** is needed (niche domain, recent events).
- **Long documents** exceed the SLM context window (typically 4K-8K tokens).
- **High-stakes correctness** is required (medical, legal, financial).
- **Complex code generation** (> 50 lines with non-trivial logic).
- **Low-shot generalisation** is needed without fine-tuning data.

## Latency vs Accuracy Trade-off

| Scenario | Recommended approach |
|---|---|
| < 100 ms latency required | SLM only; no escalation |
| < 500 ms latency required | SLM-first with 1-2 retries; escalate to medium model |
| Latency-insensitive | SLM-first with full escalation to large model |
| Batch offline processing | SLM with aggressive retries; escalate on failure |

## Cost Model

Assuming a local SLM costs ~$0.00 per token and an API-backed large model costs ~$0.01/1K tokens:

- **SLM handles task, no escalation**: $0.00
- **SLM retries 2x, then succeeds**: $0.00
- **SLM fails, escalates to large model** (e.g., 500 tokens): ~$0.005

Set `max_slm_attempts` conservatively: 2-3 retries are usually sufficient. More retries delay the inevitable escalation without improving accuracy significantly.

## Monitoring Signals

Track these metrics to tune your routing policy:

| Metric | Action if high |
|---|---|
| `slm_escalation_rate` > 20% | Improve SLM prompts or add more few-shot examples |
| `slm_first_attempt_pass_rate` < 70% | Invest in prompt format selection |
| `tool_repair_rate` > 30% | Consider fine-tuning or switching SLM |
| `verifier_fail_rate` > 15% | Add step decomposition or increase retry budget |

## Implementation Checklist

When adopting SLM-first orchestration:

- [ ] Profile the SLM on your task distribution before going to production.
- [ ] Set `PromptStyle` to match the model's training format.
- [ ] Enable `ValidJsonVerifier` for all JSON-output tasks.
- [ ] Add `RequiredKeysVerifier` for structured-output tasks.
- [ ] Seed `FewShotLibrary` with 3-5 representative examples per task type.
- [ ] Configure `EscalationPolicy` with a realistic `max_slm_attempts`.
- [ ] Record real model calls into `ReplayStore` for regression testing.
- [ ] Monitor escalation rate weekly and tune accordingly.
