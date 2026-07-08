pub mod audit;
pub mod boundary;
pub mod builder;
pub mod media;
pub mod policy;
pub mod presets;
pub mod procedure;
pub mod report;
pub mod stats;
pub mod store;
pub mod transfer;

#[cfg(test)]
mod tests {
    mod test_airgap_action_display;
    mod test_audit;
    mod test_audit_by_action;
    mod test_boundary;
    mod test_boundary_zones;
    mod test_builder;
    mod test_media;
    mod test_media_control;
    mod test_media_display;
    mod test_policy;
    mod test_policy_verdict;
    mod test_presets;
    mod test_presets_procedure;
    mod test_presets_strict;
    mod test_procedure;
    mod test_procedure_progress;
    mod test_procedure_step_status_display;
    mod test_report;
    mod test_report_counts;
    mod test_stats;
    mod test_stats_rejection_rate;
    mod test_store;
    mod test_store_pending;
    mod test_transfer;
    mod test_transfer_direction_display;
    mod test_transfer_lifecycle;
    mod test_transfer_status_display;
    mod test_zone_classification_display;
}
