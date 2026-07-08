use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Namespace {
    tenant_id: String,
    data: HashMap<String, String>,
}

impl Namespace {
    pub fn new(tenant_id: impl Into<String>) -> Self {
        Self {
            tenant_id: tenant_id.into(),
            data: HashMap::new(),
        }
    }

    pub fn tenant_id(&self) -> &str {
        &self.tenant_id
    }

    pub fn scoped_key(&self, key: &str) -> String {
        format!("{}::{}", self.tenant_id, key)
    }

    pub fn set(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.data.insert(key.into(), value.into());
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.data.get(key).map(|s| s.as_str())
    }

    pub fn remove(&mut self, key: &str) -> Option<String> {
        self.data.remove(key)
    }

    pub fn contains(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }

    pub fn count(&self) -> usize {
        self.data.len()
    }

    pub fn keys(&self) -> Vec<&str> {
        let mut keys: Vec<&str> = self.data.keys().map(|k| k.as_str()).collect();
        keys.sort();
        keys
    }

    pub fn is_isolated_from(&self, other: &Namespace) -> bool {
        self.tenant_id != other.tenant_id
    }
}
