/// The outcome of running a graph that may suspend at an AwaitHuman node.
pub enum RunOutcome {
    /// The graph ran to completion and returned the final output.
    Completed(String),
    /// Execution paused at an AwaitHuman node and is waiting for a decision.
    Suspended(SuspendedRun),
}

/// Captures the minimal state needed to resume a run after a human decision.
pub struct SuspendedRun {
    pub run_id: String,
    pub node_id: String,
    pub pending_input: String,
    pub deadline_ms: Option<u64>,
}
