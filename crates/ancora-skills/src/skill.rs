use serde_json::Value;

/// Descriptor for a reusable skill.
#[derive(Debug, Clone)]
pub struct SkillDescriptor {
    pub name: String,
    pub version: u32,
    pub description: String,
    pub capability_tags: Vec<String>,
    pub input_schema: Value,
    pub permission_scope: SkillScope,
}

/// Permission scope for a skill.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SkillScope {
    ReadOnly,
    LocalWrite,
    Unrestricted,
}

impl SkillDescriptor {
    pub fn new(name: &str, version: u32, description: &str, tags: Vec<&str>, scope: SkillScope) -> Self {
        Self {
            name: name.to_string(),
            version,
            description: description.to_string(),
            capability_tags: tags.into_iter().map(|s| s.to_string()).collect(),
            input_schema: serde_json::json!({ "type": "object" }),
            permission_scope: scope,
        }
    }

    pub fn has_tag(&self, tag: &str) -> bool {
        self.capability_tags.iter().any(|t| t == tag)
    }
}
