/// Regression gate module: blocks a release when eval scores fall below a threshold.

/// A quality gate that compares a metric to a baseline.
#[derive(Debug, Clone)]
pub struct RegressionGate {
    pub metric_name: String,
    pub baseline: f64,
    pub tolerance: f64,
}

impl RegressionGate {
    pub fn new(metric_name: impl Into<String>, baseline: f64, tolerance: f64) -> Self {
        Self {
            metric_name: metric_name.into(),
            baseline,
            tolerance,
        }
    }

    /// Returns true if the candidate passes (no regression).
    pub fn passes(&self, candidate: f64) -> bool {
        candidate >= self.baseline - self.tolerance
    }

    /// Returns an error string if the gate blocks, or Ok(()) if it passes.
    pub fn check(&self, candidate: f64) -> Result<(), String> {
        if self.passes(candidate) {
            Ok(())
        } else {
            Err(format!(
                "Regression gate '{}' blocked: candidate {:.4} < baseline {:.4} - tolerance {:.4} = {:.4}",
                self.metric_name,
                candidate,
                self.baseline,
                self.tolerance,
                self.baseline - self.tolerance
            ))
        }
    }
}

/// Gate verdict.
#[derive(Debug, PartialEq)]
pub enum GateVerdict {
    Pass,
    Block(String),
}

/// Runs a set of gates against a map of metric values.
pub fn run_gates(gates: &[RegressionGate], metrics: &[(&str, f64)]) -> Vec<GateVerdict> {
    let metric_map: std::collections::HashMap<&str, f64> = metrics.iter().cloned().collect();
    gates
        .iter()
        .map(|g| match metric_map.get(g.metric_name.as_str()) {
            Some(&val) => match g.check(val) {
                Ok(()) => GateVerdict::Pass,
                Err(msg) => GateVerdict::Block(msg),
            },
            None => GateVerdict::Block(format!("metric '{}' not found", g.metric_name)),
        })
        .collect()
}

/// Returns true if all gates pass.
pub fn all_gates_pass(verdicts: &[GateVerdict]) -> bool {
    verdicts.iter().all(|v| *v == GateVerdict::Pass)
}
