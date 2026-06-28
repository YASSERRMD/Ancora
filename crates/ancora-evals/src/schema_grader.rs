/// A simple JSON-schema-like rule set for validating structured outputs.
#[derive(Debug, Clone)]
pub enum SchemaRule {
    /// The value must be a valid JSON object (starts with `{` and ends with `}`).
    IsObject,
    /// The value must contain the given key.
    HasKey(String),
    /// The value must be parseable as a JSON array.
    IsArray,
    /// The string length must not exceed the given maximum.
    MaxLength(usize),
    /// The value must not be empty.
    NonEmpty,
}

/// Result of a schema validation.
#[derive(Debug, Clone, PartialEq)]
pub struct SchemaResult {
    /// True when all rules pass.
    pub passed: bool,
    /// Which rules failed (if any).
    pub violations: Vec<String>,
}

/// Grader that validates an output against a set of schema rules.
#[derive(Debug, Clone, Default)]
pub struct SchemaGrader {
    rules: Vec<SchemaRule>,
}

impl SchemaGrader {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a rule to the grader.
    pub fn with_rule(mut self, rule: SchemaRule) -> Self {
        self.rules.push(rule);
        self
    }

    /// Validate the candidate string against all registered rules.
    pub fn validate(&self, candidate: &str) -> SchemaResult {
        let trimmed = candidate.trim();
        let mut violations = Vec::new();

        for rule in &self.rules {
            match rule {
                SchemaRule::IsObject => {
                    if !(trimmed.starts_with('{') && trimmed.ends_with('}')) {
                        violations.push("Expected a JSON object".to_string());
                    }
                }
                SchemaRule::HasKey(key) => {
                    let needle = format!("\"{}\"", key);
                    if !trimmed.contains(&needle) {
                        violations.push(format!("Missing key: {}", key));
                    }
                }
                SchemaRule::IsArray => {
                    if !(trimmed.starts_with('[') && trimmed.ends_with(']')) {
                        violations.push("Expected a JSON array".to_string());
                    }
                }
                SchemaRule::MaxLength(max) => {
                    if trimmed.len() > *max {
                        violations.push(format!(
                            "Value length {} exceeds maximum {}",
                            trimmed.len(),
                            max
                        ));
                    }
                }
                SchemaRule::NonEmpty => {
                    if trimmed.is_empty() {
                        violations.push("Value must not be empty".to_string());
                    }
                }
            }
        }

        SchemaResult {
            passed: violations.is_empty(),
            violations,
        }
    }
}
