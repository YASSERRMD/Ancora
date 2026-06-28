# Plugin Runtime Tradeoffs

## WASM vs Subprocess

Ancora supports two plugin isolation strategies: WebAssembly (WASM) and
child-process (subprocess). Each has distinct overhead characteristics.

### WASM plugins

- **Serialisation overhead**: input and output must be copied across the WASM
  linear-memory boundary. For small payloads this is negligible; for payloads
  above ~64 KiB the copy dominates.
- **Startup cost**: negligible after the module is compiled and cached.
- **Isolation**: memory-safe by construction; a buggy plugin cannot corrupt
  the host process.
- **Concurrency**: WASM instances can be cloned and run in parallel within
  the same OS thread pool.

### Subprocess plugins

- **Serialisation overhead**: input/output travel over OS pipes (stdin/stdout).
  Framing and newline handling add small but measurable latency.
- **Startup cost**: high for short-lived processes (fork + exec + dynamic
  linking). Mitigated by the `persistent` mode which reuses a single child
  process per plugin.
- **Isolation**: full OS process isolation; strongest security boundary.
- **Concurrency**: limited by the number of child processes spawned.

## Recommendation

Use WASM for plugins that are called frequently (> 100 calls/s) with small
payloads. Use subprocess isolation for plugins that handle sensitive data or
that require access to OS resources unavailable inside a WASM sandbox.
