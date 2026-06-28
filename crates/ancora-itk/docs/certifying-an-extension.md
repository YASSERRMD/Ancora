# Certifying an Extension

This guide walks through the full certification process for an Ancora extension
using the Interop Test Kit.

## Step 1 - Implement the required trait

Each extension type has a corresponding trait in `ancora-itk`. Implement it for
your adapter struct. For a provider:

```rust
use ancora_itk::provider_kit::Provider;

pub struct AcmeProvider {
    api_key: String,
}

impl Provider for AcmeProvider {
    fn name(&self) -> &str { "acme" }
    fn models(&self) -> Vec<String> { vec!["acme-nano".into(), "acme-pro".into()] }
    fn complete(&self, prompt: &str) -> Result<String, String> {
        // call the Acme API (stub for certification)
        Ok(format!("[acme] {prompt}"))
    }
}
```

## Step 2 - Run the kit

```rust
use ancora_itk::provider_kit::ProviderKit;

let kit = ProviderKit::new();
let provider = AcmeProvider { api_key: "test".into() };
let results = kit.run(&provider);
```

## Step 3 - Collect results with the Runner

```rust
use ancora_itk::runner::Runner;

let mut runner = Runner::new();
runner.record_kit(
    "provider_kit",
    results.iter().map(|r| (r.name.clone(), r.passed, r.message.clone())),
);
let report = runner.into_report("Acme Provider Certification");
println!("{}", report.render());
```

## Step 4 - Issue a badge

```rust
use ancora_itk::badge::issue_badge;

if let Some(badge) = issue_badge("acme-provider", &report) {
    if badge.is_compliant() {
        println!("Certified: {}", badge.render());
    } else {
        println!("Not certified: {}", badge.render());
        // Review report.failing_lines() for details.
    }
}
```

## Certification Criteria

An extension is considered certified when all conformance checks for its kit
pass, yielding a `Compliant` badge. Partially compliant extensions may be
included in the registry with a warning label. Non-compliant extensions must
fix all failures before inclusion.

## Continuous Integration

Add the kit run to your CI pipeline using the bundled adapter tests in
`.github/workflows/itk.yml` (see the ci(itk) commit for a template). The
workflow builds `ancora-itk` and runs all kit tests offline, with no external
network calls required.
