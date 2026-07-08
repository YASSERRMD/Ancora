pub mod auction;
pub mod blackboard;
pub mod conflict;
pub mod contract;
pub mod contract_net;
pub mod deadlock;
pub mod error;
pub mod journal;
pub mod negotiation;

#[cfg(test)]
mod tests;

pub use auction::Auction;
pub use blackboard::{Blackboard, BlackboardError};
pub use conflict::{Claim, ConflictPolicy, ConflictResolver};
pub use contract::AgentContract;
pub use contract_net::{Bid, ContractNet};
pub use deadlock::DeadlockDetector;
pub use error::CoordError;
pub use journal::{CoordEvent, CoordJournal};
pub use negotiation::{Negotiation, Proposal};
