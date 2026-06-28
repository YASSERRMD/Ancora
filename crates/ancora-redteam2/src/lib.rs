pub mod scenario;
pub mod attack;
pub mod objective;
pub mod detection;
pub mod store;
pub mod audit;
pub mod stats;
pub mod report;
pub mod builder;
pub mod presets;

#[cfg(test)]
mod tests {
    mod test_scenario_kind_display;
    mod test_scenario_status_display;
    mod test_scenario_new;
    mod test_scenario_lifecycle;
    mod test_scenario_duration;
    mod test_scenario_metadata;
    mod test_attack_vector_display;
    mod test_attack_outcome_display;
    mod test_attack_step_new;
    mod test_attack_step_flags;
    mod test_attack_log_basic;
    mod test_attack_log_filter;
    mod test_objective_status_display;
    mod test_objective_new;
    mod test_objective_lifecycle;
    mod test_objective_tracker;
    mod test_detection_source_display;
    mod test_detection_event_new;
    mod test_detection_log_basic;
    mod test_detection_log_rates;
    mod test_store_basic;
    mod test_store_filter;
    mod test_audit_action_display;
    mod test_audit_log;
    mod test_stats;
    mod test_report;
    mod test_builder;
    mod test_presets;
}
