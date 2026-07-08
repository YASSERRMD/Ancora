use crate::key::HsmKey;
use std::collections::HashMap;

pub struct HsmStats {
    pub total_keys: usize,
    pub by_algorithm: HashMap<String, usize>,
    pub extractable_count: usize,
    pub sensitive_count: usize,
}

impl HsmStats {
    pub fn from_keys(keys: &[&HsmKey]) -> Self {
        let total_keys = keys.len();
        let extractable_count = keys.iter().filter(|k| k.extractable).count();
        let sensitive_count = keys.iter().filter(|k| k.sensitive).count();
        let mut by_algorithm = HashMap::new();
        for k in keys {
            *by_algorithm.entry(format!("{}", k.algorithm)).or_insert(0) += 1;
        }
        Self {
            total_keys,
            by_algorithm,
            extractable_count,
            sensitive_count,
        }
    }

    pub fn sensitive_ratio(&self) -> f64 {
        if self.total_keys == 0 {
            return 0.0;
        }
        self.sensitive_count as f64 / self.total_keys as f64
    }
}
