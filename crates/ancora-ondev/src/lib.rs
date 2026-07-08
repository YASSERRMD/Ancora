//! On-device runtime for ARM and mobile targets.
//!
//! Provides a minimal, offline-only agent runtime designed for
//! ARM64, ARM32, Android JNI, and iOS C-ABI deployment targets.

pub mod android;
pub mod build_profile;
pub mod docs_meta;
pub mod features;
pub mod footprint;
pub mod inference;
pub mod ios;
pub mod journal;
pub mod memory;
pub mod perf;
pub mod targets;

#[cfg(test)]
mod tests;
