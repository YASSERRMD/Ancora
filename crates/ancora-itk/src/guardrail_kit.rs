/// Conformance kit for guardrail extensions.

/// Decision made by a guardrail.
#[derive(Debug, Clone, PartialEq)]
pub enum GuardrailDecision {
    Allow,
    Block { reason: String },
}

/// Trait that every guardrail extension must satisfy.
pub trait Guardrail {
    fn name(&self) -> &str;
    fn check(&self, text: &str) -> Result<GuardrailDecision, String>;
}

/// A single conformance check result.
#[derive(Debug, Clone, PartialEq)]
pub struct CheckResult {
    pub name: String,
    pub passed: bool,
    pub message: String,
}

/// Kit that runs conformance checks against a [`Guardrail`].
pub struct GuardrailKit {
    /// Text the kit expects to be blocked.
    pub blocked_sample: String,
    /// Text the kit expects to be allowed.
    pub allowed_sample: String,
}

impl GuardrailKit {
    pub fn new(blocked_sample: impl Into<String>, allowed_sample: impl Into<String>) -> Self {
        GuardrailKit {
            blocked_sample: blocked_sample.into(),
            allowed_sample: allowed_sample.into(),
        }
    }

    pub fn run<G: Guardrail>(&self, guardrail: &G) -> Vec<CheckResult> {
        vec![
            self.check_name(guardrail),
            self.check_blocks(guardrail),
            self.check_allows(guardrail),
        ]
    }

    fn check_name<G: Guardrail>(&self, guardrail: &G) -> CheckResult {
        if guardrail.name().is_empty() {
            CheckResult {
                name: "guardrail_name_nonempty".into(),
                passed: false,
                message: "Guardrail name must not be empty".into(),
            }
        } else {
            CheckResult {
                name: "guardrail_name_nonempty".into(),
                passed: true,
                message: format!("Guardrail name: {}", guardrail.name()),
            }
        }
    }

    fn check_blocks<G: Guardrail>(&self, guardrail: &G) -> CheckResult {
        match guardrail.check(&self.blocked_sample) {
            Ok(GuardrailDecision::Block { .. }) => CheckResult {
                name: "guardrail_blocks_sample".into(),
                passed: true,
                message: "Blocked the expected content".into(),
            },
            Ok(GuardrailDecision::Allow) => CheckResult {
                name: "guardrail_blocks_sample".into(),
                passed: false,
                message: "Guardrail allowed content it should have blocked".into(),
            },
            Err(e) => CheckResult {
                name: "guardrail_blocks_sample".into(),
                passed: false,
                message: format!("check() errored: {e}"),
            },
        }
    }

    fn check_allows<G: Guardrail>(&self, guardrail: &G) -> CheckResult {
        match guardrail.check(&self.allowed_sample) {
            Ok(GuardrailDecision::Allow) => CheckResult {
                name: "guardrail_allows_safe_content".into(),
                passed: true,
                message: "Allowed safe content".into(),
            },
            Ok(GuardrailDecision::Block { reason }) => CheckResult {
                name: "guardrail_allows_safe_content".into(),
                passed: false,
                message: format!("Guardrail over-blocked safe content: {reason}"),
            },
            Err(e) => CheckResult {
                name: "guardrail_allows_safe_content".into(),
                passed: false,
                message: format!("check() errored: {e}"),
            },
        }
    }
}
