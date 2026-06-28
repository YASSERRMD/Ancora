/// ancora-costan: Cost analytics for the Ancora agent framework.
///
/// Breaks down spend by every dimension: model, provider, tool, tenant/project,
/// and capability. Provides anomaly detection, forecasting, and optimization
/// suggestions, along with a JSON-serializable dashboard.

pub mod timeseries;
pub mod by_model;
pub mod by_provider;
pub mod by_tool;
pub mod by_tenant;
pub mod by_capability;
pub mod cache_savings;
pub mod anomaly;
pub mod forecast;
pub mod suggestions;
pub mod dashboard;
pub mod api;

#[cfg(test)]
mod tests;
