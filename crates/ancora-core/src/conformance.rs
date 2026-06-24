/// A scenario the Ancora core and all bindings must pass.
pub struct ConformanceScenario {
    pub id: &'static str,
    pub description: &'static str,
    pub tags: &'static [&'static str],
}

/// Possible outcome when a binding runs a conformance scenario.
pub enum ConformanceResult {
    Passed,
    Failed { reason: String },
    Skipped { reason: String },
}
