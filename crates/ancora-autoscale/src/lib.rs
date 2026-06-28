pub mod metrics;
pub mod policy;
pub mod cooldown;
pub mod bounds;
pub mod simulator;
pub mod decision;
pub mod signals;

#[cfg(test)]
mod tests;

pub use policy::{ScalePolicy, ScaleDirection};
pub use decision::ScaleDecision;
