# Adapting a Sample App

## Adding a New App to the Gallery

1. Create `src/<your_app>.rs` with your structs and logic.
2. Declare `pub mod <your_app>;` in `src/lib.rs`.
3. Add an `AppEntry` to `index::gallery()` in `src/index.rs`.
4. Create `src/tests/test_<your_app>_offline.rs` with at least one `#[test]`.
5. Declare the test module in the `#[cfg(test)] mod tests` block in `lib.rs`.

## Customising the Compliance Reviewer

Replace the government preset with a custom rule set:

```rust
use ancora_apps::compliance_review::{ComplianceReviewer, ComplianceRule, FindingSeverity};

let rules = vec![
    ComplianceRule::new("CORP-001", "Must include author.", FindingSeverity::High)
        .require("AUTHOR"),
];
let reviewer = ComplianceReviewer::new(ComplianceProfile::Commercial, rules);
```

## Customising Guardrails

Extend the default rule set with domain-specific patterns:

```rust
use ancora_apps::safety::{GuardrailRule, SafetyGuardrail};

let mut rules = SafetyGuardrail::default_rules().rules; // borrow pattern shown for clarity
rules.push(GuardrailRule::block("CUSTOM-001", "confidential_field"));
let guard = SafetyGuardrail::new(rules);
```

## Plugging in a Real Local Model

Swap the `ModelBackend::Stub` for `ModelBackend::LocalFile` and wire the
inference call through your GGUF runtime. The `run_local_inference` function
signature in `local_models` is the integration point.
