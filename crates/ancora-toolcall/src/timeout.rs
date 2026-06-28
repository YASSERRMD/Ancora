use std::collections::HashMap;

pub struct ToolTimeoutTracker {
    /// Maps call_id to (deadline_secs, tool_name)
    deadlines: HashMap<String, (u64, String)>,
}

impl ToolTimeoutTracker {
    pub fn new() -> Self {
        Self { deadlines: HashMap::new() }
    }

    pub fn register(&mut self, call_id: &str, tool_name: &str, started_at: u64, timeout_ms: u64) {
        let deadline = started_at + timeout_ms / 1000;
        self.deadlines.insert(call_id.to_string(), (deadline, tool_name.to_string()));
    }

    pub fn is_timed_out(&self, call_id: &str, now: u64) -> bool {
        self.deadlines.get(call_id).map(|(d, _)| now >= *d).unwrap_or(false)
    }

    pub fn timed_out_calls(&self, now: u64) -> Vec<(&str, &str)> {
        self.deadlines
            .iter()
            .filter(|(_, (d, _))| now >= *d)
            .map(|(id, (_, name))| (id.as_str(), name.as_str()))
            .collect()
    }

    pub fn remove(&mut self, call_id: &str) {
        self.deadlines.remove(call_id);
    }
}

impl Default for ToolTimeoutTracker {
    fn default() -> Self {
        Self::new()
    }
}
