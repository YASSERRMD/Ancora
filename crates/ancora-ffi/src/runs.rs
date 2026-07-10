use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

use ancora_core::agent::{Agent, AgentOutcome, ModelClient};
use ancora_core::journal::JournalStore;
use ancora_proto::ancora::{AgentSpec, ToolResultContent};

use crate::model_client::ModelBackend;
use crate::tool_dispatch::FfiToolDispatcher;
use crate::tool_registry::ToolRegistry;

/// Internal state for a single run, populated by driving a real
/// `ancora_core::agent::Agent` loop to completion.
pub(crate) struct InnerRun {
    pub id: String,
    pub events: VecDeque<String>,
    pub cost_usd: f64,
    /// Live agent state, kept only while the run is suspended waiting on a
    /// human decision for an approval-gated tool call (see
    /// `ToolDispatcher::requires_approval`). `None` once the run has
    /// completed or failed -- there is nothing left to resume.
    agent: Option<Agent>,
    /// Shared with every model client adapter built for this run (at start
    /// and at each resume), so cost keeps accumulating across suspend
    /// boundaries instead of resetting.
    cost_sink: Arc<Mutex<f64>>,
}

impl InnerRun {
    /// Decode and run an agent spec, collecting FFI-level lifecycle events
    /// (`started`, `tool_call`*, `completed`/`failed`/`suspended`) for
    /// `ancora_run_poll` to drain. A `suspended` run keeps its `Agent` alive
    /// internally so `resume` can continue the exact same loop state later.
    ///
    /// `spec.instructions` doubles as the initial user input: the C ABI has
    /// no separate "input" parameter today, so this is the closest faithful
    /// mapping given how callers already use `instructions` as the whole
    /// prompt. A dedicated input parameter is a natural follow-up that would
    /// need a coordinated change across every language SDK.
    pub(crate) fn execute(
        id: &str,
        spec: AgentSpec,
        backend: &ModelBackend,
        tools: &Mutex<ToolRegistry>,
        journal: Arc<dyn JournalStore>,
    ) -> Self {
        let mut events = VecDeque::new();
        events.push_back(
            serde_json::json!({"kind": "started", "run_id": id, "model_id": spec.model_id})
                .to_string(),
        );

        let input = spec.instructions.clone();
        let cost_sink = Arc::new(Mutex::new(0.0));
        let model: Box<dyn ModelClient> = backend.make_adapter(Arc::clone(&cost_sink));
        let dispatcher = FfiToolDispatcher::new(tools, id);

        let mut agent = Agent::new(spec, id, journal);
        let outcome = agent.run_loop(&input, model.as_ref(), &dispatcher);

        for event in dispatcher.into_events() {
            events.push_back(event);
        }

        let agent_state = Self::apply_outcome(id, outcome, &mut events, agent);
        let cost_usd = *cost_sink.lock().unwrap();

        InnerRun {
            id: id.to_owned(),
            events,
            cost_usd,
            agent: agent_state,
            cost_sink,
        }
    }

    pub fn poll_event(&mut self) -> Option<String> {
        self.events.pop_front()
    }

    /// Resume a run with a human decision for the pending approval-gated
    /// tool call.
    ///
    /// If the run is genuinely suspended, this decodes `decision_bytes`
    /// into a `ToolResultContent` for the pending call and continues the
    /// exact same `Agent` loop state forward -- a real resume.
    ///
    /// If nothing is actually suspended (the run already completed/failed,
    /// or was never gated), this preserves the pre-existing FFI contract:
    /// a harmless `resumed`/`completed` event pair. That keeps every
    /// caller that resumes defensively or idempotently working unchanged,
    /// while runs that really do suspend now get genuine resume behavior.
    pub fn resume(
        &mut self,
        decision_bytes: &[u8],
        backend: &ModelBackend,
        tools: &Mutex<ToolRegistry>,
    ) {
        let Some(mut agent) = self.agent.take() else {
            self.events.push_back(
                serde_json::json!({
                    "kind": "resumed",
                    "run_id": &self.id,
                    "decision": String::from_utf8_lossy(decision_bytes),
                })
                .to_string(),
            );
            self.events
                .push_back(format!(r#"{{"kind":"completed","run_id":"{}"}}"#, self.id));
            return;
        };

        let tool_call_id = agent.pending_tool_call_id().unwrap_or_default().to_owned();
        let decision = decode_decision(decision_bytes, &tool_call_id);

        self.events.push_back(
            serde_json::json!({
                "kind": "resumed",
                "run_id": &self.id,
                "decision": String::from_utf8_lossy(decision_bytes),
            })
            .to_string(),
        );

        let model: Box<dyn ModelClient> = backend.make_adapter(Arc::clone(&self.cost_sink));
        let dispatcher = FfiToolDispatcher::new(tools, &self.id);
        let outcome = agent.resume(decision, model.as_ref(), &dispatcher);

        for event in dispatcher.into_events() {
            self.events.push_back(event);
        }

        let id = self.id.clone();
        self.agent = Self::apply_outcome(&id, outcome, &mut self.events, agent);
        self.cost_usd = *self.cost_sink.lock().unwrap();
    }

    /// Turn an `Agent::run_loop`/`resume` outcome into FFI lifecycle events,
    /// returning the `Agent` back (to keep alive) only if it suspended.
    fn apply_outcome(
        id: &str,
        outcome: Result<AgentOutcome, ancora_core::error::AncoraError>,
        events: &mut VecDeque<String>,
        agent: Agent,
    ) -> Option<Agent> {
        match outcome {
            Ok(AgentOutcome::Completed(text)) => {
                events.push_back(
                    serde_json::json!({"kind": "completed", "run_id": id, "output": text})
                        .to_string(),
                );
                None
            }
            Ok(AgentOutcome::Suspended { tool_call, prompt }) => {
                events.push_back(
                    serde_json::json!({
                        "kind": "suspended",
                        "run_id": id,
                        "tool_call_id": tool_call.tool_call_id,
                        "tool_name": tool_call.tool_name,
                        "arguments_json": tool_call.arguments_json,
                        "prompt": prompt,
                    })
                    .to_string(),
                );
                Some(agent)
            }
            Err(err) => {
                events.push_back(
                    serde_json::json!({"kind": "failed", "run_id": id, "error": err.to_string()})
                        .to_string(),
                );
                None
            }
        }
    }
}

/// Decode a resume decision from bytes into a `ToolResultContent` for
/// `tool_call_id`. Accepts `{"result_json": "...", "is_error": bool}`; any
/// other shape (plain text, bare JSON value, empty bytes) is wrapped as a
/// JSON string result with `is_error: false` rather than rejected, matching
/// this crate's established never-fail-on-malformed-input decoding style.
fn decode_decision(bytes: &[u8], tool_call_id: &str) -> ToolResultContent {
    if let Ok(serde_json::Value::Object(obj)) = serde_json::from_slice::<serde_json::Value>(bytes) {
        let result_json = obj
            .get("result_json")
            .and_then(|v| v.as_str())
            .map(|s| s.to_owned())
            .unwrap_or_else(|| serde_json::Value::Object(obj.clone()).to_string());
        let is_error = obj
            .get("is_error")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        return ToolResultContent {
            tool_call_id: tool_call_id.to_owned(),
            result_json,
            is_error,
        };
    }
    let text = String::from_utf8_lossy(bytes).into_owned();
    ToolResultContent {
        tool_call_id: tool_call_id.to_owned(),
        result_json: serde_json::Value::String(text).to_string(),
        is_error: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_decision_reads_structured_shape() {
        let bytes = br#"{"result_json":"\"approved\"","is_error":false}"#;
        let result = decode_decision(bytes, "tc-1");
        assert_eq!(result.tool_call_id, "tc-1");
        assert_eq!(result.result_json, "\"approved\"");
        assert!(!result.is_error);
    }

    #[test]
    fn decode_decision_reads_is_error_flag() {
        let bytes = br#"{"result_json":"\"denied\"","is_error":true}"#;
        let result = decode_decision(bytes, "tc-1");
        assert!(result.is_error);
    }

    #[test]
    fn decode_decision_wraps_plain_text_as_json_string() {
        let result = decode_decision(b"approve", "tc-1");
        assert_eq!(result.result_json, "\"approve\"");
        assert!(!result.is_error);
    }

    #[test]
    fn decode_decision_wraps_empty_bytes() {
        let result = decode_decision(b"", "tc-1");
        assert_eq!(result.result_json, "\"\"");
    }
}
