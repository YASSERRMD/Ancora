# Interoperability Limits

ancora-fwadapt translates static definitions and dispatches in-process
messages. The following limitations apply.

## No runtime execution of external frameworks

The crate does NOT embed Python, the LangChain runtime, CrewAI, or any other
framework. Adapters produce Rust values (structs, enums) that callers use to
drive Ancora's own execution engine.

## Network I/O is out of scope

`AncoraToolAdapter`, `A2ADispatcher`, and `HandoffBridge` all call in-process
function pointers. The caller must implement HTTP/gRPC transport if the peer
agent runs in a separate process.

## LangGraph: DAGs only

`map_langgraph_to_stages` uses a topological sort. Graphs with cycles
return `GraphMappingError::CycleDetected`. Conditional branching (edges with
predicates) is not modelled in the current type system; flatten conditionals
into stages before mapping.

## CrewAI: no memory or tool state

`AncoraCrewPlan` captures roles and task assignments. CrewAI memory (short-term,
long-term, entity) must be mapped separately using the `ancora-agentmem` crate.

## Semantic Kernel: function-level granularity only

SK plugin imports resolve at function level. SK's planner and step-wise
execution are not modelled; they must be re-implemented using Ancora's
orchestration primitives.

## A2A correlation IDs

`build_message` generates a deterministic correlation ID from sender and
recipient IDs. Production deployments should inject a UUID-based ID to avoid
collisions across concurrent sessions.
