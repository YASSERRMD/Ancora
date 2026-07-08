use serde_json::Value;
use std::collections::HashMap;

/// In-memory fake Kubernetes API for envtest-style operator testing.
#[derive(Default)]
pub struct FakeK8s {
    resources: HashMap<String, Value>,
}

impl FakeK8s {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn apply(&mut self, kind: &str, name: &str, resource: Value) {
        let key = format!("{}/{}", kind, name);
        self.resources.insert(key, resource);
    }

    pub fn get(&self, kind: &str, name: &str) -> Option<&Value> {
        let key = format!("{}/{}", kind, name);
        self.resources.get(&key)
    }

    pub fn delete(&mut self, kind: &str, name: &str) -> bool {
        let key = format!("{}/{}", kind, name);
        self.resources.remove(&key).is_some()
    }

    pub fn list_kind(&self, kind: &str) -> Vec<(&str, &Value)> {
        let prefix = format!("{}/", kind);
        self.resources
            .iter()
            .filter(|(k, _)| k.starts_with(&prefix))
            .map(|(k, v)| (k.as_str(), v))
            .collect()
    }

    pub fn exists(&self, kind: &str, name: &str) -> bool {
        self.get(kind, name).is_some()
    }
}
