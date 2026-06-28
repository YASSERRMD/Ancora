/// A single benchmark result for one capability area.
#[derive(Debug, Clone)]
pub struct BenchResult {
    /// Identifier such as `"planner"`, `"reflection"`, `"routing"`.
    pub name: String,
    /// Wall-clock nanoseconds for a single representative operation.
    pub elapsed_ns: u64,
    /// Optional token count (for capabilities that emit journal entries).
    pub token_units: u64,
    /// Quality score in [0.0, 1.0] when applicable; `None` otherwise.
    pub quality: Option<f64>,
}

impl BenchResult {
    pub fn new(name: impl Into<String>, elapsed_ns: u64) -> Self {
        Self {
            name: name.into(),
            elapsed_ns,
            token_units: 0,
            quality: None,
        }
    }

    pub fn with_token_units(mut self, units: u64) -> Self {
        self.token_units = units;
        self
    }

    pub fn with_quality(mut self, q: f64) -> Self {
        self.quality = Some(q);
        self
    }
}

/// Accumulated benchmark results for a full suite run.
#[derive(Debug, Default)]
pub struct BenchReport {
    pub results: Vec<BenchResult>,
}

impl BenchReport {
    pub fn push(&mut self, r: BenchResult) {
        self.results.push(r);
    }

    /// Find a result by name.
    pub fn get(&self, name: &str) -> Option<&BenchResult> {
        self.results.iter().find(|r| r.name == name)
    }

    /// Summarize as a plain-text table (for CI logs).
    pub fn summary(&self) -> String {
        let mut lines = vec!["name                    elapsed_ns  token_units  quality".to_string()];
        for r in &self.results {
            let q = match r.quality {
                Some(v) => format!("{:.3}", v),
                None => "-".to_string(),
            };
            lines.push(format!(
                "{:<24} {:>11}  {:>11}  {}",
                r.name, r.elapsed_ns, r.token_units, q
            ));
        }
        lines.join("\n")
    }
}
