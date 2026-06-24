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

/// Single agent node runs to completion.
pub const SINGLE_AGENT: ConformanceScenario = ConformanceScenario {
    id: "single-agent",
    description: "A single agent node runs to completion without error",
    tags: &["agent", "basic"],
};

/// Agent and verifier nodes where verifier depends on agent output.
pub const MULTI_AGENT_VERIFIER: ConformanceScenario = ConformanceScenario {
    id: "multi-agent-verifier",
    description: "An agent node and a verifier node with an explicit dependency",
    tags: &["agent", "verifier", "graph"],
};

/// Run suspends for human input and resumes after approval.
pub const HUMAN_IN_LOOP: ConformanceScenario = ConformanceScenario {
    id: "human-in-loop",
    description: "A run suspends awaiting human approval and then resumes correctly",
    tags: &["suspend", "resume", "human"],
};

/// Run journal survives a simulated process restart and replays correctly.
pub const CRASH_AND_RECOVER: ConformanceScenario = ConformanceScenario {
    id: "crash-and-recover",
    description: "A run journal persists across restart and replays deterministically",
    tags: &["journal", "replay", "recovery"],
};
