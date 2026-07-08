use serde_json::Value;

/// Describes a dynamically synthesized tool.
#[derive(Debug, Clone)]
pub struct ToolSpec {
    pub name: String,
    pub description: String,
    pub input_schema: Value,
    pub effect_class: EffectClass,
}

/// Classification of what side-effects a tool may have.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EffectClass {
    ReadOnly,
    WriteLocal,
    WriteExternal,
    Destructive,
}

impl ToolSpec {
    pub fn new(
        name: &str,
        description: &str,
        input_schema: Value,
        effect_class: EffectClass,
    ) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            input_schema,
            effect_class,
        }
    }
}

/// Generates a ToolSpec from a natural-language goal.
pub fn spec_from_goal(goal: &str) -> ToolSpec {
    ToolSpec::new(
        &goal.to_lowercase().replace(' ', "_"),
        goal,
        serde_json::json!({ "type": "object", "properties": {} }),
        EffectClass::ReadOnly,
    )
}
