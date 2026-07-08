#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LhError {
    DeadlineExceeded { run_id: String, at: u64 },
    Throttled { ops_this_tick: u32, max: u32 },
}

impl std::fmt::Display for LhError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LhError::DeadlineExceeded { run_id, at } => {
                write!(f, "run {run_id} deadline exceeded at tick {at}")
            }
            LhError::Throttled { ops_this_tick, max } => {
                write!(f, "throttled: {ops_this_tick}/{max} ops this tick")
            }
        }
    }
}
