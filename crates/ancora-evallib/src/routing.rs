//! Routing efficiency eval suite.
//!
//! Tests that a router dispatches requests to the appropriate model or handler
//! based on cost, latency, and capability requirements.

/// Capability tier required by a request.
#[derive(Debug, Clone, PartialEq)]
pub enum CapabilityTier {
    /// Simple lookup or factual question - can use a small, cheap model.
    Light,
    /// Moderate reasoning - mid-tier model.
    Standard,
    /// Complex multi-step reasoning - large, powerful model.
    Advanced,
}

/// A routing eval case.
#[derive(Debug, Clone)]
pub struct RoutingCase {
    pub id: String,
    pub prompt: String,
    pub required_tier: CapabilityTier,
    /// Name of the model/handler that should be selected.
    pub expected_route: String,
}

impl RoutingCase {
    pub fn new(
        id: impl Into<String>,
        prompt: impl Into<String>,
        required_tier: CapabilityTier,
        expected_route: impl Into<String>,
    ) -> Self {
        RoutingCase {
            id: id.into(),
            prompt: prompt.into(),
            required_tier,
            expected_route: expected_route.into(),
        }
    }
}

/// Outcome of a routing eval.
#[derive(Debug, Clone, PartialEq)]
pub enum RoutingOutcome {
    Correct,
    WrongRoute { selected: String },
}

/// Simple offline router that classifies by keyword complexity.
pub struct LocalRouter;

impl LocalRouter {
    /// Classify the prompt into a capability tier.
    pub fn classify(&self, prompt: &str) -> CapabilityTier {
        let lower = prompt.to_lowercase();
        if lower.contains("prove") || lower.contains("multi-step") || lower.contains("complex") {
            CapabilityTier::Advanced
        } else if lower.contains("explain")
            || lower.contains("summarize")
            || lower.contains("compare")
        {
            CapabilityTier::Standard
        } else {
            CapabilityTier::Light
        }
    }

    /// Map a tier to the canonical route name.
    pub fn route(&self, tier: &CapabilityTier) -> String {
        match tier {
            CapabilityTier::Light => "model-light".into(),
            CapabilityTier::Standard => "model-standard".into(),
            CapabilityTier::Advanced => "model-advanced".into(),
        }
    }
}

/// The full routing eval suite.
pub struct RoutingSuite {
    pub cases: Vec<RoutingCase>,
}

impl RoutingSuite {
    pub fn default_catalog() -> Self {
        RoutingSuite {
            cases: vec![
                RoutingCase::new(
                    "ro-001",
                    "What is 2 + 2?",
                    CapabilityTier::Light,
                    "model-light",
                ),
                RoutingCase::new(
                    "ro-002",
                    "Explain the difference between TCP and UDP.",
                    CapabilityTier::Standard,
                    "model-standard",
                ),
                RoutingCase::new(
                    "ro-003",
                    "Prove that sqrt(2) is irrational using a multi-step proof.",
                    CapabilityTier::Advanced,
                    "model-advanced",
                ),
            ],
        }
    }

    pub fn evaluate(&self, case: &RoutingCase) -> RoutingOutcome {
        let router = LocalRouter;
        let tier = router.classify(&case.prompt);
        let selected = router.route(&tier);
        if selected == case.expected_route {
            RoutingOutcome::Correct
        } else {
            RoutingOutcome::WrongRoute { selected }
        }
    }

    pub fn run_all(&self) -> (usize, usize) {
        let total = self.cases.len();
        let passed = self
            .cases
            .iter()
            .filter(|c| self.evaluate(c) == RoutingOutcome::Correct)
            .count();
        (passed, total)
    }
}
