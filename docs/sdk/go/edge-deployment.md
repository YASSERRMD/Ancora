# Single-Binary Edge Deployment (Go)

Deploy an Ancora Go agent as a single binary that includes the native library.

## Build

```bash
# 1. Build the native library
cargo build --release -p ancora-ffi

# 2. Copy it next to your Go binary or embed it
cp target/release/libancora_ffi.so ./myagent/

# 3. Build the Go binary
CGO_ENABLED=1 CGO_LDFLAGS="-L./myagent -lancora_ffi" \
    go build -o myagent/agent ./cmd/agent
```

## Runtime library path

Set `LD_LIBRARY_PATH` so the binary can find the native library at runtime:

```bash
LD_LIBRARY_PATH=./myagent ./myagent/agent
```

## Air-gapped setup

1. Copy the binary + native library to the air-gapped host (via USB or
   secure transfer).
2. Install Ollama on the host and pull the required model weight:
   ```bash
   ollama pull llama3
   ```
3. Set `ANCORA_MODEL_URL=http://127.0.0.1:11434`.
4. Run the binary. No internet connection is required.

## SQLite journal for edge

```go
store, _ := ancora.OpenSqliteStore("/var/lib/myagent/journal.db")
```

SQLite is bundled via CGo; no external database required.

## See also

- [Deployment models concept](../../concepts/deployment-models.md)
- [Durability](durability.md)
