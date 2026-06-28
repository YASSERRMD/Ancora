# Observability and Evaluation Readiness Checklist

Use this checklist before promoting an agent to production.

## Observability checks

- obs-001: Distributed tracing is enabled and exporting spans
- obs-002: Metrics collection is configured
- obs-003: Log aggregation is connected
- obs-004: Cost analytics are recording token usage
- obs-005: Drift monitor is calibrated with a baseline
- obs-006: Safety monitor is active with critical alert routing
- obs-007: PII redaction policies are applied to telemetry

## Evaluation checks

- eval-001: Eval platform is connected to the agent under test
- eval-002: At least one dataset with ground truth is loaded
- eval-003: Regression gates are defined with threshold values
- eval-004: A/B experiment tracking is configured
- eval-005: Human feedback queue is draining to a store
- eval-006: Continuous eval schedule is set for OnDeploy
- eval-007: Dev studio is accessible for local replay
- eval-008: Observability integrations are tested end-to-end
