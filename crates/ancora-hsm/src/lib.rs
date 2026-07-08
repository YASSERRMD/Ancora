pub mod audit;
pub mod builder;
pub mod key;
pub mod mock;
pub mod policy;
pub mod presets;
pub mod report;
pub mod session;
pub mod slot;
pub mod stats;

#[cfg(test)]
mod tests {
    mod test_audit;
    mod test_audit_by_operation;
    mod test_audit_failures;
    mod test_builder;
    mod test_builder_extractable;
    mod test_hsm_operation_display;
    mod test_key;
    mod test_key_algorithm_display;
    mod test_key_class_display;
    mod test_mock;
    mod test_mock_encrypt_decrypt;
    mod test_mock_sign;
    mod test_policy;
    mod test_policy_blocked_algorithms;
    mod test_presets;
    mod test_presets_policy;
    mod test_presets_slot;
    mod test_report;
    mod test_report_ops;
    mod test_session;
    mod test_session_manager;
    mod test_session_state_display;
    mod test_slot;
    mod test_slot_manager;
    mod test_slot_state_display;
    mod test_stats;
    mod test_stats_sensitive_ratio;
}
