/// Adapter end-to-end: framework adapter that imports tools from plugins.
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ToolSpec {
    pub name: String,
    pub input_schema: String,
    pub output_schema: String,
    pub source_plugin: String,
}

impl ToolSpec {
    pub fn new(name: &str, input_schema: &str, output_schema: &str, source_plugin: &str) -> Self {
        ToolSpec {
            name: name.to_string(),
            input_schema: input_schema.to_string(),
            output_schema: output_schema.to_string(),
            source_plugin: source_plugin.to_string(),
        }
    }

    pub fn is_valid(&self) -> bool {
        !self.name.is_empty() && !self.source_plugin.is_empty()
    }
}

#[derive(Debug)]
pub struct FrameworkAdapter {
    pub framework_name: String,
    imported_tools: HashMap<String, ToolSpec>,
}

impl FrameworkAdapter {
    pub fn new(framework_name: &str) -> Self {
        FrameworkAdapter {
            framework_name: framework_name.to_string(),
            imported_tools: HashMap::new(),
        }
    }

    pub fn import_tool(&mut self, spec: ToolSpec) -> Result<(), String> {
        if !spec.is_valid() {
            return Err("tool spec is invalid".to_string());
        }
        if self.imported_tools.contains_key(&spec.name) {
            return Err(format!("tool '{}' already imported", spec.name));
        }
        self.imported_tools.insert(spec.name.clone(), spec);
        Ok(())
    }

    pub fn get_tool(&self, name: &str) -> Option<&ToolSpec> {
        self.imported_tools.get(name)
    }

    pub fn list_tools(&self) -> Vec<&ToolSpec> {
        let mut tools: Vec<&ToolSpec> = self.imported_tools.values().collect();
        tools.sort_by_key(|t| &t.name);
        tools
    }

    pub fn tool_count(&self) -> usize {
        self.imported_tools.len()
    }

    pub fn remove_tool(&mut self, name: &str) -> bool {
        self.imported_tools.remove(name).is_some()
    }
}
