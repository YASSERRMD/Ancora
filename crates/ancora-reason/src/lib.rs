//! ancora-reason: structured reasoning and verification chains for the Ancora agent framework.
//!
//! Provides composable primitives for verifiable multi-step reasoning:
//! step decomposition, intermediate verification, tool-grounded fact checks,
//! contradiction detection, evidence tracking, confidence aggregation,
//! abstention, citation output, and a deterministic reasoning journal.

pub mod abstain;
pub mod citation;
pub mod confidence;
pub mod contradiction;
pub mod decompose;
pub mod evidence;
pub mod factcheck;
pub mod journal;
pub mod verify;

pub use abstain::AbstentionPolicy;
pub use citation::CitationStore;
pub use confidence::ConfidenceAggregator;
pub use contradiction::ContradictionDetector;
pub use decompose::{ReasoningStep, StepDecomposer, StepStatus};
pub use evidence::EvidenceStore;
pub use factcheck::{FactCheck, FactChecker};
pub use journal::{ReasoningEvent, ReasoningJournal};
pub use verify::{StepVerifier, VerificationResult};

#[cfg(test)]
mod tests;
