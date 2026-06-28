/// panels - Node configuration panels: read and write node properties.

use crate::scaffold::Id;
use std::collections::HashMap;

/// A single configuration field descriptor.
#[derive(Debug, Clone)]
pub struct FieldDescriptor {
    pub key: String,
    pub label: String,
    pub field_type: FieldType,
    pub default_value: Option<String>,
    pub required: bool,
    pub hint: Option<String>,
}

impl FieldDescriptor {
    pub fn new(key: impl Into<String>, label: impl Into<String>, field_type: FieldType) -> Self {
        FieldDescriptor {
            key: key.into(),
            label: label.into(),
            field_type,
            default_value: None,
            required: false,
            hint: None,
        }
    }

    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }

    pub fn with_default(mut self, v: impl Into<String>) -> Self {
        self.default_value = Some(v.into());
        self
    }

    pub fn with_hint(mut self, h: impl Into<String>) -> Self {
        self.hint = Some(h.into());
        self
    }
}

/// Supported UI field types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldType {
    Text,
    Number,
    Bool,
    Select(Vec<String>),
    TextArea,
    Secret,
}

/// A configuration panel is a list of fields associated with a node kind.
#[derive(Debug, Clone)]
pub struct ConfigPanel {
    pub node_kind: String,
    pub title: String,
    pub fields: Vec<FieldDescriptor>,
}

impl ConfigPanel {
    pub fn new(node_kind: impl Into<String>, title: impl Into<String>) -> Self {
        ConfigPanel {
            node_kind: node_kind.into(),
            title: title.into(),
            fields: Vec::new(),
        }
    }

    pub fn add_field(mut self, field: FieldDescriptor) -> Self {
        self.fields.push(field);
        self
    }

    /// Return the descriptor for a given key, if present.
    pub fn field(&self, key: &str) -> Option<&FieldDescriptor> {
        self.fields.iter().find(|f| f.key == key)
    }
}

/// Registry of all config panels keyed by node kind.
#[derive(Debug, Default, Clone)]
pub struct PanelRegistry {
    panels: HashMap<String, ConfigPanel>,
}

impl PanelRegistry {
    pub fn new() -> Self {
        PanelRegistry::default()
    }

    /// Build a registry with default panels for all built-in node kinds.
    pub fn default_registry() -> Self {
        let mut r = PanelRegistry::new();

        r.register(
            ConfigPanel::new("agent.llm", "LLM Agent Settings")
                .add_field(FieldDescriptor::new("model", "Model", FieldType::Text).required().with_default("default"))
                .add_field(FieldDescriptor::new("temperature", "Temperature", FieldType::Number).with_default("0.7").with_hint("0.0 = deterministic, 1.0 = creative"))
                .add_field(FieldDescriptor::new("system_prompt", "System Prompt", FieldType::TextArea)),
        );

        r.register(
            ConfigPanel::new("agent.retrieval", "Retrieval Agent Settings")
                .add_field(FieldDescriptor::new("top_k", "Top-K Results", FieldType::Number).with_default("5"))
                .add_field(FieldDescriptor::new("store_url", "Vector Store URL", FieldType::Text).required()),
        );

        r.register(
            ConfigPanel::new("tool.web_search", "Web Search Settings")
                .add_field(FieldDescriptor::new("max_results", "Max Results", FieldType::Number).with_default("10"))
                .add_field(FieldDescriptor::new("safe_search", "Safe Search", FieldType::Bool).with_default("true")),
        );

        r.register(
            ConfigPanel::new("tool.code_exec", "Code Executor Settings")
                .add_field(FieldDescriptor::new("language", "Language", FieldType::Select(vec!["python".into(), "javascript".into(), "bash".into()])).with_default("python"))
                .add_field(FieldDescriptor::new("timeout_ms", "Timeout (ms)", FieldType::Number).with_default("5000")),
        );

        r.register(
            ConfigPanel::new("verifier.json_schema", "JSON Schema Verifier")
                .add_field(FieldDescriptor::new("schema", "JSON Schema", FieldType::TextArea).required()),
        );

        r.register(
            ConfigPanel::new("verifier.hallucination", "Hallucination Detector")
                .add_field(FieldDescriptor::new("threshold", "Threshold", FieldType::Number).with_default("0.5")),
        );

        r.register(
            ConfigPanel::new("control.router", "Router Settings")
                .add_field(FieldDescriptor::new("strategy", "Strategy", FieldType::Select(vec!["first_match".into(), "round_robin".into(), "random".into()]))),
        );

        r.register(
            ConfigPanel::new("control.loop", "Loop Settings")
                .add_field(FieldDescriptor::new("max_iterations", "Max Iterations", FieldType::Number).with_default("10").required())
                .add_field(FieldDescriptor::new("condition", "Break Condition", FieldType::Text).with_hint("Expression that returns true to break the loop")),
        );

        r
    }

    pub fn register(&mut self, panel: ConfigPanel) {
        self.panels.insert(panel.node_kind.clone(), panel);
    }

    pub fn get(&self, node_kind: &str) -> Option<&ConfigPanel> {
        self.panels.get(node_kind)
    }
}

/// In-progress edit session for a single node's configuration.
#[derive(Debug, Clone)]
pub struct PanelEditSession {
    pub node_id: Id,
    pub node_kind: String,
    /// Current values (key -> value string).
    pub values: HashMap<String, String>,
    /// Validation messages (key -> error message).
    pub errors: HashMap<String, String>,
}

impl PanelEditSession {
    pub fn new(node_id: Id, node_kind: impl Into<String>, initial: HashMap<String, String>) -> Self {
        PanelEditSession {
            node_id,
            node_kind: node_kind.into(),
            values: initial,
            errors: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: impl Into<String>, value: impl Into<String>) {
        let k = key.into();
        self.errors.remove(&k);
        self.values.insert(k, value.into());
    }

    /// Validate required fields against the panel descriptor.
    pub fn validate(&mut self, registry: &PanelRegistry) -> bool {
        self.errors.clear();
        if let Some(panel) = registry.get(&self.node_kind) {
            for field in &panel.fields {
                if field.required {
                    let val = self.values.get(&field.key).map(|v| v.trim()).unwrap_or("");
                    if val.is_empty() {
                        self.errors
                            .insert(field.key.clone(), format!("{} is required", field.label));
                    }
                }
            }
        }
        self.errors.is_empty()
    }

    pub fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }
}

#[cfg(test)]
mod unit {
    use super::*;

    #[test]
    fn registry_lookup() {
        let r = PanelRegistry::default_registry();
        let panel = r.get("agent.llm").expect("llm panel missing");
        assert!(panel.field("model").is_some());
    }

    #[test]
    fn validate_missing_required() {
        let r = PanelRegistry::default_registry();
        let mut session = PanelEditSession::new(Id::new("n1"), "agent.llm", HashMap::new());
        let ok = session.validate(&r);
        assert!(!ok);
        assert!(session.errors.contains_key("model"));
    }

    #[test]
    fn validate_with_required_field() {
        let r = PanelRegistry::default_registry();
        let mut values = HashMap::new();
        values.insert("model".into(), "gpt-4".into());
        let mut session = PanelEditSession::new(Id::new("n1"), "agent.llm", values);
        let ok = session.validate(&r);
        assert!(ok);
    }
}
