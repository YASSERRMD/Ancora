use serde::{Deserialize, Serialize};
use serde_json::Value;

/// JSON Schema-lite representation for structured output validation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum JsonType {
    String,
    Number,
    Boolean,
    Array,
    Object,
    Null,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldSchema {
    pub name: String,
    pub json_type: JsonType,
    pub required: bool,
    pub description: String,
}

impl FieldSchema {
    pub fn new(name: &str, json_type: JsonType, required: bool) -> Self {
        Self { name: name.to_string(), json_type, required, description: String::new() }
    }

    pub fn with_description(mut self, desc: &str) -> Self {
        self.description = desc.to_string();
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputSchema {
    pub name: String,
    pub fields: Vec<FieldSchema>,
}

impl OutputSchema {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string(), fields: vec![] }
    }

    pub fn add_field(mut self, field: FieldSchema) -> Self {
        self.fields.push(field);
        self
    }

    pub fn required_fields(&self) -> Vec<&FieldSchema> {
        self.fields.iter().filter(|f| f.required).collect()
    }

    pub fn to_json_schema(&self) -> Value {
        let props: serde_json::Map<String, Value> = self.fields
            .iter()
            .map(|f| {
                let type_str = match f.json_type {
                    JsonType::String => "string",
                    JsonType::Number => "number",
                    JsonType::Boolean => "boolean",
                    JsonType::Array => "array",
                    JsonType::Object => "object",
                    JsonType::Null => "null",
                };
                (f.name.clone(), serde_json::json!({ "type": type_str, "description": f.description }))
            })
            .collect();

        let required: Vec<&str> = self.required_fields().iter().map(|f| f.name.as_str()).collect();

        serde_json::json!({
            "type": "object",
            "name": self.name,
            "properties": props,
            "required": required,
        })
    }
}
