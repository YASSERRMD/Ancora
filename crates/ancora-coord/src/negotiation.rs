use crate::error::CoordError;

/// A single proposal in a negotiation round.
#[derive(Debug, Clone)]
pub struct Proposal {
    pub agent_id: String,
    pub value: i64,
}

/// Multi-round negotiation converging to a consensus value.
pub struct Negotiation {
    pub max_rounds: u32,
    pub current_round: u32,
    proposals: Vec<Proposal>,
}

impl Negotiation {
    pub fn new(max_rounds: u32) -> Self {
        Self { max_rounds, current_round: 0, proposals: Vec::new() }
    }

    pub fn submit(&mut self, proposal: Proposal) -> Result<(), CoordError> {
        if self.current_round >= self.max_rounds {
            return Err(CoordError::MaxRoundsExceeded { rounds: self.current_round });
        }
        self.proposals.push(proposal);
        self.current_round += 1;
        Ok(())
    }

    pub fn consensus(&self) -> Option<i64> {
        if self.proposals.is_empty() { return None; }
        let sum: i64 = self.proposals.iter().map(|p| p.value).sum();
        Some(sum / self.proposals.len() as i64)
    }

    pub fn converged(&self) -> bool {
        self.proposals.len() >= 2 && self.proposals.windows(2).all(|w| w[0].value == w[1].value)
    }
}
