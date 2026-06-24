# ADR-0001: Rust core with C ABI and optional gRPC sidecar

Date: 2026-06-24
Status: Accepted

## Context

Ancora must be usable from Go, Python, TypeScript, .NET, and Java
without rewriting the core engine in each language. The engine must be
safe, fast, and self-contained enough to run offline on an edge device.
Each host language has its own preferred interop mechanism. A single
distribution strategy that satisfies all of them is required.

## Decision

The core engine is written in Rust and compiled to a cdylib and
staticlib. A cbindgen-generated C header provides the stable ABI surface
that all language bindings consume. An optional gRPC sidecar (ancora-grpc)
wraps the same core for languages that prefer network transport over
in-process FFI (for example, browser runtimes or polyglot server
deployments).

## Consequences

- Rust gives memory safety, deterministic performance, and a mature
  async runtime (Tokio) with no GC pauses.
- The C ABI is the lowest common denominator: every major language can
  call into it through cgo, PyO3/cffi, napi-rs, P/Invoke, or FFM.
- cbindgen generates the header automatically from Rust types, reducing
  manual maintenance.
- The gRPC sidecar is additive: languages that do not need in-process
  linking use it without forcing other bindings to pay the network cost.
- Cross-compiling the native library for all target triples adds CI
  complexity, addressed in a later phase.

## Alternatives considered

- Pure gRPC only: simpler distribution but requires a running sidecar
  even for single-process use, adds latency, and complicates offline
  operation.
- Rewrite in each language: unacceptable maintenance burden and risks
  behavioral divergence across bindings.
- WASM as the common target: not viable for server-side use due to WASI
  immaturity for async I/O and filesystem access at the time of this
  decision.
