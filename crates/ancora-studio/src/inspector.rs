/// Step inspector - shows prompt, response, and tool call details for a single step.

#[derive(Debug, Clone)]
pub struct ToolCallDetail {
    pub name: String,
    pub input_json: String,
    pub output_json: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Clone)]
pub struct InspectorData {
    pub run_id: String,
    pub step_index: usize,
    pub prompt: Option<String>,
    pub response: Option<String>,
    pub tool_calls: Vec<ToolCallDetail>,
    pub metadata: std::collections::HashMap<String, String>,
    pub redacted_fields: Vec<String>,
}

impl InspectorData {
    pub fn new(run_id: impl Into<String>, step_index: usize) -> Self {
        Self {
            run_id: run_id.into(),
            step_index,
            prompt: None,
            response: None,
            tool_calls: Vec::new(),
            metadata: std::collections::HashMap::new(),
            redacted_fields: Vec::new(),
        }
    }

    pub fn with_prompt(mut self, prompt: impl Into<String>) -> Self {
        self.prompt = Some(prompt.into());
        self
    }

    pub fn with_response(mut self, response: impl Into<String>) -> Self {
        self.response = Some(response.into());
        self
    }

    pub fn add_tool_call(&mut self, call: ToolCallDetail) {
        self.tool_calls.push(call);
    }

    pub fn is_field_redacted(&self, field: &str) -> bool {
        self.redacted_fields.iter().any(|f| f == field)
    }

    pub fn visible_prompt(&self) -> Option<&str> {
        if self.is_field_redacted("prompt") {
            None
        } else {
            self.prompt.as_deref()
        }
    }

    pub fn visible_response(&self) -> Option<&str> {
        if self.is_field_redacted("response") {
            None
        } else {
            self.response.as_deref()
        }
    }

    pub fn set_metadata(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.metadata.insert(key.into(), value.into());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visible_fields_no_redaction() {
        let data = InspectorData::new("r1", 0)
            .with_prompt("hello")
            .with_response("world");
        assert_eq!(data.visible_prompt(), Some("hello"));
        assert_eq!(data.visible_response(), Some("world"));
    }

    #[test]
    fn test_redacted_prompt() {
        let mut data = InspectorData::new("r1", 0).with_prompt("secret");
        data.redacted_fields.push("prompt".into());
        assert_eq!(data.visible_prompt(), None);
    }

    #[test]
    fn test_tool_calls() {
        let mut data = InspectorData::new("r1", 0);
        data.add_tool_call(ToolCallDetail {
            name: "search".into(),
            input_json: r#"{"q":"rust"}"#.into(),
            output_json: Some(r#"{"results":[]}"#.into()),
            error: None,
        });
        assert_eq!(data.tool_calls.len(), 1);
        assert_eq!(data.tool_calls[0].name, "search");
    }
}
