pub mod media;
pub mod transfer;
pub mod policy;
pub mod boundary;
pub mod procedure;
pub mod audit;
pub mod store;
pub mod stats;
pub mod report;
pub mod builder;
pub mod presets;

#[cfg(test)]
mod tests {
    mod test_media;
    mod test_transfer;
    mod test_policy;
    mod test_boundary;
    mod test_procedure;
    mod test_audit;
    mod test_store;
    mod test_stats;
    mod test_report;
    mod test_builder;
    mod test_presets;
    mod test_media_display;
    mod test_transfer_direction_display;
    mod test_transfer_status_display;
    mod test_zone_classification_display;
    mod test_airgap_action_display;
    mod test_procedure_step_status_display;
    mod test_media_control;
    mod test_transfer_lifecycle;
    mod test_policy_verdict;
    mod test_boundary_zones;
    mod test_procedure_progress;
    mod test_audit_by_action;
    mod test_store_pending;
    mod test_stats_rejection_rate;
    mod test_presets_strict;
    mod test_presets_procedure;
    mod test_report_counts;
}
