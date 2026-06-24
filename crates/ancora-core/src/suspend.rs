use crate::error::AncoraError;

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

impl SuspendedRun {
    /// Serialize to a JSON string for durable storage.
    pub fn to_json(&self) -> Result<String, AncoraError> {
        let v = serde_json::json!({
            "run_id": self.run_id,
            "node_id": self.node_id,
            "pending_input": self.pending_input,
            "deadline_ms": self.deadline_ms,
        });
        serde_json::to_string(&v).map_err(|e| AncoraError::Storage(e.to_string()))
    }

    /// Deserialize from a JSON string produced by `to_json`.
    pub fn from_json(s: &str) -> Result<Self, AncoraError> {
        let v: serde_json::Value = serde_json::from_str(s)
            .map_err(|e| AncoraError::Storage(e.to_string()))?;
        let run_id = v["run_id"].as_str()
            .ok_or_else(|| AncoraError::Storage("missing run_id".to_string()))?
            .to_string();
        let node_id = v["node_id"].as_str()
            .ok_or_else(|| AncoraError::Storage("missing node_id".to_string()))?
            .to_string();
        let pending_input = v["pending_input"].as_str()
            .ok_or_else(|| AncoraError::Storage("missing pending_input".to_string()))?
            .to_string();
        let deadline_ms = match &v["deadline_ms"] {
            serde_json::Value::Null => None,
            n => Some(n.as_u64()
                .ok_or_else(|| AncoraError::Storage("deadline_ms must be a u64".to_string()))?),
        };
        Ok(SuspendedRun { run_id, node_id, pending_input, deadline_ms })
    }
}
