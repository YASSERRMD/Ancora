# grpc-transport

Demonstrates starting an agent run via a remote gRPC server instead of the
embedded CGO runtime. Requires a running Ancora gRPC server.

## Run

```bash
ANCORA_GRPC_ADDR=localhost:50051 go run ./examples/grpc-transport
```

## What it shows

- Dialing a gRPC server with `google.golang.org/grpc`
- Wrapping the connection in a `GRPCTransport`
- Starting and draining an agent run over the wire
- Reading the server address from `ANCORA_GRPC_ADDR`
