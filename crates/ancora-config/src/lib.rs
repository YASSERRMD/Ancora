pub mod error;
pub mod schema;
pub mod validator;
pub mod layer;
pub mod hot_reload;
pub mod tenant_overlay;
pub mod secret_provider;
pub mod env_provider;
pub mod file_provider;
pub mod external_provider;
pub mod resolver;
pub mod rotation;
pub mod redact;

#[cfg(test)]
mod tests;

pub use error::ConfigError;
pub use schema::{AncoraCfg, CoreCfg, JournalCfg, WorkerCfg, TelemetryCfg};
pub use validator::validate;
pub use layer::ConfigLayers;
pub use hot_reload::HotReloadState;
pub use tenant_overlay::TenantOverlayRegistry;
pub use secret_provider::{SecretProvider, SecretRef};
pub use env_provider::EnvSecretProvider;
pub use file_provider::FileSecretProvider;
pub use external_provider::ExternalSecretProvider;
pub use resolver::SecretResolver;
pub use rotation::{RotationLog, RotationRecord};
pub use redact::redacted_dump;
