use std::collections::HashMap;

/// Records the dependency graph between tool calls in a multi-step agent execution.
/// A -> B means B depends on the output of A.
pub struct CallGraph {
    edges: HashMap<String, Vec<String>>,
}

impl CallGraph {
    pub fn new() -> Self {
        Self {
            edges: HashMap::new(),
        }
    }

    pub fn add_dependency(&mut self, call_id: &str, depends_on: &str) {
        self.edges
            .entry(call_id.to_string())
            .or_default()
            .push(depends_on.to_string());
    }

    pub fn dependencies_of(&self, call_id: &str) -> Vec<&str> {
        self.edges
            .get(call_id)
            .map(|v| v.iter().map(|s| s.as_str()).collect())
            .unwrap_or_default()
    }

    pub fn can_run_parallel(&self, call_a: &str, call_b: &str) -> bool {
        let a_deps = self.dependencies_of(call_a);
        let b_deps = self.dependencies_of(call_b);
        !a_deps.contains(&call_b) && !b_deps.contains(&call_a)
    }

    pub fn has_cycle(&self) -> bool {
        let nodes: Vec<&String> = self.edges.keys().collect();
        for node in &nodes {
            if self.is_reachable(node, node, &mut Default::default()) {
                return true;
            }
        }
        false
    }

    fn is_reachable(
        &self,
        start: &str,
        target: &str,
        visited: &mut std::collections::HashSet<String>,
    ) -> bool {
        if let Some(deps) = self.edges.get(start) {
            for dep in deps {
                if dep == target {
                    return true;
                }
                if visited.insert(dep.clone()) && self.is_reachable(dep, target, visited) {
                    return true;
                }
            }
        }
        false
    }
}

impl Default for CallGraph {
    fn default() -> Self {
        Self::new()
    }
}
