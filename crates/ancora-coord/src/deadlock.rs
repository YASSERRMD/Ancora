use crate::error::CoordError;
use std::collections::{HashMap, HashSet};

/// Detects and breaks deadlocks in agent wait-for graphs.
pub struct DeadlockDetector {
    wait_for: HashMap<String, String>,
}

impl DeadlockDetector {
    pub fn new() -> Self {
        Self {
            wait_for: HashMap::new(),
        }
    }

    pub fn add_wait(&mut self, waiter: &str, waited_on: &str) {
        self.wait_for
            .insert(waiter.to_string(), waited_on.to_string());
    }

    pub fn has_deadlock(&self) -> bool {
        for start in self.wait_for.keys() {
            let mut visited = HashSet::new();
            let mut current = start.as_str();
            loop {
                if !visited.insert(current) {
                    return true;
                }
                match self.wait_for.get(current) {
                    Some(next) => current = next.as_str(),
                    None => break,
                }
            }
        }
        false
    }

    pub fn break_cycle(&mut self) -> Result<String, CoordError> {
        let victim = self.wait_for.keys().next().cloned();
        if let Some(v) = victim {
            self.wait_for.remove(&v);
            Ok(v)
        } else {
            Err(CoordError::NoCycleFound)
        }
    }
}

impl Default for DeadlockDetector {
    fn default() -> Self {
        Self::new()
    }
}
