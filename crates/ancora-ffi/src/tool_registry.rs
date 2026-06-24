use std::collections::HashMap;

use crate::tool_callback::AncorToolCallback;

/// Stores named tool callbacks registered by the host.
#[derive(Default)]
pub struct ToolRegistry {
    callbacks: HashMap<String, AncorToolCallback>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, name: impl Into<String>, cb: AncorToolCallback) {
        self.callbacks.insert(name.into(), cb);
    }

    pub fn unregister(&mut self, name: &str) {
        self.callbacks.remove(name);
    }

    pub fn get(&self, name: &str) -> Option<AncorToolCallback> {
        self.callbacks.get(name).copied()
    }

    pub fn count(&self) -> usize {
        self.callbacks.len()
    }

    pub fn contains(&self, name: &str) -> bool {
        self.callbacks.contains_key(name)
    }
}
