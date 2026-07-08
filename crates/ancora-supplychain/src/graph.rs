use std::collections::{HashMap, HashSet};

pub struct DependencyGraph {
    edges: HashMap<String, HashSet<String>>,
}

impl DependencyGraph {
    pub fn new() -> Self {
        Self {
            edges: HashMap::new(),
        }
    }

    pub fn add_dependency(
        &mut self,
        component_id: impl Into<String>,
        depends_on: impl Into<String>,
    ) {
        self.edges
            .entry(component_id.into())
            .or_default()
            .insert(depends_on.into());
    }

    pub fn direct_dependencies(&self, component_id: &str) -> Vec<&str> {
        self.edges
            .get(component_id)
            .map(|s| s.iter().map(String::as_str).collect())
            .unwrap_or_default()
    }

    pub fn has_dependency(&self, component_id: &str, depends_on: &str) -> bool {
        self.edges
            .get(component_id)
            .map_or(false, |s| s.contains(depends_on))
    }

    pub fn transitive_dependencies(&self, component_id: &str) -> HashSet<String> {
        let mut visited = HashSet::new();
        self.collect_transitive(component_id, &mut visited);
        visited
    }

    fn collect_transitive(&self, id: &str, visited: &mut HashSet<String>) {
        if let Some(deps) = self.edges.get(id) {
            for dep in deps {
                if visited.insert(dep.clone()) {
                    self.collect_transitive(dep, visited);
                }
            }
        }
    }

    pub fn component_count(&self) -> usize {
        self.edges.len()
    }
}
