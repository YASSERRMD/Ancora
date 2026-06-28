# A/B Testing Guide

ancora-ab provides controlled experiment infrastructure for the Ancora agent framework.
Use it to compare prompt strategies, model configurations, or any agent behaviour
change against a baseline before rolling out to all traffic.

## Quick start

1. Define an experiment with named variants and traffic weights that sum to 1.0.
2. Assign each incoming request to a variant using `assignment::assign`.
3. Log the exposure so you know who saw what.
4. Collect outcome metrics after the agent responds.
5. Run `analysis::welch_t_test` to check statistical significance.
6. Optionally attach guardrails to halt harmful variants automatically.
7. Call `lifecycle::LifecycleManager::conclude` with the winning variant.
8. Generate an `ExperimentReport` and store it for audit.

## Modules

| Module | Purpose |
|---|---|
| `experiment` | Define variants, weights, and the primary metric |
| `assignment` | Deterministic, hash-based traffic splitting |
| `exposure` | Log when a subject sees a variant |
| `outcome` | Collect per-variant metric observations |
| `analysis` | Welch's t-test and p-value computation |
| `guardrail` | Safety checks that halt experiments automatically |
| `lifecycle` | State machine: Pending -> Running -> Concluded/Stopped |
| `report` | Structured result summary |

## Key properties

- **Deterministic**: the same subject key always maps to the same variant.
- **No network required**: all logic is in-process with no external dependencies.
- **No panics in library code**: all fallible operations return `Result` or `Option`.
