use crate::factcheck::FactChecker;

#[test]
fn fact_check_grounds_a_claim() {
    let fc = FactChecker::check("the sky is blue", |_| Some("encyclopedia.org".into()));
    assert!(fc.grounded);
    assert_eq!(fc.source, "encyclopedia.org");
}

#[test]
fn fact_check_ungrounded_when_no_source() {
    let fc = FactChecker::check("unknown claim", |_| None);
    assert!(!fc.grounded);
    assert!(fc.source.is_empty());
}

#[test]
fn fact_check_claim_preserved() {
    let fc = FactChecker::check("specific claim", |_| Some("src".into()));
    assert_eq!(fc.claim, "specific claim");
}

#[test]
fn fact_check_tool_receives_claim() {
    let mut received = String::new();
    FactChecker::check("claim-x", |c| {
        received = c.to_string();
        Some("src".into())
    });
    assert_eq!(received, "claim-x");
}
