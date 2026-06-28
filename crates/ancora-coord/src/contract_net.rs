/// A bid from an agent in the contract-net protocol.
#[derive(Debug, Clone)]
pub struct Bid {
    pub agent_id: String,
    pub task_id: String,
    pub score: f64,
}

/// Runs contract-net task assignment by selecting the highest-scoring bid.
pub struct ContractNet;

impl ContractNet {
    pub fn assign(bids: &[Bid]) -> Option<&Bid> {
        bids.iter().max_by(|a, b| a.score.partial_cmp(&b.score).unwrap_or(std::cmp::Ordering::Equal))
    }
}
