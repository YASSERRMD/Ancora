//! ancora-eco: Extension ecosystem versioning, negotiation, deprecation, and governance.

pub mod breaking_detector;
pub mod compat_matrix;
pub mod conduct;
pub mod deprecation;
pub mod governance;
pub mod lifecycle;
pub mod maintainer;
pub mod rfc;
pub mod security_disclosure;
pub mod semver;
pub mod stability_policy;
pub mod version_negotiation;

#[cfg(test)]
mod tests;
