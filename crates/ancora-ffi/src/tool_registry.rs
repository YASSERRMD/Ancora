use std::collections::{HashMap, HashSet};

use crate::tool_callback::AncorToolCallback;

/// Stores named tool callbacks registered by the host.
#[derive(Default)]
pub struct ToolRegistry {
    callbacks: HashMap<String, AncorToolCallback>,
    /// Names of tools that must be approved by a human before dispatch.
    /// See `ancora_tool_register_requires_approval`.
    approval_required: HashSet<String>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, name: impl Into<String>, cb: AncorToolCallback) {
        self.callbacks.insert(name.into(), cb);
    }

    /// Register a tool callback and flag it as requiring human approval
    /// before every dispatch -- the agent loop suspends instead of calling
    /// this callback (see `ancora_run_resume`).
    pub fn register_requires_approval(&mut self, name: impl Into<String>, cb: AncorToolCallback) {
        let name = name.into();
        self.approval_required.insert(name.clone());
        self.callbacks.insert(name, cb);
    }

    pub fn unregister(&mut self, name: &str) {
        self.callbacks.remove(name);
        self.approval_required.remove(name);
    }

    pub fn get(&self, name: &str) -> Option<AncorToolCallback> {
        self.callbacks.get(name).copied()
    }

    pub fn requires_approval(&self, name: &str) -> bool {
        self.approval_required.contains(name)
    }

    pub fn count(&self) -> usize {
        self.callbacks.len()
    }

    pub fn contains(&self, name: &str) -> bool {
        self.callbacks.contains_key(name)
    }
}
