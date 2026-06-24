# ADR-0004: Language binding order: Go, Python, TypeScript, .NET, Java

Date: 2026-06-24
Status: Accepted

## Context

Ancora will eventually support bindings for Go, Python, TypeScript, .NET,
and Java. The order in which bindings are developed matters: each one
validates and exercises the C ABI surface, identifies gaps in the FFI
design, and produces conformance tests that subsequent bindings must also
pass. Choosing the wrong first binding increases rework risk.

## Decision

Bindings are developed in the order: Go, Python, TypeScript, .NET, Java.

## Consequences

Go is first because its cgo interop with C static libraries is
straightforward, its type system is strict enough to catch ABI contract
errors early, and its single-binary build validates the offline deployment
story immediately. The cgo path also exercises handle lifetimes and memory
management in a way that informs the design of later bindings.

Python is second because it is the dominant language in the AI/ML
community and PyO3 provides a mature, well-documented binding layer. The
decorator-based tool registration pattern it enables is a key ergonomics
differentiator.

TypeScript is third because the napi-rs path is well understood after Go
and Python, and the WASM browser target validates a second transport
(gRPC-web/HTTP) distinct from the in-process FFI path.

.NET is fourth because P/Invoke over a C ABI is mature and well
documented. The SafeHandle pattern maps cleanly to the opaque handle model.

Java is last because the Foreign Function and Memory API (FFM) is the
newest of the five interop mechanisms and benefits from all the ABI
stability work done in the preceding bindings.

## Alternatives considered

- Python first: reasonable, but Go's stricter compile-time type checking
  catches ABI errors faster during initial FFI design.
- Simultaneous development: maximizes parallelism but increases the risk
  that an early ABI design flaw must be fixed in multiple bindings at
  once.
