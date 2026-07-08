//! ancora-ab: Controlled A/B experiments for the Ancora agent framework.
//!
//! Provides experiment definition, traffic splitting, deterministic assignment,
//! exposure logging, metric collection, statistical analysis, guardrails,
//! lifecycle management, and structured reporting.

pub mod analysis;
pub mod assignment;
pub mod experiment;
pub mod exposure;
pub mod guardrail;
pub mod lifecycle;
pub mod outcome;
pub mod report;

#[cfg(test)]
mod tests;
