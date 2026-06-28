pub mod blackboard;
pub mod contract_net;
pub mod auction;
pub mod negotiation;
pub mod conflict;
pub mod deadlock;
pub mod journal;
pub mod contract;
pub mod error;

#[cfg(test)]
mod tests;

pub use blackboard::{Blackboard, BlackboardError};
pub use contract_net::{Bid, ContractNet};
pub use auction::Auction;
pub use negotiation::{Negotiation, Proposal};
pub use conflict::{Claim, ConflictPolicy, ConflictResolver};
pub use deadlock::DeadlockDetector;
pub use journal::{CoordJournal, CoordEvent};
pub use contract::AgentContract;
pub use error::CoordError;
