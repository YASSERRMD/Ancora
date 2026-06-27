# What is Ancora?

Ancora is a **local-first, privacy-respecting multi-agent runtime**. It lets
you define, orchestrate, and operate AI agents that run wherever your data
lives -- on your laptop, in a private datacenter, or in an air-gapped
facility -- without any data leaving your infrastructure by default.

## Key principles

**Local-first.** Ancora ships with first-class support for local inference via
Ollama, llama.cpp, and other GGUF-compatible runtimes. You can build and test
entire multi-agent systems without an internet connection.

**Durable by default.** Every agent run is backed by an append-only event
journal. If a process crashes mid-run, Ancora replays the journal and resumes
from the last safe checkpoint -- no data loss, no double-sends.

**Deterministic activity recording.** Side-effecting operations (tool calls,
emails, database writes) are recorded with idempotency keys before they
execute. Replaying the same journal always produces the same outcome.

**Multi-language.** The same Rust engine powers SDKs for Go, Python,
TypeScript, .NET, Java, and Rust. All SDKs share one wire protocol.

**Privacy by design.** Data stays in your perimeter. Remote providers
(Anthropic, OpenAI, etc.) are supported but opt-in. The default model is local.

## What Ancora is not

- A hosted SaaS platform (though you can self-host)
- A no-code agent builder
- A replacement for a full workflow engine (like Temporal)

Ancora focuses on the **agent loop**: model calls, tool dispatch, memory
retrieval, graph routing, and durable replay.

## Next steps

- [Architecture overview](architecture.md) -- how the pieces fit together
- [Quickstarts](../quickstarts/index.md) -- running your first agent
