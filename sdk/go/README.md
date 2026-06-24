# Ancora Go SDK

The Go SDK wraps the Ancora Rust core via an in-process FFI staticlib and
exposes a Go-native API for starting, polling, and resuming agent runs.

## Requirements

- Go 1.23+
- Rust toolchain (to build the FFI staticlib)
- CGO enabled (default on macOS and Linux)

## Quick start

```bash
# Build the Rust FFI staticlib
make -C sdk/go build-ffi

# Run all tests
make -C sdk/go test
```

## Basic usage

```go
package main

import (
    "context"
    "fmt"

    "ancora.io/sdk/ancora"
)

func main() {
    rt, err := ancora.NewRuntime()
    if err != nil {
        panic(err)
    }
    defer rt.Free()

    spec := ancora.NewAgentSpec("my-agent", "llama3", "you are a helpful assistant")
    ag := ancora.NewTransportAgent(ancora.NewCgoTransport(rt), spec)

    run, err := ag.Start(context.Background())
    if err != nil {
        panic(err)
    }
    fmt.Printf("run started: %s\n", run.ID())

    evs, _ := run.DrainEvents(context.Background())
    for _, ev := range evs {
        fmt.Println(ev)
    }
}
```

## Optional SQLite persistence

```go
store, _ := ancora.OpenSqliteStore("runs.db")
defer store.Close()
tr := ancora.NewStoringTransport(ancora.NewCgoTransport(rt), store)
ag := ancora.NewTransportAgent(tr, spec)
```

## Examples

| Example | Description |
|---------|-------------|
| [single-agent](examples/single-agent/) | A single agent runs to completion |
| [multi-agent-verifier](examples/multi-agent-verifier/) | Agent and verifier with a dependency |
| [human-in-loop](examples/human-in-loop/) | Run suspends for human approval |

Run any example from the `sdk/go` directory:

```bash
go run ./examples/single-agent
go run ./examples/multi-agent-verifier
go run ./examples/human-in-loop
```

## Package overview

| Package | Purpose |
|---------|---------|
| `ancora` | Core types: Runtime, Agent, Run, Transport, SqliteStore |
| `ancora/grpc` | Generated gRPC stubs for the remote RunService |
| `cmd/ancora-agent` | Single-binary agent runner with optional SQLite persistence |

## Transport selection

Set `ANCORA_TRANSPORT=grpc` and `ANCORA_GRPC_ADDR=host:port` to route runs
through a remote gRPC RunService instead of the in-process FFI. All other
values (including unset) use the default in-process CGo transport.
