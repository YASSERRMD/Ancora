# Changelog: Advanced Capabilities (Phases 161-200)

## v0.7.0 (Phase 200)

### Added

- `ancora-preset`: capability presets for research-assistant, coding-agent,
  customer-support, data-analysis, and government-compliant (air-gapped, locked).
- `ancora-advbench`: benchmark harness with 10 capability benchmarks and
  regression-gated thresholds.
- `docs/preset/`: preset catalog, customizing guide, government compliance notes.
- `docs/advbench/`: methodology, results, cost-quality tables, regression thresholds.

## v0.6.x (Phases 161-199)

### Added

- `ancora-orchestrate`: planning, fan-out, task graph, spawn tracking.
- `ancora-memcon`: episodic memory, consolidation, salience scoring, forgetting.
- `ancora-toolsynth`: tool synthesis, SynthCache, SandboxRunner, audit.
- `ancora-skills`: skill JIT loading, SkillRegistry, crew, journal.
- `ancora-lh`: long-horizon checkpoints, scheduled wakeup, lifecycle management.
- `ancora-coord`: coordination journal, contract-net, auction, deadlock detection.
- `ancora-guard`: injection guardrail, PII detection, allow/deny, journal, policy.
- `ancora-reason`: reasoning steps, citations, fact-check, contradiction, abstention.
- `ancora-ageval`: 7-metric behavior evaluation (planning, reflection, routing,
  coordination, guardrails, reasoning, memory).
- `ancora-adv-integration`: cross-crate integration patterns and examples.
- `ancora-advdet`: determinism test suite (71 tests).
- `ancora-redteam`: adversarial red-team harness with 5 scenario categories.
- `ancora-advpar`: cross-language parity (94 tests, Go example validated).
- `docs/advcap/`: 10 capability documentation files.
- `docs/redteam/`: red-team harness guide and scenario authoring.
- `docs/advpar/`: advanced feature parity matrix and per-language notes.

### Changed

- Workspace now includes 15 advanced-capability crates.

### Fixed

- All crates compile without warnings at the test binary level.
