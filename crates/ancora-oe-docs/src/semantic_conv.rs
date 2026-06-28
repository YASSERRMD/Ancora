//! Semantic conventions for agent observability attributes.

/// Well-known attribute keys following OpenTelemetry semantic conventions.
pub struct SemanticAttributes;

impl SemanticAttributes {
    pub const AGENT_ID: &'static str = "ancora.agent.id";
    pub const AGENT_VERSION: &'static str = "ancora.agent.version";
    pub const LLM_PROVIDER: &'static str = "ancora.llm.provider";
    pub const LLM_MODEL: &'static str = "ancora.llm.model";
    pub const LLM_TOKENS_INPUT: &'static str = "ancora.llm.tokens.input";
    pub const LLM_TOKENS_OUTPUT: &'static str = "ancora.llm.tokens.output";
    pub const TOOL_NAME: &'static str = "ancora.tool.name";
    pub const TOOL_SUCCESS: &'static str = "ancora.tool.success";
    pub const EVAL_SCORE: &'static str = "ancora.eval.score";
    pub const EVAL_GRADER: &'static str = "ancora.eval.grader";
}

/// A key-value attribute pair.
#[derive(Debug, Clone, PartialEq)]
pub struct Attribute {
    pub key: String,
    pub value: AttributeValue,
}

/// Supported attribute value types.
#[derive(Debug, Clone, PartialEq)]
pub enum AttributeValue {
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
}

impl Attribute {
    pub fn string(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            value: AttributeValue::String(value.into()),
        }
    }

    pub fn int(key: impl Into<String>, value: i64) -> Self {
        Self {
            key: key.into(),
            value: AttributeValue::Int(value),
        }
    }

    pub fn float(key: impl Into<String>, value: f64) -> Self {
        Self {
            key: key.into(),
            value: AttributeValue::Float(value),
        }
    }

    pub fn bool(key: impl Into<String>, value: bool) -> Self {
        Self {
            key: key.into(),
            value: AttributeValue::Bool(value),
        }
    }
}

/// Validates that an attribute key is a known semantic convention.
pub fn is_known_key(key: &str) -> bool {
    [
        SemanticAttributes::AGENT_ID,
        SemanticAttributes::AGENT_VERSION,
        SemanticAttributes::LLM_PROVIDER,
        SemanticAttributes::LLM_MODEL,
        SemanticAttributes::LLM_TOKENS_INPUT,
        SemanticAttributes::LLM_TOKENS_OUTPUT,
        SemanticAttributes::TOOL_NAME,
        SemanticAttributes::TOOL_SUCCESS,
        SemanticAttributes::EVAL_SCORE,
        SemanticAttributes::EVAL_GRADER,
    ]
    .contains(&key)
}
