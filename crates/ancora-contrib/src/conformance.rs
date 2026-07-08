//! ancora-contrib: conformance harness
//!
//! Provides a lightweight, offline conformance test framework that any
//! contribution can use to verify it meets the SDK contract.

/// A single conformance check.
pub struct Check {
    pub name: String,
    pub description: String,
    pub run: Box<dyn Fn() -> CheckResult + Send + Sync>,
}

impl Check {
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        run: impl Fn() -> CheckResult + Send + Sync + 'static,
    ) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            run: Box::new(run),
        }
    }
}

impl std::fmt::Debug for Check {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Check")
            .field("name", &self.name)
            .field("description", &self.description)
            .finish()
    }
}

/// The outcome of a single conformance check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CheckResult {
    Pass,
    Fail(String),
    Skip(String),
}

impl CheckResult {
    pub fn is_pass(&self) -> bool {
        matches!(self, CheckResult::Pass)
    }

    pub fn is_fail(&self) -> bool {
        matches!(self, CheckResult::Fail(_))
    }

    pub fn is_skip(&self) -> bool {
        matches!(self, CheckResult::Skip(_))
    }
}

impl std::fmt::Display for CheckResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CheckResult::Pass => write!(f, "PASS"),
            CheckResult::Fail(reason) => write!(f, "FAIL: {reason}"),
            CheckResult::Skip(reason) => write!(f, "SKIP: {reason}"),
        }
    }
}

/// Aggregate report from running all checks in a suite.
#[derive(Debug, Clone)]
pub struct SuiteReport {
    pub suite_name: String,
    pub results: Vec<(String, CheckResult)>,
}

impl SuiteReport {
    pub fn passed(&self) -> usize {
        self.results.iter().filter(|(_, r)| r.is_pass()).count()
    }

    pub fn failed(&self) -> usize {
        self.results.iter().filter(|(_, r)| r.is_fail()).count()
    }

    pub fn skipped(&self) -> usize {
        self.results.iter().filter(|(_, r)| r.is_skip()).count()
    }

    pub fn all_passed(&self) -> bool {
        self.failed() == 0
    }

    pub fn total(&self) -> usize {
        self.results.len()
    }
}

impl std::fmt::Display for SuiteReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Conformance suite: {}", self.suite_name)?;
        for (name, result) in &self.results {
            writeln!(f, "  [{result}] {name}")?;
        }
        writeln!(
            f,
            "Total: {} passed, {} failed, {} skipped",
            self.passed(),
            self.failed(),
            self.skipped()
        )
    }
}

/// A named collection of conformance checks.
pub struct ConformanceSuite {
    pub name: String,
    checks: Vec<Check>,
}

impl ConformanceSuite {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            checks: Vec::new(),
        }
    }

    /// Register a check.
    pub fn add(&mut self, check: Check) {
        self.checks.push(check);
    }

    /// Run all checks and collect results.
    pub fn run(&self) -> SuiteReport {
        let results = self
            .checks
            .iter()
            .map(|c| (c.name.clone(), (c.run)()))
            .collect();
        SuiteReport {
            suite_name: self.name.clone(),
            results,
        }
    }
}

// ---------------------------------------------------------------------------
// Built-in helper checks
// ---------------------------------------------------------------------------

/// Build a standard provider conformance suite given a boxed provider under test.
///
/// The provider is wrapped in an `Arc` so it can be shared between closures.
pub fn provider_suite<P>(provider: std::sync::Arc<P>) -> ConformanceSuite
where
    P: crate::provider_template::ProviderAdapter + 'static,
{
    use crate::provider_template::{GenerateRequest, Message, Role};

    let mut suite = ConformanceSuite::new("provider-conformance");

    // Check 1: provider_id is non-empty.
    {
        let p = provider.clone();
        suite.add(Check::new(
            "provider_id_nonempty",
            "provider_id() must return a non-empty string",
            move || {
                if p.provider_id().is_empty() {
                    CheckResult::Fail("provider_id() returned an empty string".to_string())
                } else {
                    CheckResult::Pass
                }
            },
        ));
    }

    // Check 2: list_models is non-empty.
    {
        let p = provider.clone();
        suite.add(Check::new(
            "list_models_nonempty",
            "list_models() must return at least one model",
            move || {
                if p.list_models().is_empty() {
                    CheckResult::Fail("list_models() returned no models".to_string())
                } else {
                    CheckResult::Pass
                }
            },
        ));
    }

    // Check 3: generate returns Ok for a valid minimal request.
    {
        let p = provider.clone();
        let models = provider.list_models();
        suite.add(Check::new(
            "generate_ok_for_valid_request",
            "generate() must return Ok for a minimal valid request",
            move || {
                let model = match models.first() {
                    Some(m) => m.clone(),
                    None => return CheckResult::Skip("no models available".to_string()),
                };
                let req = GenerateRequest {
                    model,
                    messages: vec![Message {
                        role: Role::User,
                        content: "hello".to_string(),
                    }],
                    max_tokens: Some(16),
                    temperature: Some(0.0),
                };
                match p.generate(req) {
                    Ok(_) => CheckResult::Pass,
                    Err(e) => CheckResult::Fail(format!("generate returned error: {e}")),
                }
            },
        ));
    }

    suite
}

/// Build a standard grader conformance suite.
pub fn grader_suite<G>(grader: std::sync::Arc<G>) -> ConformanceSuite
where
    G: crate::grader_template::GraderPlugin + 'static,
{
    use crate::grader_template::GradeInput;

    let mut suite = ConformanceSuite::new("grader-conformance");

    // Check 1: grader_id is non-empty.
    {
        let g = grader.clone();
        suite.add(Check::new(
            "grader_id_nonempty",
            "grader_id() must return a non-empty string",
            move || {
                if g.grader_id().is_empty() {
                    CheckResult::Fail("grader_id() returned empty string".to_string())
                } else {
                    CheckResult::Pass
                }
            },
        ));
    }

    // Check 2: score is in [0, 1].
    {
        let g = grader.clone();
        suite.add(Check::new(
            "score_in_range",
            "grade() score must be in [0.0, 1.0]",
            move || {
                let input = GradeInput::new("What is 2+2?", "4").with_reference("4");
                match g.grade(&input) {
                    Ok(result) => {
                        if result.score >= 0.0 && result.score <= 1.0 {
                            CheckResult::Pass
                        } else {
                            CheckResult::Fail(format!(
                                "score {} is out of [0.0, 1.0] range",
                                result.score
                            ))
                        }
                    }
                    Err(e) => CheckResult::Fail(format!("grade returned error: {e}")),
                }
            },
        ));
    }

    suite
}
