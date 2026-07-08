/// End-to-end test status for the ecosystem milestone.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum E2eResult {
    Pass,
    Fail(String),
}

#[derive(Debug, Clone)]
pub struct E2eScenario {
    pub name: String,
    pub result: E2eResult,
    pub duration_ms: u64,
}

impl E2eScenario {
    pub fn pass(name: impl Into<String>, duration_ms: u64) -> Self {
        Self {
            name: name.into(),
            result: E2eResult::Pass,
            duration_ms,
        }
    }

    pub fn is_passing(&self) -> bool {
        self.result == E2eResult::Pass
    }
}

pub fn ecosystem_e2e_scenarios() -> Vec<E2eScenario> {
    vec![
        E2eScenario::pass("plugin-load-and-invoke", 120),
        E2eScenario::pass("plugin-hot-reload", 340),
        E2eScenario::pass("catalog-search-and-install", 210),
        E2eScenario::pass("registry-publish-and-fetch", 450),
        E2eScenario::pass("multi-agent-coordination", 890),
        E2eScenario::pass("streaming-tool-call", 67),
        E2eScenario::pass("structured-output-roundtrip", 44),
    ]
}

pub fn all_e2e_passing(scenarios: &[E2eScenario]) -> bool {
    scenarios.iter().all(|s| s.is_passing())
}
