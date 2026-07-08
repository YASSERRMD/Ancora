use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DepStatus {
    Healthy,
    Degraded { reason: String },
    Down { reason: String },
}

pub struct DependencyHealth {
    statuses: HashMap<String, DepStatus>,
}

impl DependencyHealth {
    pub fn new() -> Self {
        Self {
            statuses: HashMap::new(),
        }
    }

    pub fn report(&mut self, name: &str, status: DepStatus) {
        self.statuses.insert(name.to_string(), status);
    }

    pub fn is_all_healthy(&self) -> bool {
        self.statuses
            .values()
            .all(|s| matches!(s, DepStatus::Healthy))
    }

    pub fn degraded_count(&self) -> usize {
        self.statuses
            .values()
            .filter(|s| matches!(s, DepStatus::Degraded { .. }))
            .count()
    }

    pub fn down_count(&self) -> usize {
        self.statuses
            .values()
            .filter(|s| matches!(s, DepStatus::Down { .. }))
            .count()
    }

    pub fn get(&self, name: &str) -> Option<&DepStatus> {
        self.statuses.get(name)
    }
}

impl Default for DependencyHealth {
    fn default() -> Self {
        Self::new()
    }
}
