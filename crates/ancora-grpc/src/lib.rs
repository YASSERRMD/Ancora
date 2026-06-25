pub mod proto {
    tonic::include_proto!("ancora");
}

pub mod agent_card;
pub mod auth;
pub mod client;
pub mod identity;
pub mod task;
pub mod service;
pub mod store;
pub mod tls;
