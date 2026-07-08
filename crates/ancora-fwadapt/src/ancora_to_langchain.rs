//! Expose an Ancora agent as a LangChain-compatible tool.
//!
//! This module produces a LangChain tool descriptor from an Ancora agent
//! specification so that Python LangChain runtimes can call the Ancora agent
//! via its string interface (name + description + invoke string).

#[derive(Debug, Clone, PartialEq)]
pub struct AncoraAgentSpec {
    pub id: String,
    pub display_name: String,
    pub capability_summary: String,
    /// Endpoint URL (or IPC address) the LangChain tool should POST to.
    pub endpoint: String,
}

/// A LangChain tool descriptor produced from an Ancora agent.
#[derive(Debug, Clone, PartialEq)]
pub struct LangchainToolDescriptor {
    pub name: String,
    pub description: String,
    /// The return type hint string as LangChain expects it.
    pub return_type: String,
    pub endpoint: String,
}

/// Convert an Ancora agent spec into a LangChain tool descriptor.
pub fn expose_as_langchain_tool(spec: &AncoraAgentSpec) -> LangchainToolDescriptor {
    LangchainToolDescriptor {
        name: spec.id.replace('-', "_"),
        description: format!(
            "[Ancora agent: {}] {}",
            spec.display_name, spec.capability_summary
        ),
        return_type: "str".to_string(),
        endpoint: spec.endpoint.clone(),
    }
}

/// Render the descriptor as a minimal Python snippet that registers it in
/// LangChain. No network I/O - this is a pure code-generation helper.
pub fn render_langchain_python_snippet(desc: &LangchainToolDescriptor) -> String {
    format!(
        r#"from langchain.tools import Tool
import requests

def _{name}_invoke(query: str) -> str:
    resp = requests.post("{endpoint}", json={{"input": query}})
    resp.raise_for_status()
    return resp.json().get("output", "")

{name}_tool = Tool(
    name="{name}",
    func=_{name}_invoke,
    description="{description}",
)
"#,
        name = desc.name,
        endpoint = desc.endpoint,
        description = desc.description.replace('"', "'"),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expose_produces_valid_descriptor() {
        let spec = AncoraAgentSpec {
            id: "data-analyst".into(),
            display_name: "Data Analyst".into(),
            capability_summary: "Analyses tabular data".into(),
            endpoint: "http://localhost:8080/invoke".into(),
        };
        let desc = expose_as_langchain_tool(&spec);
        assert_eq!(desc.name, "data_analyst");
        assert!(desc.description.contains("Data Analyst"));
        assert_eq!(desc.endpoint, "http://localhost:8080/invoke");
    }

    #[test]
    fn snippet_contains_tool_name() {
        let desc = LangchainToolDescriptor {
            name: "my_tool".into(),
            description: "does stuff".into(),
            return_type: "str".into(),
            endpoint: "http://host/run".into(),
        };
        let snippet = render_langchain_python_snippet(&desc);
        assert!(snippet.contains("my_tool"));
        assert!(snippet.contains("http://host/run"));
    }
}
