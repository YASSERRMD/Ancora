# Safety Monitoring Guide

ancora-safemon provides real-time safety monitoring for all agent outputs.

## Overview

The safety monitoring system runs every agent output through a pipeline of classifiers:

1. PII Detection - identifies emails, SSNs, phone numbers, credit card numbers
2. Toxicity Detection - scores content for harmful language
3. Policy Violation Detection - checks against configurable policy rules
4. Hallucination Detection - flags overconfident or unverifiable claims

## Quick Start

```rust
use ancora_safemon::classifier::SafetyClassifier;

let clf = SafetyClassifier::new();
let report = clf.classify("Your agent output text here.");

if !report.is_safe() {
    println!("Safety issues: {}", report.summary());
}
```

## Components

### SafetyClassifier

The top-level classifier that aggregates all sub-classifiers.

### PiiDetector

Detects personally identifiable information with redaction support.

### ToxicityDetector

Scores content on a four-level scale: None, Mild, Moderate, Severe.

### PolicyViolationDetector

Checks text against configurable rules. Add custom rules at runtime.

### HallucinationDetector

Flags overconfident facts, unverifiable claims, and conflicting statements.

### IncidentLog

Maintains an audit trail of all detected safety incidents.

### AlertManager

Fires alerts to registered channels when incidents exceed severity thresholds.

### Dashboard

Provides an operational JSON snapshot of the safety state.

## Configuration

All detectors work with zero configuration. For custom policies, use:

```rust
use ancora_safemon::policy_violation::{PolicyRule, PolicyViolationDetector, ViolationKind};

let mut detector = PolicyViolationDetector::new();
detector.add_rule(PolicyRule::new(
    "CUSTOM-001",
    ViolationKind::ComplianceRule("HIPAA".to_string()),
    vec!["patient record", "medical history"],
    "HIPAA sensitive data",
));
```
