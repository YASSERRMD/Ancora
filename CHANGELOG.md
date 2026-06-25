# Changelog

All notable changes to Ancora are documented here.

Format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).
Versioning follows [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

### Added

- CI publish dry-run for crates.io, PyPI, npm, NuGet, Maven.
- Docs CI: lychee link checker and markdownlint on every PR touching docs/.

---

## [0.1.0] - 2026-06-25

Initial release of the Ancora agentic framework.

### Added

**Core engine**
- `ancora-core`: graph definition, journal, replay, and engine loop.
- Append-only `JournalStore` trait with `MemoryStore` and SQLite implementations.
- `replay_events` with exactly-once activity semantics.
- `Graph::validate` (O(V+E), cycle detection, reachability).
- Criterion benchmarks for graph validation, replay, and journal throughput.
- Proptest property-based tests for replay determinism and idempotency.
- Chaos tests: crash-and-resume, faulty-store injection, duplicate-effect detection.

**FFI layer**
- `ancora-ffi`: C ABI shim (`ancora_version`, `ancora_runtime_new`,
  `ancora_free_runtime`).
- CI ABI stability check (`abi-check.yml`).

**Tool layer**
- `ancora-tools`: `Tool` trait, `ToolRegistry` with JSON Schema validation.
- `McpServer`: HTTP JSON-RPC 2.0 server with bearer-token auth.
- LangChain adapter (`from_langchain`).
- Security: tool-poisoning sanitization tests, auth-rejection tests.

**Policy engine**
- `ancora-policy`: declarative governance with air-gapped mode, data-residency
  allowlists, PII redaction, audit-required flag.
- `PolicyError::EgressBlocked` for air-gapped violations (replaces misleading
  empty-allowlist semantics).

**A2A protocol**
- `ancora-grpc`: `AgentCard`, Ed25519 identity, `A2aClient`, task lifecycle.
- Agent handoff (`perform_handoff`) with optional signed-identity enforcement.

**Language bindings**
- Go SDK (`sdk/go`).
- Python SDK (`sdk/python`, maturin-based).
- TypeScript SDK (`sdk/ts`).
- .NET SDK (`sdk/dotnet`).
- Java SDK (`sdk/java`).

**Cross-language conformance**
- Canonical scenario manifest (`test/xlang/scenarios.json`).
- Journal mask and structural-equality assertion (`journal_mask`).
- CI conformance suite (`xlang-conformance.yml`).

**Documentation**
- Architecture overview.
- Per-language quickstarts (Rust, Go, Python, TypeScript, .NET, Java).
- Guides: orchestration, memory, durability, observability, governance.
- Security threat model (T1-T6).
- Benchmark methodology and approximate results.
- Migration guides from LangGraph, CrewAI, Microsoft Agent Framework.
- Governance and sovereignty checklist.

### Changed

- Nothing (initial release).

### Fixed

- Nothing (initial release).

---

[Unreleased]: https://github.com/YASSERRMD/Ancora/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/YASSERRMD/Ancora/releases/tag/v0.1.0
