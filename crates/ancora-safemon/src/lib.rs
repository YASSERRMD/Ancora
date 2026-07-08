pub mod alerting;
pub mod classifier;
pub mod dashboard;
pub mod hallucination;
pub mod incident_log;
pub mod local_classifier;
pub mod pii;
pub mod policy_violation;
pub mod toxicity;

#[cfg(test)]
mod tests {
    mod test_alert_fires;
    mod test_dashboard_json;
    mod test_hallucination_flagged;
    mod test_incident_logged;
    mod test_local_classifier;
    mod test_pii_flagged;
    mod test_policy_flagged;
    mod test_respects_redaction;
    mod test_toxic_flagged;
}
