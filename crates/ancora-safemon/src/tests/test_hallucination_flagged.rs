use crate::hallucination::{HallucinationDetector, HallucinationSignal};
use crate::classifier::SafetyClassifier;

#[test]
fn overconfident_fact_is_flagged() {
    let d = HallucinationDetector::new();
    let flags = d.analyze("It is definitely true that water boils at 100 C.");
    assert!(!flags.is_empty());
    assert!(flags.iter().any(|f| f.signal == HallucinationSignal::OverconfidentFact));
}

#[test]
fn unverifiable_claim_is_flagged() {
    let d = HallucinationDetector::new();
    let flags = d.analyze("According to unnamed sources, the merger was cancelled.");
    assert!(!flags.is_empty());
    assert!(flags.iter().any(|f| f.signal == HallucinationSignal::UnverifiableClaim));
}

#[test]
fn clean_factual_statement_not_flagged() {
    let d = HallucinationDetector::new();
    let flags = d.analyze("The capital of Germany is Berlin.");
    assert!(flags.is_empty());
}

#[test]
fn classifier_reports_hallucination() {
    let clf = SafetyClassifier::new();
    let report = clf.classify("Studies show that this is 100% guaranteed.");
    assert!(report.hallucination_suspected);
}

#[test]
fn multiple_hallucination_signals_detected() {
    let d = HallucinationDetector::new();
    let text = "Experts say it is research proves the case.";
    let flags = d.analyze(text);
    assert!(flags.len() >= 1);
}
