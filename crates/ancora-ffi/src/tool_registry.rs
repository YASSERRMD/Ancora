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
}
