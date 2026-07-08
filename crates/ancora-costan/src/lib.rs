pub mod anomaly;
pub mod api;
pub mod by_capability;
pub mod by_model;
pub mod by_provider;
pub mod by_tenant;
pub mod by_tool;
pub mod cache_savings;
pub mod dashboard;
pub mod forecast;
pub mod suggestions;
/// ancora-costan: Cost analytics for the Ancora agent framework.
///
/// Breaks down spend by every dimension: model, provider, tool, tenant/project,
/// and capability. Provides anomaly detection, forecasting, and optimization
/// suggestions, along with a JSON-serializable dashboard.
pub mod timeseries;

#[cfg(test)]
mod tests;
