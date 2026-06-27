# Glossary

**Activity key**
A unique string identifying a single tool execution within a run. Used for
idempotency: if replay encounters an existing activity key, it returns the
cached result without re-running the tool.

**AgentSpec**
The configuration record for a single agent: model ID, instructions, tools,
max steps, and optional output schema.

**Checkpoint**
An opaque blob saved at the end of each graph node. On resume, replay starts
from the most recent checkpoint rather than the full journal beginning.

**Effect class**
A label on a tool indicating how it should be replayed: `READ` (safe to
repeat), `WRITE` (cache result; do not repeat), or `IDEMPOTENT_WRITE` (safe to
repeat but treat as cached if key exists).

**GraphSpec**
A directed acyclic graph of agent nodes and edges. The graph executor runs
nodes in topological order.

**Journal**
An append-only log of all events for a run. Provides durability and enables
deterministic replay.

**MemoryStore**
An in-process, non-durable `JournalStore` implementation used for tests and
single-process runs.

**Provider**
An inference backend: Ollama, Anthropic, OpenAI, etc. Providers are
configured at startup and can be swapped without changing agent code.

**Run**
A single execution of an `AgentSpec`. Has a unique UUID, a lifecycle status,
and an event journal.

**RunHandle**
An SDK object representing an in-flight or completed run. Used to stream
events, resume after suspension, and retrieve cost data.

**ToolSpec**
The configuration record for a tool: name, description, input/output schemas,
and effect class.

**Working memory**
In-process or SQLite-backed storage scoped to the current run. Holds
retrieved passages, intermediate results, and checkpoints.
