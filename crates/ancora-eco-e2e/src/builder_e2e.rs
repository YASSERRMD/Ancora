/// Builder end-to-end: graph builder for plugin pipelines.

use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NodeId(pub u64);

#[derive(Debug, Clone)]
pub struct Node {
    pub id: NodeId,
    pub label: String,
    pub plugin_name: String,
}

impl Node {
    pub fn new(id: u64, label: &str, plugin_name: &str) -> Self {
        Node {
            id: NodeId(id),
            label: label.to_string(),
            plugin_name: plugin_name.to_string(),
        }
    }
}

#[derive(Debug, Default)]
pub struct PluginGraph {
    nodes: HashMap<u64, Node>,
    edges: Vec<(u64, u64)>,
}

impl PluginGraph {
    pub fn new() -> Self {
        PluginGraph {
            nodes: HashMap::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node: Node) -> Result<(), String> {
        let id = node.id.0;
        if self.nodes.contains_key(&id) {
            return Err(format!("node {} already exists", id));
        }
        self.nodes.insert(id, node);
        Ok(())
    }

    pub fn add_edge(&mut self, from: u64, to: u64) -> Result<(), String> {
        if !self.nodes.contains_key(&from) {
            return Err(format!("node {} not found", from));
        }
        if !self.nodes.contains_key(&to) {
            return Err(format!("node {} not found", to));
        }
        if self.edges.contains(&(from, to)) {
            return Err(format!("edge ({}, {}) already exists", from, to));
        }
        self.edges.push((from, to));
        Ok(())
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    pub fn topological_order(&self) -> Result<Vec<u64>, String> {
        let mut in_degree: HashMap<u64, usize> = self.nodes.keys().map(|&k| (k, 0)).collect();
        for &(_, to) in &self.edges {
            *in_degree.entry(to).or_insert(0) += 1;
        }
        let mut queue: Vec<u64> = in_degree
            .iter()
            .filter(|(_, &deg)| deg == 0)
            .map(|(&id, _)| id)
            .collect();
        queue.sort();
        let mut result = Vec::new();
        let mut visited: HashSet<u64> = HashSet::new();
        while !queue.is_empty() {
            queue.sort();
            let node = queue.remove(0);
            result.push(node);
            visited.insert(node);
            for &(from, to) in &self.edges {
                if from == node {
                    let deg = in_degree.entry(to).or_insert(1);
                    if *deg > 0 {
                        *deg -= 1;
                    }
                    if *deg == 0 && !visited.contains(&to) {
                        queue.push(to);
                    }
                }
            }
        }
        if result.len() != self.nodes.len() {
            return Err("graph contains a cycle".to_string());
        }
        Ok(result)
    }

    pub fn has_cycle(&self) -> bool {
        self.topological_order().is_err()
    }
}
