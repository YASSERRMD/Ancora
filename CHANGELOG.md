# Changelog

All notable changes to Ancora are documented here.
Format follows [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).
Versions follow [Semantic Versioning](https://semver.org/).

## [0.6.0] - 2026-06-28

### Added

- Performance and benchmark suite (18 bench files covering all hot paths)
- Example parity tests confirming all 6 SDK languages produce identical outputs
- Documentation audit tests (19 audit files)
- Coverage gate tests (19 gate files ensuring no category drops below threshold)
- Security and policy tests (11 security files, 8 policy files)
- Reliability and chaos tests (9 chaos, 5 load, 5 reliability files)
- Determinism guarantee tests (19 det_ files with 14-guarantee doc)
- Cross-language journal interop tests for all 6 SDK languages
- A2A envelope conformance across all 6 language pairs
- MCP tool conformance tests (rust-server, go-server)
- OTel span field validation (6 required fields, canonical trace_id)
- Chinese provider support: Qwen, GLM, DeepSeek, Kimi, MiniMax, StepFun, ERNIE, Hunyuan, Doubao, MiMo
- Vector store conformance for all 11 backends
- Edge deployment with wasm32 targets
- Structured output validation across all 6 languages
- Human-in-the-loop gate with approve/reject/pending states

### Changed

- Workspace version bumped to 0.6.0 across all crates

### Fixed

- Replay divergence detection now checks both length and value mismatches

## [0.5.0] - 2026-04-01

### Added

- Initial provider coverage matrix
- Go, Python, TypeScript SDK examples
- Basic journal write and replay

## [0.1.0] - 2025-12-01

### Added

- Initial workspace scaffolding
- ancora-core, ancora-proto, ancora-inference crates
- Basic journal format with 10 event types
