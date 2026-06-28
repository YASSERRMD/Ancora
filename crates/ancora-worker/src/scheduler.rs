use ancora_controlplane::model::RunPriority;

/// Priority lane: higher priority runs are served before lower ones.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Lane {
    Critical,
    High,
    Normal,
    Low,
}

impl Lane {
    pub fn from_priority(p: RunPriority) -> Self {
        match p {
            RunPriority::Critical => Lane::Critical,
            RunPriority::High => Lane::High,
            RunPriority::Normal => Lane::Normal,
            RunPriority::Low => Lane::Low,
        }
    }
}

/// Backpressure: signal emitted when the pool is saturated.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Backpressure {
    None,
    Soft,
    Hard,
}

pub fn backpressure(queue_depth: usize, worker_count: usize, concurrency: usize) -> Backpressure {
    let capacity = worker_count * concurrency;
    if capacity == 0 {
        return Backpressure::Hard;
    }
    let ratio = queue_depth as f64 / capacity as f64;
    if ratio >= 2.0 {
        Backpressure::Hard
    } else if ratio >= 1.0 {
        Backpressure::Soft
    } else {
        Backpressure::None
    }
}
