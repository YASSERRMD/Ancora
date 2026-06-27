# Go SDK Test Plan

This document describes the Go SDK test suite structure, how to run it, and conventions for adding new tests.

## Overview

The Go SDK test suite lives in `sdk/go/ancora/` alongside the implementation files. Tests use the standard `testing` package and the `ancora_test` external package to verify the public API.

## Test Categories

| Category | File Pattern | Description |
|---|---|---|
| Unit (lifecycle) | `lifecycle_test.go` | Runtime alloc/free, GC safety |
| Unit (spec) | `spec_ffi_test.go`, `spec_test.go` | AgentSpec and ToolSpec round-trip |
| Unit (run) | `single_agent_run_test.go` | StartRun, PollEvent, DrainEvents |
| Unit (tools) | `tool_error_propagation_test.go`, `toolkit_test.go` | Tool callbacks and error paths |
| Unit (structured output) | `structured_output_test.go`, `schema_test.go` | Schema generation |
| Unit (store) | `memory_readwrite_test.go`, `sqlite_store_test.go` | SQLite journal operations |
| Unit (transport) | `cgo_transport_test.go`, `grpc_transport_test.go` | Transport implementations |
| Conformance | `conformance_suite_test.go`, `e2e_conformance_suite_test.go` | All four canonical scenarios |
| End-to-end | `e2e_*_test.go` | Full-stack offline e2e flows |
| Security | `e2e_security_*_test.go` | Airgap and auth tests |
| Reliability | `e2e_reliability_test.go`, `e2e_restart_recovery_test.go` | Stability and persistence |
| Performance | `ffi_overhead_bench_test.go` | FFI call overhead benchmarks |

## Running Tests

### Prerequisites

Build the Rust FFI library first:

```bash
cargo build -p ancora-ffi --manifest-path Cargo.toml
```

### Run all tests

```bash
cd sdk/go
make test
```

Or manually:

```bash
cd sdk/go
CGO_ENABLED=1 \
CGO_LDFLAGS="-L../../target/debug -lancora_ffi" \
go test ./ancora/... -v
```

### Run benchmarks

```bash
cd sdk/go
CGO_ENABLED=1 \
CGO_LDFLAGS="-L../../target/debug -lancora_ffi" \
go test ./ancora/... -bench=. -benchtime=5s -count=3
```

### Run with coverage

```bash
cd sdk/go
make coverage
```

The `coverage` target enforces a minimum statement coverage of 70% by default. Override with:

```bash
make coverage COVER_MIN=80
```

### Run a single test

```bash
CGO_ENABLED=1 \
CGO_LDFLAGS="-L../../target/debug -lancora_ffi" \
go test ./ancora/... -run TestE2ESingleAgent -v
```

## Conventions

- All tests are in `package ancora_test` (external test package).
- No live network calls: fixture callbacks return in-process data.
- Store tests use `:memory:` for speed; restart-recovery tests use `t.TempDir()`.
- Benchmarks are in `*_bench_test.go` files using `func Benchmark*(b *testing.B)`.
- Helper functions (`mustRuntime`, `drainEvents`) are defined in `helpers_test.go`.
- Use `t.Logf` for informational output that is not a hard failure.

## Test Helper: `mustRuntime`

```go
func mustRuntime(t *testing.T) *ancora.Runtime {
    t.Helper()
    rt, err := ancora.NewRuntime()
    if err != nil {
        t.Fatalf("NewRuntime: %v", err)
    }
    return rt
}
```

## Coverage Targets

| Package | Minimum Coverage |
|---|---|
| `ancora.io/sdk/ancora` | 70% (enforced by `make coverage`) |

## Adding New Tests

1. Create a `*_test.go` file in `sdk/go/ancora/`.
2. Use `package ancora_test`.
3. Import `ancora.io/sdk/ancora`.
4. Ensure the test runs offline (no live HTTP calls).
5. Run `go vet ./ancora/...` before committing.
