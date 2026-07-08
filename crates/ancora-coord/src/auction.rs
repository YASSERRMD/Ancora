use crate::contract_net::Bid;

/// A sealed-bid auction that assigns tasks to winning bidders.
#[derive(Debug, Default)]
pub struct Auction {
    pub task_id: String,
    bids: Vec<Bid>,
}

impl Auction {
    pub fn new(task_id: &str) -> Self {
        Self {
            task_id: task_id.to_string(),
            bids: Vec::new(),
        }
    }

    pub fn submit(&mut self, bid: Bid) {
        self.bids.push(bid);
    }

    pub fn resolve(&self) -> Option<&Bid> {
        self.bids.iter().max_by(|a, b| {
            a.score
                .partial_cmp(&b.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
    }

    pub fn bid_count(&self) -> usize {
        self.bids.len()
    }
}
