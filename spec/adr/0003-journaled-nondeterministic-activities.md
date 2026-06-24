# ADR-0003: Journaled non-deterministic activities for replay

Date: 2026-06-24
Status: Accepted

## Context

Agent runs involve inherently non-deterministic operations: model calls
return different tokens each invocation, tool calls may have side effects,
and wall-clock time varies. If a process crashes mid-run, restarting from
scratch wastes compute, money, and time. The system must be able to resume
a run from any point without re-executing completed work or duplicating
side effects.

## Decision

Every non-deterministic activity (model call, tool call with side effects,
human decision, external API call) is wrapped in a record-or-replay
abstraction. On first execution the activity runs and its result is
appended to the durable journal with a stable idempotency key. On any
subsequent execution of the same code path (crash recovery, debugging,
eval replay) the wrapper detects the journaled result and returns it
directly without re-executing the activity. All other code is expected to
be deterministic given the same journaled inputs.

## Consequences

- Crash recovery is cheap: resume from the last journaled event and
  re-execute only the remaining steps.
- Eval and debugging workflows can replay a run offline against recorded
  model responses with no live model calls.
- Idempotency keys must be stable across restarts; they are derived from
  the run id, node id, and sequence number.
- Any code path that branches on a non-journaled value (current time,
  random number, external state) is a latent non-determinism bug. The
  replay engine detects divergence by comparing the expected next event
  type against the journaled sequence.
- The journal is append-only; corrections are new events, never rewrites.

## Alternatives considered

- Checkpoint-and-restart (serialize full state): simpler to implement
  but the checkpoint can become large and does not provide a human-
  readable audit trail of what happened.
- Idempotent re-execution without a journal: requires all activities to
  be truly idempotent, which is impossible for model calls that produce
  different outputs each time.
