// ancora-fleet: edge fleet management for the Ancora agent framework

pub mod registration;
pub mod inventory;
pub mod config_push;
pub mod model_dist;
pub mod rollout;
pub mod telemetry;
pub mod policy;
pub mod decommission;
pub mod airgap;
pub mod dashboard;

#[cfg(test)]
mod tests;
