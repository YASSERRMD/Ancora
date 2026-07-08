//! Regression gate parity - validates that regression gates fire consistently across SDKs.

/// A regression gate compares current eval scores against a baseline.
#[derive(Debug, Clone)]
pub struct RegressionGate {
    pub name: String,
    pub metric: String,
    pub baseline: f64,
    pub tolerance: f64,
}

impl RegressionGate {
    pub fn new(
        name: impl Into<String>,
        metric: impl Into<String>,
        baseline: f64,
        tolerance: f64,
    ) -> Self {
        Self {
            name: name.into(),
            metric: metric.into(),
            baseline,
            tolerance,
        }
    }

    /// Returns Ok if current score is within tolerance of baseline, Err otherwise.
    pub fn check(&self, current: f64) -> Result<(), String> {
        let delta = (current - self.baseline).abs();
        if delta <= self.tolerance {
            Ok(())
        } else {
            Err(format!(
                "gate {:?}: metric {:?} regressed by {:.4} (baseline={:.4}, current={:.4}, tolerance={:.4})",
                self.name, self.metric, delta, self.baseline, current, self.tolerance
            ))
        }
    }

    /// Check whether the gate passes for a given score (boolean helper).
    pub fn passes(&self, current: f64) -> bool {
        self.check(current).is_ok()
    }
}

/// Gate evaluation result for one language.
#[derive(Debug, Clone)]
pub struct GateResult {
    pub language: String,
    pub gate_name: String,
    pub passed: bool,
    pub reason: Option<String>,
}

impl GateResult {
    pub fn pass(language: impl Into<String>, gate_name: impl Into<String>) -> Self {
        Self {
            language: language.into(),
            gate_name: gate_name.into(),
            passed: true,
            reason: None,
        }
    }

    pub fn fail(
        language: impl Into<String>,
        gate_name: impl Into<String>,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            language: language.into(),
            gate_name: gate_name.into(),
            passed: false,
            reason: Some(reason.into()),
        }
    }
}

/// Check that all languages agree on gate pass/fail outcomes.
pub fn check_gate_parity(results: &[GateResult]) -> Vec<String> {
    use std::collections::HashMap;
    let mut issues = Vec::new();

    // Group by gate name
    let mut by_gate: HashMap<&str, Vec<&GateResult>> = HashMap::new();
    for r in results {
        by_gate.entry(&r.gate_name).or_default().push(r);
    }

    for (gate, group) in &by_gate {
        if let Some(first) = group.first() {
            for other in group.iter().skip(1) {
                if first.passed != other.passed {
                    issues.push(format!(
                        "gate {:?} outcome mismatch: {:?}={} vs {:?}={}",
                        gate, first.language, first.passed, other.language, other.passed
                    ));
                }
            }
        }
    }

    issues
}

/// Build standard regression gates used in the parity suite.
pub fn standard_gates() -> Vec<RegressionGate> {
    vec![
        RegressionGate::new("accuracy_gate", "mean_score", 0.85, 0.05),
        RegressionGate::new("latency_gate", "p50_latency_ms", 200.0, 50.0),
        RegressionGate::new("cost_gate", "total_cost_usd", 0.10, 0.02),
    ]
}

/// Run all standard gates against a set of language scores and return results.
pub fn run_gates(language: impl Into<String>, scores: &[(&str, f64)]) -> Vec<GateResult> {
    let lang = language.into();
    let gates = standard_gates();
    let score_map: std::collections::HashMap<&str, f64> = scores.iter().copied().collect();

    gates
        .iter()
        .map(|gate| match score_map.get(gate.metric.as_str()) {
            None => GateResult::fail(
                &lang,
                &gate.name,
                format!("metric {:?} not present", gate.metric),
            ),
            Some(&val) => match gate.check(val) {
                Ok(()) => GateResult::pass(&lang, &gate.name),
                Err(reason) => GateResult::fail(&lang, &gate.name, reason),
            },
        })
        .collect()
}
