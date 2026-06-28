#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CoordError {
    MaxRoundsExceeded { rounds: u32 },
    NoCycleFound,
    ContractViolation { contract_id: String, obligation: String },
}

impl std::fmt::Display for CoordError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CoordError::MaxRoundsExceeded { rounds } => write!(f, "max rounds exceeded: {rounds}"),
            CoordError::NoCycleFound => write!(f, "no cycle found to break"),
            CoordError::ContractViolation { contract_id, obligation } => {
                write!(f, "contract {contract_id} violation: obligation '{obligation}' not fulfilled")
            }
        }
    }
}
