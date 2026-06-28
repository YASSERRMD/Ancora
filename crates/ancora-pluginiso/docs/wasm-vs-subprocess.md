# Wasm vs Subprocess Plugins

## Overview

ancora-pluginiso supports two plugin runtime backends: WebAssembly (Wasm) and
subprocess. Each trades isolation strength against capability and overhead.

## WebAssembly Runtime

### How it works

The Wasm runtime compiles the plugin to a sandboxed linear-memory execution
environment. The host controls every import the plugin can call - there is no
way for the plugin to access host memory or OS resources outside what the host
explicitly exports.

In production, the runtime is backed by a Wasm engine such as Wasmtime or
Wasmer. CPU limits are enforced via fuel metering (each Wasm instruction
consumes one unit of fuel). Memory limits are enforced via the linear-memory
size cap at instantiation.

### Strengths

- Strong memory isolation: linear memory is fully separated from host memory
- Fine-grained CPU accounting via fuel metering
- Portable: same .wasm binary runs on any host OS/architecture
- Fast cold-start: compilation is cached; subsequent instantiations re-use the
  compiled artifact

### Limitations

- Limited system call surface: plugins must use WASI or host-defined imports
- No multi-threading within the Wasm module without the threads proposal
- Wasm binary size can be larger than a native executable for the same logic

### Use cases

- Untrusted third-party plugins from a plugin marketplace
- Short-lived, stateless compute tasks
- Plugins that need to be loaded and unloaded frequently

## Subprocess Runtime

### How it works

The subprocess runtime spawns the plugin as a child process. The host
communicates with it over a stdio-based IPC protocol. OS-level process
isolation (address-space separation) provides the primary containment. On
POSIX systems, `setrlimit(2)` enforces resource limits; a seccomp-bpf filter
restricts the set of allowed syscalls.

### Strengths

- Full OS-level address-space isolation
- Plugin can be written in any language that can read/write stdio
- Richer system call surface when the host explicitly allows it
- Crash causes a SIGCHLD - the host can detect and record it cleanly

### Limitations

- Higher process-spawn overhead compared to Wasm instantiation
- IPC latency: every call crosses a process boundary
- On Windows, Job Objects replace rlimits, and seccomp is unavailable

### Use cases

- Plugins written in languages without a Wasm compilation target
- Plugins that require native system libraries
- Long-running daemon-style plugins

## Choosing a Runtime

| Criterion              | Wasm            | Subprocess        |
|------------------------|-----------------|-------------------|
| Isolation strength     | Very high       | High              |
| Cold-start latency     | Low (cached)    | Medium            |
| Memory overhead        | Low             | Moderate          |
| Language support       | Wasm targets    | Any language      |
| Syscall surface        | WASI only       | Configurable      |
| CPU metering           | Instruction-level | Wall-clock + rlimit |
| Crash detection        | Trap handler    | SIGCHLD           |

When in doubt, prefer Wasm for untrusted plugins and subprocess for trusted
plugins that require native capabilities.
