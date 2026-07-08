use crate::classifier::SafetyClassifier;
use crate::toxicity::{ToxicityDetector, ToxicityLevel};

#[test]
fn mild_insult_is_flagged_mild() {
    let d = ToxicityDetector::new();
    let r = d.analyze("Don't be so stupid about this.");
    assert_eq!(r.level, ToxicityLevel::Mild);
    assert!(r.is_toxic());
}

#[test]
fn severe_threat_is_flagged_severe() {
    let d = ToxicityDetector::new();
    let r = d.analyze("I will kill the connection if it times out.");
    assert_eq!(r.level, ToxicityLevel::Severe);
}

#[test]
fn hate_speech_detected() {
    let d = ToxicityDetector::new();
    let r = d.analyze("This racist content should not appear here.");
    assert!(r.is_toxic());
    assert!(!r.matched_terms.is_empty());
}

#[test]
fn classifier_reports_toxic() {
    let clf = SafetyClassifier::new();
    let report = clf.classify("You idiot, why did you do that?");
    assert!(report.toxic);
}

#[test]
fn clean_output_not_flagged() {
    let d = ToxicityDetector::new();
    let r = d.analyze("Here is a summary of the quarterly report.");
    assert!(!r.is_toxic());
    assert_eq!(r.level, ToxicityLevel::None);
}
