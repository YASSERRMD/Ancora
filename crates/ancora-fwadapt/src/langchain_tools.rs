//! Adapters for importing LangChain tool definitions into Ancora.
//!
//! LangChain tools carry a name, description, and a callable string-in/string-out
//! interface. This module wraps them in the Ancora tool model without any network I/O.

#[derive(Debug, Clone, PartialEq)]
pub struct LangchainToolDef {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct AncoraToolAdapter {
    pub tool_name: String,
    pub tool_description: String,
    handler: fn(&str) -> Result<String, ToolAdapterError>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ToolAdapterError {
    pub message: String,
}

impl std::fmt::Display for ToolAdapterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ToolAdapterError: {}", self.message)
    }
}

impl AncoraToolAdapter {
    /// Create an Ancora tool adapter from a LangChain tool definition and a handler function.
    pub fn from_langchain(
        def: LangchainToolDef,
        handler: fn(&str) -> Result<String, ToolAdapterError>,
    ) -> Self {
        Self {
            tool_name: def.name,
            tool_description: def.description,
            handler,
        }
    }

    /// Execute the wrapped tool with the given input.
    pub fn run(&self, input: &str) -> Result<String, ToolAdapterError> {
        (self.handler)(input)
    }
}

/// Convert a list of LangChain tool definitions into Ancora adapters using a
/// shared echo-style handler (useful for migration testing).
pub fn import_langchain_tools(defs: Vec<LangchainToolDef>) -> Vec<AncoraToolAdapter> {
    defs.into_iter()
        .map(|d| {
            AncoraToolAdapter::from_langchain(d, |input| Ok(format!("ancora-echo: {}", input)))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_langchain_tool() {
        let def = LangchainToolDef {
            name: "search".to_string(),
            description: "Web search tool".to_string(),
        };
        let adapter = AncoraToolAdapter::from_langchain(def, |i| Ok(i.to_uppercase()));
        assert_eq!(adapter.tool_name, "search");
        assert_eq!(adapter.run("hello").unwrap(), "HELLO");
    }
}
