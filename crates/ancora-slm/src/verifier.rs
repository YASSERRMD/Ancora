//! Verifier-heavy patterns for small models.
//!
//! When running small models, verification is crucial: the model may silently
//! produce plausible-sounding but incorrect output.  This module implements:
//!
//! - Lightweight rule-based verifiers that run offline.
//! - A "model-as-verifier" hook for using a separate (optionally larger) model
//!   to double-check the primary model's output.
//! - An aggregating verifier that combines multiple checks.

use serde_json::Value;

/// The outcome of a verification check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Verdict {
    /// Output passes all checks.
    Pass,
    /// Output fails with a description of what went wrong.
    Fail(String),
}

impl Verdict {
    pub fn is_pass(&self) -> bool {
        matches!(self, Verdict::Pass)
    }
    pub fn is_fail(&self) -> bool {
        !self.is_pass()
    }
}

/// A single verification check.
pub trait Verifier: Send + Sync {
    /// Name of this verifier (for error messages).
    fn name(&self) -> &str;
    /// Run the verification check on `output`.
    fn verify(&self, output: &str) -> Verdict;
}

// ── Built-in rule-based verifiers ────────────────────────────────────────────

/// Checks that the output is not empty.
pub struct NonEmptyVerifier;

impl Verifier for NonEmptyVerifier {
    fn name(&self) -> &str {
        "non-empty"
    }
    fn verify(&self, output: &str) -> Verdict {
        if output.trim().is_empty() {
            Verdict::Fail("output is empty".into())
        } else {
            Verdict::Pass
        }
    }
}

/// Checks that the output is valid JSON.
pub struct ValidJsonVerifier;

impl Verifier for ValidJsonVerifier {
    fn name(&self) -> &str {
        "valid-json"
    }
    fn verify(&self, output: &str) -> Verdict {
        match serde_json::from_str::<Value>(output.trim()) {
            Ok(_) => Verdict::Pass,
            Err(e) => Verdict::Fail(format!("invalid JSON: {}", e)),
        }
    }
}

/// Checks that the output contains all required keywords (case-insensitive).
pub struct ContainsKeywordsVerifier {
    pub keywords: Vec<String>,
}

impl Verifier for ContainsKeywordsVerifier {
    fn name(&self) -> &str {
        "contains-keywords"
    }
    fn verify(&self, output: &str) -> Verdict {
        let lower = output.to_lowercase();
        for kw in &self.keywords {
            if !lower.contains(&kw.to_lowercase()) {
                return Verdict::Fail(format!("missing required keyword '{}'", kw));
            }
        }
        Verdict::Pass
    }
}

/// Checks that the output length is within a range (in characters).
pub struct LengthVerifier {
    pub min: usize,
    pub max: usize,
}

impl Verifier for LengthVerifier {
    fn name(&self) -> &str {
        "length"
    }
    fn verify(&self, output: &str) -> Verdict {
        let len = output.chars().count();
        if len < self.min {
            Verdict::Fail(format!("output too short: {} < {}", len, self.min))
        } else if len > self.max {
            Verdict::Fail(format!("output too long: {} > {}", len, self.max))
        } else {
            Verdict::Pass
        }
    }
}

/// Checks that a JSON object contains all required keys.
pub struct RequiredKeysVerifier {
    pub keys: Vec<String>,
}

impl Verifier for RequiredKeysVerifier {
    fn name(&self) -> &str {
        "required-keys"
    }
    fn verify(&self, output: &str) -> Verdict {
        let v: Value = match serde_json::from_str(output.trim()) {
            Ok(v) => v,
            Err(e) => return Verdict::Fail(format!("not JSON: {}", e)),
        };
        let obj = match v.as_object() {
            Some(o) => o,
            None => return Verdict::Fail("not a JSON object".into()),
        };
        for key in &self.keys {
            if !obj.contains_key(key.as_str()) {
                return Verdict::Fail(format!("missing required key '{}'", key));
            }
        }
        Verdict::Pass
    }
}

// ── Aggregating verifier ──────────────────────────────────────────────────────

/// Result of running a composed verification pipeline.
#[derive(Debug, Clone)]
pub struct VerificationReport {
    /// Overall result — `Fail` if any individual check failed.
    pub verdict: Verdict,
    /// Per-check results.
    pub checks: Vec<(String, Verdict)>,
}

impl VerificationReport {
    pub fn passed(&self) -> bool {
        self.verdict.is_pass()
    }
}

/// Run a slice of verifiers against `output` and return an aggregated report.
pub fn run_verifiers(output: &str, verifiers: &[Box<dyn Verifier>]) -> VerificationReport {
    let mut checks = Vec::new();
    let mut overall = Verdict::Pass;

    for v in verifiers {
        let verdict = v.verify(output);
        if verdict.is_fail() {
            overall = verdict.clone();
        }
        checks.push((v.name().to_string(), verdict));
    }

    VerificationReport {
        verdict: overall,
        checks,
    }
}

/// A function-based verifier for quick inline checks.
pub struct FnVerifier {
    name: String,
    func: Box<dyn Fn(&str) -> Verdict + Send + Sync>,
}

impl FnVerifier {
    pub fn new(
        name: impl Into<String>,
        func: impl Fn(&str) -> Verdict + Send + Sync + 'static,
    ) -> Self {
        Self {
            name: name.into(),
            func: Box::new(func),
        }
    }
}

impl Verifier for FnVerifier {
    fn name(&self) -> &str {
        &self.name
    }
    fn verify(&self, output: &str) -> Verdict {
        (self.func)(output)
    }
}
