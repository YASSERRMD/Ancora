//! Graph builder API for constructing custom task graphs.
//!
//! Plugin authors can use this API to define directed acyclic graphs
//! of tasks that the Ancora runtime will schedule.

use std::collections::{HashMap, HashSet};

/// Identifier for a node in the task graph.
pub type NodeId = String;

/// A single node in the task graph.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphNode {
    pub id: NodeId,
    pub label: String,
}

/// A directed task graph.
#[derive(Debug, Default)]
pub struct TaskGraph {
    nodes: HashMap<NodeId, GraphNode>,
    edges: Vec<(NodeId, NodeId)>,
}

impl TaskGraph {
    /// Create a new empty task graph.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a node. Returns an error if the id already exists.
    pub fn add_node(&mut self, node: GraphNode) -> Result<(), String> {
        if self.nodes.contains_key(&node.id) {
            return Err(format!("node '{}' already exists", node.id));
        }
        self.nodes.insert(node.id.clone(), node);
        Ok(())
    }

    /// Add a directed edge from `from` to `to`.
    /// Returns an error if either node does not exist.
    pub fn add_edge(&mut self, from: &str, to: &str) -> Result<(), String> {
        if !self.nodes.contains_key(from) {
            return Err(format!("node '{from}' not found"));
        }
        if !self.nodes.contains_key(to) {
            return Err(format!("node '{to}' not found"));
        }
        self.edges.push((from.to_string(), to.to_string()));
        Ok(())
    }

    /// Returns `true` if the graph contains a cycle.
    pub fn has_cycle(&self) -> bool {
        let mut visited: HashSet<&str> = HashSet::new();
        let mut in_stack: HashSet<&str> = HashSet::new();

        let adj: HashMap<&str, Vec<&str>> = {
            let mut m: HashMap<&str, Vec<&str>> = HashMap::new();
            for (from, to) in &self.edges {
                m.entry(from.as_str()).or_default().push(to.as_str());
            }
            m
        };

        fn dfs<'a>(
            node: &'a str,
            adj: &HashMap<&'a str, Vec<&'a str>>,
            visited: &mut HashSet<&'a str>,
            in_stack: &mut HashSet<&'a str>,
        ) -> bool {
            if in_stack.contains(node) {
                return true;
            }
            if visited.contains(node) {
                return false;
            }
            visited.insert(node);
            in_stack.insert(node);
            for &neighbor in adj.get(node).map(|v| v.as_slice()).unwrap_or(&[]) {
                if dfs(neighbor, adj, visited, in_stack) {
                    return true;
                }
            }
            in_stack.remove(node);
            false
        }

        for id in self.nodes.keys() {
            if dfs(id.as_str(), &adj, &mut visited, &mut in_stack) {
                return true;
            }
        }
        false
    }

    /// Returns the number of nodes.
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn node(id: &str) -> GraphNode {
        GraphNode { id: id.into(), label: id.into() }
    }

    #[test]
    fn simple_dag_has_no_cycle() {
        let mut g = TaskGraph::new();
        g.add_node(node("a")).unwrap();
        g.add_node(node("b")).unwrap();
        g.add_edge("a", "b").unwrap();
        assert!(!g.has_cycle());
    }

    #[test]
    fn cyclic_graph_detected() {
        let mut g = TaskGraph::new();
        g.add_node(node("x")).unwrap();
        g.add_node(node("y")).unwrap();
        g.add_edge("x", "y").unwrap();
        g.add_edge("y", "x").unwrap();
        assert!(g.has_cycle());
    }

    #[test]
    fn duplicate_node_fails() {
        let mut g = TaskGraph::new();
        g.add_node(node("dup")).unwrap();
        assert!(g.add_node(node("dup")).is_err());
    }
}
