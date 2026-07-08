pub mod dedup;
pub mod episodic;
pub mod forgetting;
pub mod job;
pub mod journal;
pub mod retrieval;
pub mod salience;
pub mod summarizer;
pub mod token_budget;

#[cfg(test)]
mod tests;

pub use dedup::Deduplicator;
pub use episodic::{EpisodicEntry, EpisodicToSemanticPromoter, SemanticEntry};
pub use forgetting::ForgettingPolicy;
pub use job::{ConsolidationJob, ConsolidationOutput};
pub use journal::{ConsolidationEvent, ConsolidationJournal, JournalEntry};
pub use retrieval::RetrievalChecker;
pub use salience::{SalienceItem, SalienceScorer};
pub use summarizer::{ConversationSummarizer, SummarizationPolicy, SummaryResult, Turn};
pub use token_budget::TokenBudget;
