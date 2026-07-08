use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProviderStatus {
    Active,
    Failed,
    Failover,
}

pub struct ProviderFailover {
    statuses: HashMap<String, ProviderStatus>,
    priority: Vec<String>,
}

impl ProviderFailover {
    pub fn new(providers_in_priority_order: Vec<String>) -> Self {
        let mut statuses = HashMap::new();
        for p in &providers_in_priority_order {
            statuses.insert(p.clone(), ProviderStatus::Active);
        }
        Self {
            statuses,
            priority: providers_in_priority_order,
        }
    }

    pub fn mark_failed(&mut self, provider: &str) {
        if let Some(s) = self.statuses.get_mut(provider) {
            *s = ProviderStatus::Failed;
        }
    }

    pub fn active_provider(&self) -> Option<&str> {
        self.priority
            .iter()
            .find(|p| self.statuses.get(p.as_str()) == Some(&ProviderStatus::Active))
            .map(|s| s.as_str())
    }

    pub fn failover(&mut self) -> Option<&str> {
        let current = self.active_provider().map(|s| s.to_string());
        if let Some(cur) = current {
            if let Some(s) = self.statuses.get_mut(&cur) {
                *s = ProviderStatus::Failover;
            }
        }
        self.active_provider()
    }

    pub fn status(&self, provider: &str) -> Option<&ProviderStatus> {
        self.statuses.get(provider)
    }
}
