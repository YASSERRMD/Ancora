/// Bridge between Microsoft Semantic Kernel plugins and Ancora.
///
/// Semantic Kernel plugins are collections of functions with metadata (name,
/// description, parameters). This module converts SK plugin descriptors into
/// Ancora tool definitions and vice versa.

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct SKFunctionParam {
    pub name: String,
    pub description: String,
    pub default_value: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SKFunctionDef {
    pub name: String,
    pub description: String,
    pub params: Vec<SKFunctionParam>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SKPluginDef {
    pub plugin_name: String,
    pub functions: Vec<SKFunctionDef>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AncoraSkToolSpec {
    pub qualified_name: String,
    pub description: String,
    pub param_defaults: HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SKBridgeError {
    EmptyPlugin,
    DuplicateFunction(String),
}

impl std::fmt::Display for SKBridgeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyPlugin => write!(f, "plugin has no functions"),
            Self::DuplicateFunction(n) => write!(f, "duplicate function: {}", n),
        }
    }
}

/// Convert a Semantic Kernel plugin into a list of Ancora tool specs.
pub fn import_sk_plugin(
    plugin: SKPluginDef,
) -> Result<Vec<AncoraSkToolSpec>, SKBridgeError> {
    if plugin.functions.is_empty() {
        return Err(SKBridgeError::EmptyPlugin);
    }

    let mut seen: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut specs = Vec::new();

    for func in plugin.functions {
        let qname = format!("{}.{}", plugin.plugin_name, func.name);
        if !seen.insert(func.name.clone()) {
            return Err(SKBridgeError::DuplicateFunction(func.name));
        }
        let defaults: HashMap<String, String> = func
            .params
            .iter()
            .filter_map(|p| p.default_value.clone().map(|v| (p.name.clone(), v)))
            .collect();
        specs.push(AncoraSkToolSpec {
            qualified_name: qname,
            description: func.description,
            param_defaults: defaults,
        });
    }

    Ok(specs)
}

/// Expose an Ancora tool spec as a Semantic Kernel function descriptor string.
pub fn export_to_sk_descriptor(spec: &AncoraSkToolSpec) -> String {
    let parts: Vec<String> = spec
        .param_defaults
        .iter()
        .map(|(k, v)| format!("{}={}", k, v))
        .collect();
    format!(
        "SK[{}]: {} (defaults: {})",
        spec.qualified_name,
        spec.description,
        if parts.is_empty() {
            "none".to_string()
        } else {
            parts.join(", ")
        }
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn import_sk_plugin_succeeds() {
        let plugin = SKPluginDef {
            plugin_name: "TextPlugin".into(),
            functions: vec![SKFunctionDef {
                name: "summarize".into(),
                description: "Summarize text".into(),
                params: vec![SKFunctionParam {
                    name: "length".into(),
                    description: "Max words".into(),
                    default_value: Some("100".into()),
                }],
            }],
        };
        let specs = import_sk_plugin(plugin).unwrap();
        assert_eq!(specs.len(), 1);
        assert_eq!(specs[0].qualified_name, "TextPlugin.summarize");
        assert_eq!(specs[0].param_defaults.get("length").unwrap(), "100");
    }
}
