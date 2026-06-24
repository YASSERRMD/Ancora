use std::collections::HashMap;

/// Resolves which model to use for each node in a graph.
pub struct ModelRouter {
    default_model: String,
    bindings: HashMap<String, String>,
}

impl ModelRouter {
    pub fn new(default_model: impl Into<String>) -> Self {
        Self { default_model: default_model.into(), bindings: HashMap::new() }
    }

    /// Bind a specific model to a node.
    pub fn bind(&mut self, node_id: impl Into<String>, model_id: impl Into<String>) -> &mut Self {
        self.bindings.insert(node_id.into(), model_id.into());
        self
    }

    /// Bind a smaller, cheaper model to a set of node ids in one call.
    pub fn bind_small(&mut self, node_ids: &[&str], small_model: impl Into<String>) -> &mut Self {
        let model = small_model.into();
        for id in node_ids {
            self.bindings.insert((*id).to_owned(), model.clone());
        }
        self
    }

    /// Resolve which model to use for a node.
    /// Priority: explicit binding > node.model_id override > default.
    pub fn resolve<'a>(&'a self, node_id: &str, node_model_id: Option<&'a str>) -> &'a str {
        if let Some(m) = self.bindings.get(node_id) {
            return m.as_str();
        }
        if let Some(m) = node_model_id {
            if !m.is_empty() {
                return m;
            }
        }
        self.default_model.as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binding_overrides_node_model_id() {
        let mut router = ModelRouter::new("big-model");
        router.bind("node-a", "small-model");
        assert_eq!(router.resolve("node-a", Some("other")), "small-model");
    }

    #[test]
    fn node_model_id_used_when_no_binding() {
        let router = ModelRouter::new("big-model");
        assert_eq!(router.resolve("node-b", Some("node-specific")), "node-specific");
    }

    #[test]
    fn default_model_used_when_no_binding_and_no_node_model() {
        let router = ModelRouter::new("big-model");
        assert_eq!(router.resolve("node-c", None), "big-model");
    }

    #[test]
    fn bind_small_assigns_small_model_to_multiple_nodes() {
        let mut router = ModelRouter::new("large");
        router.bind_small(&["simple-1", "simple-2"], "small");
        assert_eq!(router.resolve("simple-1", None), "small");
        assert_eq!(router.resolve("simple-2", None), "small");
        assert_eq!(router.resolve("complex", None), "large");
    }
}
