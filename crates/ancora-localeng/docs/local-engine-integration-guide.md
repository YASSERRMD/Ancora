# Local Engine Integration Guide

This document explains how to integrate `ancora-localeng` with each
supported local inference engine.

## Supported Engines

| Engine | Default Port | Transport |
|---|---|---|
| llama.cpp server | 8080 | HTTP |
| llama.cpp embedded | N/A | In-process |
| Ollama | 11434 | HTTP |
| vLLM | 8000 | HTTP (OpenAI-compat) |
| SGLang | 30000 | HTTP |
| LM Studio | 1234 | HTTP (OpenAI-compat) |
| TGI | 8080 | HTTP |
| ONNX Runtime | N/A | In-process |

## Architecture

Every engine module exposes:

1. A typed `EngineConfig` builder.
2. An error enum.
3. A transport trait (e.g., `LlamaServerTransport`) that can be
   implemented by real HTTP clients or mock structs for tests.
4. A client struct that owns the config and transport.

This layering means all engine logic is unit-testable offline without
starting any server.

## Usage Pattern

```rust
use ancora_localeng::ollama::{OllamaClient, default_config};

// In production, supply a real HTTP transport.
// In tests, supply a mock transport.
let config = default_config().with_endpoint("http://127.0.0.1:11434");
let client = OllamaClient::new(config, "llama3", my_transport);
let result = client.complete(&request)?;
```

## Health Checking

Use `health::MockHealthChecker` in tests and supply a real prober in
production:

```rust
use ancora_localeng::health::{MockHealthChecker, HealthChecker};
use ancora_localeng::model::EngineKind;

let checker = MockHealthChecker::healthy(EngineKind::Ollama);
let status = checker.check();
assert!(status.is_ready());
```

## Capability Detection

Query the static capability table before selecting an engine:

```rust
use ancora_localeng::capability::Capabilities;
use ancora_localeng::model::EngineKind;

let caps = Capabilities::for_engine(&EngineKind::Vllm);
assert!(caps.continuous_batching);
```

## Engine Selection by Hardware

Use `runtime::select_engine` to pick the best engine automatically:

```rust
use ancora_localeng::runtime::{HardwareProfile, SelectionCriteria, select_engine};

let hw = HardwareProfile::cpu_only(32.0, 8).with_cuda(24.0);
let criteria = SelectionCriteria::new(hw).prefer_throughput();
let result = select_engine(&criteria);
println!("selected: {} ({})", result.engine, result.reason);
```

## Running Tests Offline

All tests use mock transports and require no running server:

```
cargo test -p ancora-localeng
```
