use std::collections::HashMap;
use crate::schema::ToolDef;
use crate::error::ToolError;

pub struct ToolRegistry {
    tools: HashMap<String, ToolDef>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self { tools: HashMap::new() }
    }

    pub fn register(&mut self, def: ToolDef) {
        self.tools.insert(def.name.clone(), def);
    }

    pub fn get(&self, name: &str) -> Option<&ToolDef> {
        self.tools.get(name)
    }

    pub fn names(&self) -> Vec<&str> {
        self.tools.keys().map(|s| s.as_str()).collect()
    }

    pub fn count(&self) -> usize {
        self.tools.len()
    }

    pub fn validate_call(&self, tool_name: &str) -> Result<&ToolDef, ToolError> {
        self.get(tool_name).ok_or_else(|| ToolError::UnknownTool { name: tool_name.to_string() })
    }
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}
