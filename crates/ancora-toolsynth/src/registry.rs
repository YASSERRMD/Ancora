use crate::error::SynthError;
use crate::spec::ToolSpec;
use std::collections::HashMap;

/// Registry of approved synthesized tools.
#[derive(Debug, Default)]
pub struct SynthRegistry {
    tools: HashMap<String, ToolSpec>,
}

impl SynthRegistry {
    pub fn register(&mut self, spec: ToolSpec) {
        self.tools.insert(spec.name.clone(), spec);
    }

    pub fn get(&self, name: &str) -> Option<&ToolSpec> {
        self.tools.get(name)
    }

    pub fn remove(&mut self, name: &str) -> Option<ToolSpec> {
        self.tools.remove(name)
    }

    pub fn len(&self) -> usize {
        self.tools.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tools.is_empty()
    }

    pub fn lookup(&self, name: &str) -> Result<&ToolSpec, SynthError> {
        self.tools
            .get(name)
            .ok_or_else(|| SynthError::NotFound(name.to_string()))
    }
}
