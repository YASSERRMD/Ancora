/// Catalog end-to-end: tool catalog with install, remove, and lookup.

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct CatalogTool {
    pub name: String,
    pub description: String,
    pub plugin_id: u64,
}

impl CatalogTool {
    pub fn new(name: &str, description: &str, plugin_id: u64) -> Self {
        CatalogTool {
            name: name.to_string(),
            description: description.to_string(),
            plugin_id,
        }
    }
}

#[derive(Debug, Default)]
pub struct ToolCatalog {
    tools: HashMap<String, CatalogTool>,
}

impl ToolCatalog {
    pub fn new() -> Self {
        ToolCatalog {
            tools: HashMap::new(),
        }
    }

    pub fn install(&mut self, tool: CatalogTool) -> Result<(), String> {
        if tool.name.is_empty() {
            return Err("tool name must not be empty".to_string());
        }
        if self.tools.contains_key(&tool.name) {
            return Err(format!("tool '{}' already installed", tool.name));
        }
        self.tools.insert(tool.name.clone(), tool);
        Ok(())
    }

    pub fn remove(&mut self, name: &str) -> bool {
        self.tools.remove(name).is_some()
    }

    pub fn get(&self, name: &str) -> Option<&CatalogTool> {
        self.tools.get(name)
    }

    pub fn list(&self) -> Vec<&CatalogTool> {
        let mut tools: Vec<&CatalogTool> = self.tools.values().collect();
        tools.sort_by_key(|t| &t.name);
        tools
    }

    pub fn count(&self) -> usize {
        self.tools.len()
    }
}
