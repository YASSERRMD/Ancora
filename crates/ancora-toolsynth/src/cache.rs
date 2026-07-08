use crate::spec::ToolSpec;
use std::collections::HashMap;

/// Caches synthesized tools by goal for reuse across invocations.
#[derive(Debug, Default)]
pub struct SynthCache {
    by_goal: HashMap<String, ToolSpec>,
}

impl SynthCache {
    pub fn insert(&mut self, goal: &str, spec: ToolSpec) {
        self.by_goal.insert(goal.to_string(), spec);
    }

    pub fn get(&self, goal: &str) -> Option<&ToolSpec> {
        self.by_goal.get(goal)
    }

    pub fn len(&self) -> usize {
        self.by_goal.len()
    }

    pub fn is_empty(&self) -> bool {
        self.by_goal.is_empty()
    }
}
