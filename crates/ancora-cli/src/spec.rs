use serde::{Deserialize, Serialize};

use crate::store::open_store;
use crate::trace::print_trace;

/// A node entry in the spec file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeSpec {
    pub id: String,
    pub kind: String,
    pub model: Option<String>,
    #[serde(default)]
    pub depends_on: Vec<String>,
}

/// Top-level graph spec file format.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphSpec {
    pub name: String,
    #[serde(default)]
    pub nodes: Vec<NodeSpec>,
}

/// Error types for spec validation.
#[derive(Debug, thiserror::Error)]
pub enum SpecError {
    #[error("duplicate node id: {0}")]
    DuplicateNodeId(String),
    #[error("unknown dependency '{dep}' in node '{node}'")]
    UnknownDependency { node: String, dep: String },
}

impl GraphSpec {
    /// Validate that all node IDs are unique and dependencies exist.
    pub fn validate(&self) -> Result<(), SpecError> {
        let ids: std::collections::HashSet<&str> =
            self.nodes.iter().map(|n| n.id.as_str()).collect();
        let mut seen = std::collections::HashSet::new();
        for node in &self.nodes {
            if !seen.insert(node.id.as_str()) {
                return Err(SpecError::DuplicateNodeId(node.id.clone()));
            }
            for dep in &node.depends_on {
                if !ids.contains(dep.as_str()) {
                    return Err(SpecError::UnknownDependency {
                        node: node.id.clone(),
                        dep: dep.clone(),
                    });
                }
            }
        }
        Ok(())
    }
}

/// Load and run a graph spec from a YAML file.
pub fn run_graph(path: &str, store_kind: &str) -> Result<(), Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(path)?;
    let spec: GraphSpec = serde_yaml::from_str(&content)?;
    let store = open_store(store_kind)?;
    println!("ancora: running graph '{}'", spec.name);
    for node in &spec.nodes {
        println!("  node {} ({})", node.id, node.kind);
    }
    println!("ancora: run complete");
    print_trace(&spec, store.as_ref());
    Ok(())
}
