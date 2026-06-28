# ancora-compliance: Compliance Reporting

`ancora-compliance` provides offline-first compliance evidence collection and reporting for SOC 2, FedRAMP, and ISO 27001 within Ancora multi-tenant agents.

## Frameworks

| Framework | Display | Example controls |
|-----------|---------|-----------------|
| `Soc2` | `SOC 2` | CC6.1, CC7.1, A1.1 |
| `Fedramp` | `FedRAMP` | AC-1, AU-2, IA-2 |
| `Iso27001` | `ISO 27001` | A.5.1, A.9.1, A.16.1 |
| `Pci` | `PCI DSS` | custom |
| `Hipaa` | `HIPAA` | custom |

## ComplianceControl

A control has an id, framework, title, description, status, evidence list, and optional assessed tick.

Status lifecycle: `NotAssessed` -> `Compliant` / `NonCompliant` / `PartiallyCompliant` / `NotApplicable`.

```rust
let mut ctrl = ComplianceControl::new("CC6.1", Framework::Soc2, "Access", "Access controls");
ctrl.set_status(ControlStatus::Compliant, tick);
ctrl.attach_evidence("ev-001");
```

## ControlRegistry

In-memory multi-framework registry. Query by framework, status, compliant/non-compliant counts.

## Presets

```rust
use ancora_compliance::presets;
let controls = presets::soc2_controls();    // 5 SOC 2 controls
let controls = presets::iso27001_controls(); // 5 ISO 27001 controls
let controls = presets::fedramp_controls();  // 5 FedRAMP controls
```

## AutoAssessor

```rust
AutoAssessor::load_preset(&mut registry, presets::soc2_controls());
AutoAssessor::bulk_mark_compliant(&mut registry, &mut audit, &["CC6.1", "CC6.2"], &Framework::Soc2, "tenant", "alice", tick);
```

## ComplianceReport

```rust
let report = ComplianceReport::generate(&registry, &Framework::Soc2, "tenant-1", tick);
println!("Compliance rate: {:.1}%", report.compliance_rate() * 100.0);
println!("Fully compliant: {}", report.is_fully_compliant());
```

## Gap Analysis

`GapAnalyzer::analyze(registry, framework)` returns non-compliant and not-assessed controls.
`GapAnalyzer::critical_gaps(registry, framework)` returns non-compliant controls with no evidence.

## Evidence

`EvidenceItem` records proof artifacts (log entries, test results, policy docs). `EvidenceStore` organizes them by tenant.

## Export

`report_to_csv(report)` and `controls_to_csv(controls)` for reporting pipelines.
