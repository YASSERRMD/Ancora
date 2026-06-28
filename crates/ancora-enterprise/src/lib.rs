pub mod license;
pub mod feature;
pub mod posture;
pub mod incident;
pub mod checkpoint;
pub mod audit;
pub mod stats;
pub mod report;
pub mod builder;
pub mod presets;

#[cfg(test)]
mod tests {
    mod test_license_tier_display;
    mod test_enterprise_cap_display;
    mod test_license_new;
    mod test_license_caps;
    mod test_license_expiry;
    mod test_feature_state_display;
    mod test_feature_flag;
    mod test_feature_registry;
    mod test_posture_level_display;
    mod test_domain_score;
    mod test_security_posture;
    mod test_posture_level_mapping;
    mod test_incident_severity_display;
    mod test_incident_status_display;
    mod test_incident_new;
    mod test_incident_lifecycle;
    mod test_incident_log;
    mod test_check_status_display;
    mod test_health_check;
    mod test_checkpoint;
    mod test_audit_action_display;
    mod test_audit_log;
    mod test_stats;
    mod test_report;
    mod test_builder;
    mod test_presets;
}
