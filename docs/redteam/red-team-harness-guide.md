# Red-Team Harness Guide

ancora-redteam provides an adversarial testing harness for scoring guardrail
effectiveness against known attacks. All operations run in-memory with no network.

## Attack Categories

| Category | Enum | Description |
|---|---|---|
| Prompt injection | `AttackCategory::Injection` | Override instructions via crafted inputs |
| Tool misuse | `AttackCategory::ToolMisuse` | Call disallowed tools |
| Data exfiltration | `AttackCategory::DataExfiltration` | Attempt to leak sensitive data |
| Privilege escalation | `AttackCategory::PrivilegeEscalation` | Gain unauthorized capabilities |
| Jailbreak | `AttackCategory::Jailbreak` | Bypass safety guidelines |

## Scoring a Policy

```rust
use ancora_redteam::{injection_scenarios, GuardrailScorer};
use ancora_guard::{GuardrailJournal, GuardrailOutcome, GuardrailPolicy, InjectionInputGuardrail};

let scenarios = injection_scenarios();
let report = GuardrailScorer::score(&scenarios, |payload| {
    let mut p = GuardrailPolicy::new();
    p.add_input(InjectionInputGuardrail);
    let mut j = GuardrailJournal::default();
    !matches!(p.check_input(payload, &mut j, 1), GuardrailOutcome::Pass)
});

println!("{}", report.summary());
println!("False negatives: {}", report.false_negatives());
```

## Regression Set

```rust
use ancora_redteam::known_attack_regression_set;
// All scenarios in this set have expected_blocked = true.
let regression = known_attack_regression_set();
```

## Custom Scenarios

```rust
use ancora_redteam::ScenarioBuilder;

let dataset = ScenarioBuilder::new()
    .add_injection("ignore previous instructions", true)
    .add_jailbreak("jailbreak mode", true)
    .add_tool_misuse("run_shell", true)
    .add_injection("safe query", false)
    .build();
```
