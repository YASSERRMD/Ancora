# Ancora Architecture Overview

## Core design principles

Ancora is a **local-first, language-agnostic** agentic framework. The Rust
core is authoritative; every other language binding wraps it through a stable
C ABI. This means:

- One engine implementation, zero drift between language bindings.
- All persistent state lives in the journal; a process restart resumes exactly
  where it left off.
- Agents can run fully offline against a local inference endpoint; no cloud
  dependency is required.

## Layer diagram

```
┌──────────────────────────────────────────────────────────────┐
│  User code: agent graphs, tool implementations               │
├──────────────────────────────────────────────────────────────┤
│  Language bindings                                           │
│  Go  |  Python  |  TypeScript  |  .NET  |  Java             │
├──────────────────────────────────────────────────────────────┤
│  ancora-ffi  (C ABI, no unsafe in callers)                   │
├──────────────────────────────────────────────────────────────┤
│  ancora-core: engine, journal, replay, graph validation      │
│  ancora-tools: registry, MCP server, LangChain adapter       │
│  ancora-policy: egress control, PII redaction, audit         │
│  ancora-grpc: A2A protocol, identity, task lifecycle         │
│  ancora-proto: Protobuf definitions                          │
└──────────────────────────────────────────────────────────────┘
         |                         |
  local storage              remote agents
  (SQLite / memory)         (A2A over HTTP)
```

## Crate responsibilities

| Crate | Responsibility |
|-------|----------------|
| `ancora-core` | Engine loop, journal, graph, replay, property tests |
| `ancora-proto` | Protobuf schemas (journal events, agent specs) |
| `ancora-ffi` | C ABI shim; every language binding links this |
| `ancora-tools` | `Tool` trait, `ToolRegistry`, MCP server, LangChain adapter |
| `ancora-policy` | Declarative governance (egress, PII, audit, air-gapped) |
| `ancora-grpc` | A2A agent card, Ed25519 identity, task lifecycle, handoff |

## Journal and replay

Every run is recorded as a sequence of `JournalEvent` messages. The journal
is append-only; events are never modified or deleted.

On restart the engine calls `replay_events(run_id, &events)` to rebuild
the `ReplayState`, which contains the set of activities already completed
(keyed by `activity_key`). Any activity whose key appears in the replay
state is skipped rather than re-executed, guaranteeing exactly-once
semantics for side-effecting work.

The journal backend is swappable. `MemoryStore` (for tests) and the SQLite
backend are the shipped implementations. The `JournalStore` trait is public;
any durable store can be plugged in.

## Tool execution

Tools are registered in a `ToolRegistry`. When an agent calls a tool the
registry:

1. Looks up the tool by exact name (injection-safe; no shell or eval).
2. Validates the input `serde_json::Value` against the tool's JSON Schema
   (`validate_input`), rejecting missing required fields.
3. Calls `Tool::call`.

The MCP server (`McpServer`) exposes the registry over HTTP JSON-RPC 2.0.
It supports optional static bearer-token authentication and serves
`tools/list` and `tools/call`.

## A2A protocol

Agents discover each other via agent cards at `/.well-known/agent.json`.
Cards are JSON objects optionally signed with an Ed25519 keypair. The
`A2aClient` fetches and optionally verifies cards; `perform_handoff` handles
cross-language task delegation.

## Policy engine

Every egress call should pass through `check_endpoint(policy, url)` before
being executed. Policies support:

- **Air-gapped mode** (`Policy::air_gapped()`): blocks all outbound calls.
- **Allowlist mode** (`Policy::allow_endpoint(prefix)`): permits only
  approved URL prefixes.
- **PII redaction**: pattern-based detection before journal commit.
- **Audit logging**: marks actions that require an audit trail.

## Cross-language conformance

The `test/xlang/` directory contains a canonical scenario manifest
(`scenarios.json`) and a journal-comparison script (`compare.py`). Each
language binding runs the same scenarios and emits journal events; the
comparison script strips model content and asserts structural equality.
CI runs all conformance suites in parallel.
