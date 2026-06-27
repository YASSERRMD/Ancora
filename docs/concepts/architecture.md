# Architecture Overview

Ancora is organized as a layered Rust workspace with language SDKs built on
top of a shared C FFI boundary.

## Layer diagram

```
┌────────────────────────────────────────────┐
│   Language SDKs (Go / Python / TS / .NET / Java / Rust) │
├────────────────────────────────────────────┤
│          ancora-ffi  (C ABI)               │
├────────────────────────────────────────────┤
│  ancora-core      │  ancora-inference       │
│  (engine, graph,  │  (provider adapters)    │
│   journal, replay)│                         │
├────────────────────────────────────────────┤
│  ancora-memory    │  ancora-tools           │
│  (vector stores)  │  (built-in tools)       │
├────────────────────────────────────────────┤
│  ancora-proto  (protobuf wire types)        │
└────────────────────────────────────────────┘
```

## Core crates

| Crate | Role |
|-------|------|
| `ancora-proto` | Canonical protobuf definitions for all wire types |
| `ancora-core` | Agent loop, graph executor, journal, replay, idempotency |
| `ancora-inference` | Provider adapters: Anthropic, OpenAI, Ollama, ... |
| `ancora-memory` | Vector store integrations: LanceDB, pgvector, Milvus, ... |
| `ancora-tools` | Built-in tools: web search, code execution, file I/O |
| `ancora-policy` | Data residency and governance rule evaluation |
| `ancora-observability` | OTEL span builder, cost tracking |
| `ancora-ffi` | C ABI exports consumed by all language SDKs |

## Data flow for a single run

1. SDK calls `ancora_run_start` with a serialized `AgentSpec`.
2. `ancora-core` creates a `Run` in `Pending` state and writes a
   `RunStarted` event to the journal.
3. The agent loop calls `ancora-inference` to complete a model turn.
4. Tool calls are recorded via `ActivityRecorded` before dispatch.
5. On completion, `RunCompleted` is written to the journal.
6. On crash, replay re-executes from the journal; recorded activities
   return cached results without re-running side effects.

## See also

- [Durability and Replay](durability-and-replay.md)
- [Agents](agents.md)
