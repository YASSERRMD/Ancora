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
