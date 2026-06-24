# Ancora

Ancora is a language-agnostic, local-first agentic framework. The core
engine is written in Rust and exposed to all major languages (Go, Python,
TypeScript, .NET, Java) through a stable C ABI and an optional gRPC sidecar.

## Local-first positioning

Ancora defaults to a local OpenAI-compatible endpoint (Ollama, llama.cpp,
vLLM). Cloud providers are additive and never assumed. The full test suite
runs offline.

## Architecture overview

- `ancora-core`: agent loop, graph executor, durable journal, replay, retry
- `ancora-proto`: protobuf wire spec and JSON mirror
- `ancora-inference`: model adapters (OpenAI-compatible, local-first)
- `ancora-memory`: working, episodic, semantic, and archival memory tiers
- `ancora-tools`: typed tool contract, MCP client, permission broker
- `ancora-observability`: OpenTelemetry GenAI emission
- `ancora-policy`: residency, PII, and governance descriptors
- `ancora-ffi`: C ABI surface (cdylib and staticlib), cbindgen header
- `ancora-grpc`: optional gRPC sidecar over the same core
- `ancora-cli`: reference single-binary runtime and dev studio backend

Language bindings live under `bindings/` (Go, Python, TypeScript, .NET,
Java). Runnable examples are in `examples/`. Cross-language conformance
tests are in `tests/`.

## Status

Active development. See the phase plan in `spec/` for the roadmap.

## License

Apache-2.0. See [LICENSE](LICENSE).
