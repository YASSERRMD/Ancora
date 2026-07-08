use crate::compliance_review::ComplianceReviewer;

#[test]
fn compliance_review_runs_air_gapped() {
    // All evaluation is local; no network call is issued.
    let reviewer = ComplianceReviewer::government_preset();

    let compliant_text =
        "CLASSIFICATION: UNCLASSIFIED\nAUTHORITY: Office of Management and Budget\nRETENTION: 10 years";

    let result = reviewer.review("artifact-001", compliant_text);
    assert!(result.passed, "compliant artifact should pass");
    assert_eq!(result.critical_count(), 0);
    assert_eq!(result.high_count(), 0);
}

#[test]
fn missing_classification_causes_critical_finding() {
    let reviewer = ComplianceReviewer::government_preset();
    let text = "AUTHORITY: DoD\nRETENTION: 7 years";
    let result = reviewer.review("artifact-002", text);
    assert!(!result.passed);
    assert!(
        result.findings.iter().any(|f| f.rule_id == "GOV-001"),
        "GOV-001 should fire when CLASSIFICATION is absent"
    );
}

#[test]
fn plaintext_secret_causes_critical_finding() {
    let reviewer = ComplianceReviewer::government_preset();
    let text = "CLASSIFICATION: UNCLASSIFIED\nAUTHORITY: DoD\nRETENTION: 7 years\nSECRET=abc123";
    let result = reviewer.review("artifact-003", text);
    assert!(!result.passed);
    assert!(
        result.findings.iter().any(|f| f.rule_id == "GOV-002"),
        "GOV-002 should fire when SECRET= appears"
    );
}

#[test]
fn severity_distribution_is_correct() {
    let reviewer = ComplianceReviewer::government_preset();
    // Provide nothing - all rules should fire.
    let result = reviewer.review("artifact-004", "no relevant keywords here");
    assert!(result.critical_count() >= 1);
    assert!(result.high_count() >= 1);
}
