pub mod bounds;
pub mod cooldown;
pub mod decision;
pub mod metrics;
pub mod perf;
pub mod policy;
pub mod signals;
pub mod simulator;
pub mod tenant_policy;

#[cfg(test)]
mod tests;

pub use policy::{ScalePolicy, ScaleDirection};
pub use decision::ScaleDecision;
