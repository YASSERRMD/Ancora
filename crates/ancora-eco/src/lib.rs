//! ancora-eco: Extension ecosystem versioning, negotiation, deprecation, and governance.

pub mod stability_policy;
pub mod semver;
pub mod version_negotiation;
pub mod deprecation;
pub mod compat_matrix;
pub mod breaking_detector;
pub mod lifecycle;
pub mod governance;
pub mod rfc;
pub mod maintainer;
pub mod security_disclosure;
pub mod conduct;

#[cfg(test)]
mod tests;
