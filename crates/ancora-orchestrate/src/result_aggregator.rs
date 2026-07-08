use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct AgentResult {
    pub task_id: String,
    pub agent_id: String,
    pub output: Value,
    pub success: bool,
}

pub struct ResultAggregator {
    results: HashMap<String, AgentResult>,
}

impl ResultAggregator {
    pub fn new() -> Self {
        Self {
            results: HashMap::new(),
        }
    }

    pub fn record(&mut self, result: AgentResult) {
        self.results.insert(result.task_id.clone(), result);
    }

    pub fn get(&self, task_id: &str) -> Option<&AgentResult> {
        self.results.get(task_id)
    }

    pub fn successful_count(&self) -> usize {
        self.results.values().filter(|r| r.success).count()
    }

    pub fn failed_count(&self) -> usize {
        self.results.values().filter(|r| !r.success).count()
    }

    pub fn merge_outputs(&self, task_ids: &[&str]) -> Value {
        let mut merged = serde_json::Map::new();
        for id in task_ids {
            if let Some(r) = self.get(id) {
                merged.insert(r.task_id.clone(), r.output.clone());
            }
        }
        Value::Object(merged)
    }
}

impl Default for ResultAggregator {
    fn default() -> Self {
        Self::new()
    }
}
