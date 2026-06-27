# Edge Deployment (Rust)

## Release build

```bash
cargo build --release
```

The binary in `target/release/` is fully self-contained. Copy it to any
machine with the same OS and architecture.

## Static linking (musl)

Build a fully static binary for Linux:

```bash
rustup target add x86_64-unknown-linux-musl
cargo build --release --target x86_64-unknown-linux-musl
```

The resulting binary has no shared library dependencies and runs in a scratch
Docker container.

## Minimal Docker image

```dockerfile
FROM scratch
COPY target/x86_64-unknown-linux-musl/release/my_agent /agent
ENV ANCORA_MODEL_URL=http://ollama:11434
ENTRYPOINT ["/agent"]
```

```bash
docker build -t my-agent:latest .
docker run -e ANTHROPIC_API_KEY=sk-... my-agent:latest
```

## Cross-compilation

Install a cross-compiler and build for ARM64:

```bash
rustup target add aarch64-unknown-linux-musl
cargo install cross
cross build --release --target aarch64-unknown-linux-musl
```

## WebAssembly (WASI)

Ancora supports the `wasm32-wasip1` target for WASI runtimes:

```bash
rustup target add wasm32-wasip1
cargo build --release --target wasm32-wasip1
wasmtime target/wasm32-wasip1/release/my_agent.wasm
```

Tool calls that require network access work via WASI socket APIs.

## See also

- [Install](install.md)
- [Configuration](configuration.md)
