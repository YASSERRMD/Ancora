pub mod spec;
pub mod schema_validator;
pub mod sandbox;
pub mod permission;
pub mod approval;
pub mod registry;
pub mod audit;
pub mod cache;
pub mod error;

#[cfg(test)]
mod tests;

pub use spec::{ToolSpec, EffectClass, spec_from_goal};
pub use schema_validator::SchemaValidator;
pub use sandbox::SandboxRunner;
pub use permission::PermissionScope;
pub use approval::ApprovalGate;
pub use registry::SynthRegistry;
pub use audit::{SynthAudit, AuditEvent, AuditEntry};
pub use cache::SynthCache;
pub use error::SynthError;
