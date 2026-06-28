# Known Limitations

## Planning

- `PlanningMetric::score` is O(n x m) where n = expected and m = actual step count.
  For step counts above 500, consider pre-sorting or using a hash-based approach.
- The `fan_out` helper uses sequential task IDs; it is not safe to merge task graphs
  from two separate `fan_out` calls without de-duplicating task IDs first.

## Memory consolidation

- `ConsolidationJob` takes full ownership of the `SummarizationPolicy` and
  `ForgettingPolicy` at construction; callers must reconstruct the job to change
  policies between runs.
- The token budget (`TokenBudget`) estimates are heuristic; they do not reflect
  actual LLM token counts.

## Long-horizon

- `CheckpointCadence` is tick-based, not time-based.  If the agent pauses between
  ticks, the cadence does not account for wall-clock elapsed time.
- `ScheduledWakeup.should_fire` uses monotonic u64 ticks; it will wrap at u64::MAX
  (practical limit: ~584 years at 1 tick/ns).

## Tool synthesis

- `SandboxRunner::execute` returns `Result<serde_json::Value, SynthError>`;
  sandbox isolation is logical (no OS-level sandboxing in the current implementation).

## Skills

- `JitLoader` is a stub; actual JIT loading from disk or network is not implemented.
  All skills must be pre-loaded via `SkillRegistry::load`.

## Red-team

- `InjectionInputGuardrail` checks only 4 hard-coded patterns.  New attack vectors
  require adding patterns to the constant array.
- `ScenarioDataset` has no persistence; scenarios must be re-constructed on each run.

## Cross-language parity

- Go, Python, TypeScript, .NET, and Java ports validate canonical values only; they
  do not have full implementations of all advanced capabilities.

## Presets

- Preset `locked` flag is encoded in the system prompt but is not enforced by the
  orchestrator in this release.  Enforcement must be added by the caller.
