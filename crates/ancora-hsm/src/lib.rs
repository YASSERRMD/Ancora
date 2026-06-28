pub mod slot;
pub mod key;
pub mod session;
pub mod mock;
pub mod audit;
pub mod stats;
pub mod policy;
pub mod report;
pub mod builder;
pub mod presets;

#[cfg(test)]
mod tests {
    mod test_slot;
    mod test_key;
    mod test_session;
    mod test_mock;
    mod test_audit;
    mod test_stats;
    mod test_policy;
    mod test_report;
    mod test_builder;
    mod test_presets;
    mod test_slot_state_display;
    mod test_key_algorithm_display;
    mod test_key_class_display;
    mod test_session_state_display;
    mod test_hsm_operation_display;
    mod test_slot_manager;
    mod test_session_manager;
    mod test_mock_sign;
    mod test_mock_encrypt_decrypt;
    mod test_audit_failures;
    mod test_audit_by_operation;
    mod test_stats_sensitive_ratio;
    mod test_policy_blocked_algorithms;
    mod test_builder_extractable;
    mod test_presets_slot;
    mod test_presets_policy;
    mod test_report_ops;
}
