/// ancora-conteval - Continuous evaluation pipeline for the Ancora agent framework.
///
/// Tracks quality metrics over time per model and provider, with rolling
/// windows, trend detection, automatic dataset refresh, and alerting.

pub mod alerting;
pub mod dashboard;
pub mod model_tracking;
pub mod prod_samples;
pub mod provider_tracking;
pub mod refresh;
pub mod rolling_metric;
pub mod scheduler;
pub mod trend;

#[cfg(test)]
mod tests {
    mod test_dashboard_json;
    mod test_dataset_refresh;
    mod test_per_model_trend;
    mod test_production_samples;
    mod test_quality_alert;
    mod test_respects_redaction;
    mod test_rolling_metric;
    mod test_scheduled_runs;
    mod test_trend_detection;
}
