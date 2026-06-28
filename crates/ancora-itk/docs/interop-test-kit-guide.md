# Interop Test Kit Guide

The `ancora-itk` crate provides a collection of conformance kits that certify
Ancora extensions against the interoperability specification. Each kit
exercises a specific extension point and produces structured pass/fail results.

## Overview

Extensions in the Ancora ecosystem (providers, vector stores, tools, graders,
guardrails, exporters, and plugins) must satisfy well-defined contracts. The
interop test kit (ITK) encodes those contracts as executable checks.

## Available Kits

| Kit | Extension Type | Checks |
|-----|---------------|--------|
| `ProviderKit` | LLM providers | name, models list, complete() |
| `VectorStoreKit` | Vector stores | name, upsert + search round-trip |
| `ToolKit` | Tool extensions | name, description, schema, call() |
| `GraderKit` | Graders | name, perfect-match score, score bounds |
| `GuardrailKit` | Guardrails | name, blocks configured sample, allows safe content |
| `ExporterKit` | Exporters | name, format, export non-empty, export empty |
| `PluginKit` | Plugins | name, version, Init/Shutdown events, metadata |

## Quick Start

```rust
use ancora_itk::provider_kit::{Provider, ProviderKit};

struct MyProvider;

impl Provider for MyProvider {
    fn name(&self) -> &str { "my-provider" }
    fn models(&self) -> Vec<String> { vec!["my-model-v1".into()] }
    fn complete(&self, prompt: &str) -> Result<String, String> {
        Ok(format!("Response to: {prompt}"))
    }
}

fn main() {
    let kit = ProviderKit::new();
    let results = kit.run(&MyProvider);
    for r in &results {
        println!("{}: {}", if r.passed { "PASS" } else { "FAIL" }, r.name);
    }
}
```

## Runner and Reports

Use the `Runner` to collect results from multiple kits and produce a unified
`KitReport`:

```rust
use ancora_itk::runner::Runner;
use ancora_itk::badge::issue_badge;

let mut runner = Runner::new();
// record results from each kit...
runner.record_kit("provider_kit", results.iter().map(|r| {
    (r.name.clone(), r.passed, r.message.clone())
}));

let report = runner.into_report("My Extension Suite");
println!("{}", report.render());

if let Some(badge) = issue_badge("my-extension", &report) {
    println!("{}", badge.render());
}
```

## Badge Tiers

| Tier | Condition |
|------|-----------|
| `Compliant` | All checks pass |
| `PartiallyCompliant` | Fewer than half of checks fail |
| `NonCompliant` | Half or more checks fail |
