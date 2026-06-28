# Release Notes -- Ancora 0.6.0

**Release date:** 2026-06-28

## Highlights

Ancora 0.6.0 ships the complete offline test program: 100+ test files covering
determinism, security, policy, reliability, chaos, load, coverage gating,
documentation audit, example parity, and performance benchmarks.

All six SDK languages (Rust, Go, Python, TypeScript, .NET, Java) are at parity,
with each producing identical journal events and structured outputs across
cross-language scenarios.

## What is new

### Complete test program

- **Determinism suite** (19 det_ files): 14 formally stated guarantees, all proved offline.
- **Reliability and chaos** (19 files): circuit breaker, deadline, rate limit, network fault,
  clock skew, OOM guard, disk quota, journal corruption, and partial write scenarios.
- **Security and policy** (19 files): prompt injection, PII guard, tool allowlist, key rotation,
  audit log, RBAC, input size limits, TLS config, data residency, GDPR erasure, offline mode.
- **Coverage gates** (19 files): no module, event type, SDK language, vector backend,
  security property, or doc page can be silently dropped.
- **Documentation audit** (19 files): completeness, cross-references, code samples, FAQ,
  freshness, and troubleshooting entries all verified programmatically.
- **Example parity** (19 files): single-agent, verifier, HIL, RAG, MCP, streaming, cost,
  OTel, local provider, structured output, policy, durability, A2A, error handling,
  Chinese providers, multi-agent, edge deployment, journal format, and summary.
- **Benchmark suite** (18 files): all hot paths measured with named budgets, reproducible
  on stock CI hardware.

### Chinese provider support

All ten Chinese providers (Qwen, GLM, DeepSeek, Kimi, MiniMax, StepFun, ERNIE, Hunyuan,
Doubao, MiMo) run from recorded fixtures with correct cost accounting and residency tags.

### Vector store coverage

All 11 vector store backends (inmemory, sqlite, pgvector, qdrant, weaviate, milvus, lancedb,
chroma, pinecone, vespa, redis) pass the conformance suite.

### A2A and MCP interop

A2A envelopes with protocol=a2a/1.0 are validated across all 6 language pairs.
MCP tools are exercised from both rust-server and go-server hosts.

## Breaking changes

None. This is an additive release.

## Upgrade guide

See docs/release/upgrade-0.5-to-0.6.md.
