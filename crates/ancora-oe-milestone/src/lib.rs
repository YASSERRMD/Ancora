//! ancora-oe-milestone: Observability and eval release checkpoint.
//!
//! This crate consolidates the obs and eval milestone status, documents
//! parity, feature matrix, known limitations, upgrade notes, changelog,
//! quickstarts, privacy posture, self-hosted summaries, announcements,
//! readiness checklists, and the metrics/evals catalog index.

pub mod suite_status;
pub mod parity_status;
pub mod feature_matrix;
pub mod limitations;
pub mod upgrade_notes;
pub mod changelog;
pub mod quickstarts;
pub mod privacy_summary;
pub mod self_hosted_summary;
pub mod announcement;
pub mod readiness;
pub mod catalog_index;

#[cfg(test)]
mod tests;
