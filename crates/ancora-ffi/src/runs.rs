use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

use ancora_core::agent::{Agent, ModelClient};
use ancora_core::journal::JournalStore;
use ancora_proto::ancora::AgentSpec;

use crate::model_client::ModelBackend;
use crate::tool_dispatch::FfiToolDispatcher;
use crate::tool_registry::ToolRegistry;

/// Internal state for a single run, populated by driving a real
/// `ancora_core::agent::Agent` loop to completion.
pub(crate) struct InnerRun {
    pub id: String,
    pub events: VecDeque<String>,
    pub cost_usd: f64,
}

impl InnerRun {
    /// Decode and run an agent spec to completion synchronously, collecting
    /// FFI-level lifecycle events (`started`, `tool_call`*, `completed` or
    /// `failed`) for `ancora_run_poll` to drain.
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

        match outcome {
            Ok(text) => {
                events.push_back(
                    serde_json::json!({"kind": "completed", "run_id": id, "output": text})
                        .to_string(),
                );
            }
            Err(err) => {
                events.push_back(
                    serde_json::json!({"kind": "failed", "run_id": id, "error": err.to_string()})
                        .to_string(),
                );
            }
        }

        let cost_usd = *cost_sink.lock().unwrap();
        InnerRun {
            id: id.to_owned(),
            events,
            cost_usd,
        }
    }

    pub fn poll_event(&mut self) -> Option<String> {
        self.events.pop_front()
    }

    /// Append a synthetic `resumed` lifecycle pair for human-in-loop
    /// callers. `Agent::run_loop` runs to completion synchronously and does
    /// not yet support suspending mid-run, so there is no real paused state
    /// to resume from; this preserves the pre-existing FFI contract
    /// (`resumed` followed by `completed`) rather than changing behavior
    /// unrelated callers depend on. Wiring real suspend/resume through
    /// `ancora_core::suspend` is tracked separately.
    pub fn resume(&mut self, decision: &str) {
        self.events.push_back(
            serde_json::json!({"kind": "resumed", "run_id": &self.id, "decision": decision})
                .to_string(),
        );
        self.events
            .push_back(format!(r#"{{"kind":"completed","run_id":"{}"}}"#, self.id));
    }
}
