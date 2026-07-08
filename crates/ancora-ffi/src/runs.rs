use std::collections::VecDeque;

/// Internal state for a single run.
pub(crate) struct InnerRun {
    pub id: String,
    pub events: VecDeque<String>,
    pub cost_usd: f64,
}

impl InnerRun {
    pub fn new(id: &str, agent_spec: &str) -> Self {
        let mut events = VecDeque::new();
        events.push_back(
            serde_json::json!({"kind": "started", "run_id": id, "spec": agent_spec}).to_string(),
        );
        events.push_back(format!(r#"{{"kind":"completed","run_id":"{}"}}"#, id));
        InnerRun {
            id: id.to_string(),
            events,
            cost_usd: 0.0,
        }
    }

    pub fn poll_event(&mut self) -> Option<String> {
        self.events.pop_front()
    }

    pub fn resume(&mut self, decision: &str) {
        self.events.push_back(
            serde_json::json!({"kind": "resumed", "run_id": &self.id, "decision": decision})
                .to_string(),
        );
        self.events
            .push_back(format!(r#"{{"kind":"completed","run_id":"{}"}}"#, self.id));
    }
}
