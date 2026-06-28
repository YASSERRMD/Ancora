# Per-Language App Notes

## Go

- SDK: anthropic-go
- Entry point: `GoApp::new(name)` + `GoApp::run(input)`
- Trace ID format: `go-trace-{len}`
- Valid roles: user, assistant, system

## Python

- SDK: anthropic (PyPI)
- Entry point: `PythonApp::new(name, model)` + `PythonApp::run(input)`
- Trace ID format: `py-trace-{word_count}`
- Token counting: whitespace-split word count (offline stub)

## TypeScript

- SDK: @anthropic-ai/sdk (npm)
- Entry point: `TsApp::new(name, sdk_version)` + `TsApp::run(input)`
- Trace ID format: `ts-trace-{len}`
- Stop reason: always `end_turn` in the offline stub

## .NET

- SDK: Anthropic.Client (NuGet)
- Entry point: `DotnetApp::new(name, framework)` + `DotnetApp::run(input)`
- Supported frameworks: net6.0, net8.0, net9.0
- Trace ID format: `dotnet-trace-{len}`

## Java

- SDK: anthropic-java (Maven)
- Entry point: `JavaApp::new(name, java_version)` + `JavaApp::run(input)`
- Minimum Java version: 11
- Trace ID format: `java-trace-{len}`

## Rust

- SDK: anthropic-rs (crates.io)
- Entry point: `RustApp::new(name, edition)` + `RustApp::run(input)`
- Supported editions: 2018, 2021, 2024
- Trace ID format: `rust-trace-{len}`
