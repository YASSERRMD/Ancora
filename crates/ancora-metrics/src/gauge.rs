use std::collections::HashMap;

/// Queue depth gauge: current number of queued runs per tenant.
#[derive(Default)]
pub struct QueueDepthGauge {
    depths: HashMap<String, i64>,
}

impl QueueDepthGauge {
    pub fn set(&mut self, tenant: &str, depth: i64) {
        self.depths.insert(tenant.into(), depth);
    }

    pub fn get(&self, tenant: &str) -> i64 {
        self.depths.get(tenant).copied().unwrap_or(0)
    }
}

/// Worker utilization gauge: ratio of busy workers to total workers.
#[derive(Default)]
pub struct WorkerUtilizationGauge {
    busy: u32,
    total: u32,
}

impl WorkerUtilizationGauge {
    pub fn set(&mut self, busy: u32, total: u32) {
        self.busy = busy;
        self.total = total;
    }

    pub fn utilization(&self) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            self.busy as f64 / self.total as f64
        }
    }

    pub fn busy(&self) -> u32 {
        self.busy
    }
    pub fn total(&self) -> u32 {
        self.total
    }
}
