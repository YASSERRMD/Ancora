pub mod approval;
pub mod audit;
pub mod cache;
pub mod error;
pub mod permission;
pub mod registry;
pub mod sandbox;
pub mod schema_validator;
pub mod spec;

#[cfg(test)]
mod tests;

pub use approval::ApprovalGate;
pub use audit::{AuditEntry, AuditEvent, SynthAudit};
pub use cache::SynthCache;
pub use error::SynthError;
pub use permission::PermissionScope;
pub use registry::SynthRegistry;
pub use sandbox::SandboxRunner;
pub use schema_validator::SchemaValidator;
pub use spec::{spec_from_goal, EffectClass, ToolSpec};
