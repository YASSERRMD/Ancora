pub mod audit;
pub mod builder;
pub mod escalation;
pub mod incident;
pub mod postmortem;
pub mod presets;
pub mod query;
pub mod report;
pub mod runbook;
pub mod stats;
pub mod store;
pub mod summary;
pub mod timeline;

#[cfg(test)]
mod tests {
    mod test_audit;
    mod test_audit_for_incident;
    mod test_audit_for_tenant;
    mod test_builder;
    mod test_escalation;
    mod test_escalation_channel_display;
    mod test_incident;
    mod test_incident_action_display;
    mod test_incident_status_display;
    mod test_postmortem;
    mod test_postmortem_completion_rate;
    mod test_presets;
    mod test_query;
    mod test_query_active_only;
    mod test_report;
    mod test_report_progress;
    mod test_runbook;
    mod test_runbook_is_complete;
    mod test_runbook_progress;
    mod test_severity_ordering;
    mod test_stats;
    mod test_stats_mean_duration;
    mod test_step_status_display;
    mod test_store;
    mod test_store_by_severity;
    mod test_store_by_status;
    mod test_summary;
    mod test_summary_healthy;
    mod test_timeline;
    mod test_timeline_event_kind_display;
}
