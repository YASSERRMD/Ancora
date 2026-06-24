# Ancora Conformance Suite

Every Ancora binding (Python, Go, TypeScript, etc.) must pass these scenarios against the Rust core. Each scenario is identified by a stable string ID and a human-readable description. The canonical list is returned by `ancora_core::conformance::all_scenarios()`.

## Scenarios

### `single-agent`

**Description:** A single agent node runs to completion without error.

**What to verify:**
- Build a `Graph` containing exactly one `NodeKind::Agent` node.
- Call `Graph::validate()` and assert `Ok`.
- The node count must equal 1.

### `multi-agent-verifier`

**Description:** An agent node and a verifier node with an explicit dependency.

**What to verify:**
- Build a `Graph` with two `NodeKind::Agent` nodes: `agent` and `verifier`.
- Add an `Edge` from `agent` to `verifier`.
- Call `Graph::validate()` and assert `Ok`.
- Assert `edges[0].from == "agent"` and `edges[0].to == "verifier"`.

### `human-in-loop`

**Description:** A run suspends awaiting human approval and then resumes correctly.

**What to verify:**
- Construct a `SuspendedRun` with non-empty `run_id`, `node_id`, and `pending_input`.
- Serialize to JSON with `SuspendedRun::to_json()` and assert `Ok`.
- Deserialize with `SuspendedRun::from_json()` and assert round-trip equality of all fields.
- Repeat with `deadline_ms: Some(...)` and assert the value is preserved.

### `crash-and-recover`

**Description:** A run journal persists across restart and replays deterministically.

**What to verify:**
- Append two `JournalEvent` values to a `MemoryStore` under the same `run_id`.
- Call `store.read(run_id)` and assert `len == 2`.
- Clone the store; append an event via the original; read via the clone and assert the event is visible (shared backing state).

## Adding a new scenario

1. Add a `pub const MY_SCENARIO: ConformanceScenario` in `ancora-core/src/conformance.rs`.
2. Push the reference into `all_scenarios()`.
3. Add a `#[test]` in the same module that exercises the scenario.
4. Document it in this file.
