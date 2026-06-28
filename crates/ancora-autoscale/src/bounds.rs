use serde::{Deserialize, Serialize};

/// Min/max bounds for worker count.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ScaleBounds {
    pub min_workers: usize,
    pub max_workers: usize,
}

impl ScaleBounds {
    pub fn new(min: usize, max: usize) -> Self {
        assert!(min <= max, "min must be <= max");
        ScaleBounds {
            min_workers: min,
            max_workers: max,
        }
    }

    pub fn clamp(&self, desired: usize) -> usize {
        desired.max(self.min_workers).min(self.max_workers)
    }

    pub fn at_max(&self, current: usize) -> bool {
        current >= self.max_workers
    }

    pub fn at_min(&self, current: usize) -> bool {
        current <= self.min_workers
    }
}

/// Per-tenant cap on workers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantCap {
    pub tenant_id: String,
    pub max_workers: usize,
}
