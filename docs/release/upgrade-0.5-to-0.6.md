# Upgrade and Migration Guide: 0.5 to 0.6

## Overview

Ancora 0.6.0 is an additive release with no breaking API changes.
You can upgrade by bumping the version in your dependency manifest.

## Rust

Update `Cargo.toml`:

```toml
[dependencies]
ancora-core = "0.6"
```

## Go

```bash
go get github.com/YASSERRMD/ancora@v0.6.0
```

## Python

```bash
pip install ancora==0.6.0
```

## TypeScript

```bash
npm install ancora@0.6.0
```

## .NET

```bash
dotnet add package Ancora --version 0.6.0
```

## Java

```xml
<dependency>
  <groupId>io.ancora</groupId>
  <artifactId>ancora-java</artifactId>
  <version>0.6.0</version>
</dependency>
```

## New features to adopt

### Chinese providers

Pass the provider name in your run config. No code changes needed for existing runs.

```rust
let config = ProviderConfig::new("qwen3-local").with_residency("cn");
```

### Vector store backends

All 11 backends are now conformance-tested. Specify the backend in `VectorStoreConfig.backend`.

### Benchmark budgets

The `bench_*` tests document expected throughput. If your production numbers
differ significantly from the budgets, investigate I/O or allocation pressure.

## Nothing to migrate

- Journal format unchanged.
- A2A envelope schema unchanged.
- OTel span fields unchanged.
- Cost formula unchanged.
