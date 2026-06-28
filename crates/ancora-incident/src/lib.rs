pub mod incident;
pub mod runbook;
pub mod escalation;
pub mod timeline;
pub mod store;
pub mod postmortem;
pub mod audit;
pub mod stats;
pub mod builder;
pub mod presets;
pub mod query;
pub mod report;
pub mod summary;

#[cfg(test)]
mod tests {
    mod test_incident;
    mod test_runbook;
    mod test_escalation;
    mod test_timeline;
    mod test_store;
    mod test_postmortem;
    mod test_audit;
    mod test_stats;
    mod test_builder;
    mod test_presets;
    mod test_query;
    mod test_report;
    mod test_summary;
    mod test_incident_status_display;
    mod test_severity_ordering;
    mod test_step_status_display;
    mod test_escalation_channel_display;
    mod test_timeline_event_kind_display;
    mod test_incident_action_display;
    mod test_runbook_progress;
    mod test_runbook_is_complete;
    mod test_postmortem_completion_rate;
    mod test_stats_mean_duration;
    mod test_summary_healthy;
    mod test_report_progress;
    mod test_query_active_only;
    mod test_store_by_severity;
    mod test_store_by_status;
    mod test_audit_for_incident;
    mod test_audit_for_tenant;
}
