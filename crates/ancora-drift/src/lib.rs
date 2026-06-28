//! ancora-drift - Production quality and drift monitoring with alerting and
//! feedback into evals.
//!
//! # Modules
//!
//! - [`sampling`] - Online quality sampling; captures a fraction of live traces.
//! - [`reference`] - Reference distribution capture from baseline traces.
//! - [`input_drift`] - Detection of drift in request-input distributions.
//! - [`output_drift`] - Detection of drift in model-output distributions.
//! - [`tool_drift`] - Detection of changes in tool-call frequencies.
//! - [`cost_drift`] - Detection of per-request cost drift.
//! - [`provider_change`] - Detection of provider-level behavioral changes.
//! - [`alerting`] - Structured alerts aggregated from all drift signals.
//! - [`dashboard`] - JSON dashboard snapshot generation.

pub mod alerting;
pub mod cost_drift;
pub mod dashboard;
pub mod input_drift;
pub mod output_drift;
pub mod provider_change;
pub mod reference;
pub mod sampling;
pub mod tool_drift;

#[cfg(test)]
mod tests {
    mod test_input_drift;
    mod test_output_drift;
    mod test_tool_drift;
    mod test_provider_change;
    mod test_alert_fires;
    mod test_no_false_alarm;
    mod test_sampled_traces;
    mod test_dashboard_json;
}
