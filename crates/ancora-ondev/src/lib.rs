//! On-device runtime for ARM and mobile targets.
//!
//! Provides a minimal, offline-only agent runtime designed for
//! ARM64, ARM32, Android JNI, and iOS C-ABI deployment targets.

pub mod android;
pub mod build_profile;
pub mod footprint;
pub mod ios;
pub mod targets;
pub mod features;
pub mod journal;
pub mod memory;
pub mod inference;
pub mod perf;
pub mod docs_meta;

#[cfg(test)]
mod tests;
